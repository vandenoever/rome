use super::grammar::{statement, tws};
use super::grammar_helper::*;
use super::grammar_structs::*;
use constants;
use error::{Error, Result};
use graph;
use namespaces::*;
use nom::types::CompleteStr;
use nom::Err;
use ontology::iri::{rdf, xsd};
use regex::Regex;
use std::collections::HashMap;
use std::marker::PhantomData;

struct StatementIterator<'a> {
    src: CompleteStr<'a>,
    done: bool,
}

impl<'a> StatementIterator<'a> {
    pub fn new(src: &str) -> Result<StatementIterator> {
        match tws(CompleteStr(src)) {
            Ok((left, _)) => Ok(StatementIterator {
                src: left,
                done: false,
            }),
            Err(Err::Incomplete(_)) => Ok(StatementIterator {
                src: CompleteStr(src),
                done: false,
            }),
            Err(_) => Err(Error::Custom("cannot start parsing")),
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<Statement<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match statement(self.src) {
            Ok((left, s)) => {
                r = Some(Ok(s));
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                self.done = true;
                r = None;
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                r = Some(Err(Error::from(e)));
                self.done = true;
            }
        }
        match tws(self.src) {
            Ok((left, _)) => {
                self.src = left;
            }
            Err(Err::Incomplete(_)) => {
                self.done = true;
            }
            Err(_) => {
                r = Some(Err(Error::Custom("error parsing whitespace")));
                self.done = true;
            }
        }
        if r.is_none() && !self.src.is_empty() {
            r = Some(Err(Error::Custom("trailing bytes")));
        }
        r
    }
}

struct ParserState<'a, W: 'a>
where
    W: graph::GraphWriter<'a>,
{
    base: String,
    prefixes: Namespaces,
    blank_nodes: HashMap<&'a str, W::BlankNode>,
    writer: &'a mut W,
    buffer: String,
    iri: String,
    literal: String,
    rdf_lang_string: Option<W::Datatype>,
    xsd_boolean: Option<W::Datatype>,
    xsd_decimal: Option<W::Datatype>,
    xsd_double: Option<W::Datatype>,
    xsd_integer: Option<W::Datatype>,
    xsd_string: Option<W::Datatype>,
    rdf_first: Option<W::IRI>,
    rdf_rest: Option<W::IRI>,
    rdf_nil: Option<W::IRI>,
}

pub struct TurtleParser<'a, W: 'a>
where
    W: graph::GraphWriter<'a>,
{
    statement_iterator: StatementIterator<'a>,
    state: ParserState<'a, W>,
    done: bool,
}

fn is_absolute(url: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^[a-z][a-z0-9+.-]*:").unwrap();
    }
    RE.is_match(url)
}

fn join_iri(base: &str, p: &str, to: &mut String) -> Result<()> {
    to.clear();
    if !is_absolute(p) {
        let mut end = base.len();
        if !p.starts_with('#') {
            if let Some(pos) = base.rfind('/') {
                end = pos + 1;
            }
        }
        to.push_str(&base[..end]);
    }
    to.push_str(p);
    Ok(())
}

impl<'a, W: 'a> TurtleParser<'a, W>
where
    W: graph::GraphWriter<'a>,
{
    pub fn new(src: &'a str, base: &str, writer: &'a mut W) -> Result<TurtleParser<'a, W>> {
        if !is_absolute(base) {
            return Err(Error::Custom("base url is not absolute"));
        }
        Ok(TurtleParser {
            statement_iterator: StatementIterator::new(src)?,
            state: ParserState {
                base: String::from(base),
                prefixes: Namespaces::new(),
                blank_nodes: HashMap::new(),
                writer,
                buffer: String::new(),
                iri: String::new(),
                literal: String::new(),
                rdf_lang_string: None,
                xsd_boolean: None,
                xsd_decimal: None,
                xsd_double: None,
                xsd_integer: None,
                xsd_string: None,
                rdf_first: None,
                rdf_rest: None,
                rdf_nil: None,
            },
            done: false,
        })
    }
    pub fn prefixes(&self) -> &Namespaces {
        &self.state.prefixes
    }
    fn set_prefix(&mut self, prefix: &'a str, value: String) {
        self.state.prefixes.insert(prefix, value);
    }
    /// return Ok(true) when done
    fn parse_statement(&mut self) -> Result<bool> {
        if let Some(statement) = self.statement_iterator.next() {
            match statement {
                Ok(Statement::Prefix(prefix, iri)) => {
                    let mut result = String::with_capacity(iri.len());
                    self.state.buffer.clear();
                    unescape_iri(iri, &mut self.state.buffer)?;
                    join_iri(
                        self.state.base.as_str(),
                        self.state.buffer.as_str(),
                        &mut result,
                    )?;
                    self.set_prefix(prefix, result);
                }
                Ok(Statement::Base(new_base)) => {
                    self.state.buffer.clear();
                    unescape_iri(new_base, &mut self.state.buffer)?;
                    let old_base = self.state.base.clone();
                    join_iri(
                        old_base.as_str(),
                        self.state.buffer.as_str(),
                        &mut self.state.base,
                    )?;
                }
                Ok(Statement::Triples(new_triples)) => {
                    add_triples(new_triples, &mut self.state)?;
                }
                Err(e) => return Err(e),
            }
            Ok(false)
        } else {
            Ok(true)
        }
    }
}

impl<'a, W: 'a> Iterator for TurtleParser<'a, W>
where
    W: graph::GraphWriter<'a>,
{
    type Item = Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        match self.parse_statement() {
            Ok(true) => {
                self.done = true;
                None
            }
            Ok(_) => Some(Ok(())),
            Err(e) => Some(Err(e)),
        }
    }
}

fn unescape_literal(string: &str, to: &mut String) -> Result<()> {
    to.clear();
    unescape(string, to)?;
    Ok(())
}

impl<'a, W> ParserState<'a, W>
where
    W: graph::GraphWriter<'a>,
{
    fn new_blank(&mut self) -> W::BlankNode {
        self.writer.create_blank_node()
    }
    fn get_blank(&mut self, label: &'a str) -> W::BlankNode {
        if let Some(n) = self.blank_nodes.get(label) {
            return n.clone();
        }
        let n = self.new_blank();
        self.blank_nodes.insert(label, n.clone());
        n
    }
    fn resolve_iri(&mut self, iri: IRI) -> Result<()> {
        self.iri.clear();
        match iri {
            IRI::IRI(iri) => {
                self.buffer.clear();
                unescape_iri(iri, &mut self.buffer)?;
                join_iri(&self.base, self.buffer.as_str(), &mut self.iri)?;
            }
            IRI::PrefixedName(prefix, local) => match self.prefixes.find_namespace(&prefix) {
                Some(ns) => {
                    self.iri.push_str(ns);
                    pn_local_unescape(local, &mut self.iri)?;
                }
                None => return Err(Error::Custom("Cannot find prefix.")),
            },
        }
        Ok(())
    }
    fn get_datatype(&mut self, datatype: &Datatype) -> Result<W::Datatype> {
        Ok(match datatype {
            Datatype::IRI(iri) => {
                self.resolve_iri(*iri)?;
                self.writer.create_datatype(&self.iri)
            }
            Datatype::RDFLangString => {
                get_cached_datatype(&mut self.rdf_lang_string, self.writer, rdf::LANG_STRING)
            }
            Datatype::XSDBoolean => {
                get_cached_datatype(&mut self.xsd_boolean, self.writer, xsd::BOOLEAN)
            }
            Datatype::XSDDecimal => {
                get_cached_datatype(&mut self.xsd_decimal, self.writer, xsd::DECIMAL)
            }
            Datatype::XSDDouble => {
                get_cached_datatype(&mut self.xsd_double, self.writer, xsd::DOUBLE)
            }
            Datatype::XSDInteger => {
                get_cached_datatype(&mut self.xsd_integer, self.writer, xsd::INTEGER)
            }
            Datatype::XSDString => {
                get_cached_datatype(&mut self.xsd_string, self.writer, xsd::STRING)
            }
        })
    }
}

fn get_cached_datatype<'a, W: 'a>(
    cache: &mut Option<W::Datatype>,
    writer: &mut W,
    datatype: &str,
) -> W::Datatype
where
    W: graph::GraphWriter<'a>,
{
    if cache.is_none() {
        *cache = Some(writer.create_datatype(datatype));
    }
    cache.clone().unwrap()
}

fn get_cached_iri<'a, W: 'a>(cache: &mut Option<W::IRI>, writer: &mut W, iri: &str) -> W::IRI
where
    W: graph::GraphWriter<'a>,
{
    if cache.is_none() {
        *cache = Some(writer.create_iri(&iri));
    }
    cache.clone().unwrap()
}

fn make_blank<'a, W: 'a>(blank_node: &BlankNode<'a>, state: &mut ParserState<'a, W>) -> W::BlankNode
where
    W: graph::GraphWriter<'a>,
{
    match blank_node {
        BlankNode::Anon => state.new_blank(),
        BlankNode::BlankNode(label) => state.get_blank(label),
    }
}

fn s2o<'a, W>(s: graph::WriterBlankNodeOrIRI<'a, W>) -> graph::WriterResource<'a, W>
where
    W: graph::GraphWriter<'a>,
{
    match s {
        graph::WriterBlankNodeOrIRI::IRI(iri) => graph::WriterResource::IRI(iri),
        graph::WriterBlankNodeOrIRI::BlankNode(n, p) => graph::WriterResource::BlankNode(n, p),
    }
}

fn make_collection<'a, W>(
    collection: Vec<Object<'a>>,
    state: &mut ParserState<'a, W>,
) -> Result<graph::WriterBlankNodeOrIRI<'a, W>>
where
    W: graph::GraphWriter<'a>,
{
    let mut head = graph::WriterBlankNodeOrIRI::IRI(get_cached_iri(
        &mut state.rdf_nil,
        state.writer,
        constants::RDF_NIL,
    ));
    for object in collection.into_iter().rev() {
        let this = state.new_blank();
        let rdf_first = get_cached_iri(&mut state.rdf_first, state.writer, rdf::FIRST);
        let rdf_rest = get_cached_iri(&mut state.rdf_rest, state.writer, rdf::REST);
        let o = make_object(object, state)?;
        match o {
            graph::WriterResource::BlankNode(o, _) => {
                state.writer.add_blank_blank(&this, &rdf_first, &o);
            }
            graph::WriterResource::IRI(o) => {
                state.writer.add_blank_iri(&this, &rdf_first, &o);
            }
            graph::WriterResource::Literal(o) => {
                state.writer.add_blank_literal(&this, &rdf_first, &o);
            }
        }
        match head {
            graph::WriterBlankNodeOrIRI::BlankNode(head, _) => {
                state.writer.add_blank_blank(&this, &rdf_rest, &head);
            }
            graph::WriterBlankNodeOrIRI::IRI(head) => {
                state.writer.add_blank_iri(&this, &rdf_rest, &head);
            }
        }
        head = graph::WriterBlankNodeOrIRI::BlankNode(this, PhantomData);
    }
    Ok(head)
}

fn make_subject<'a, W>(
    subject: Subject<'a>,
    state: &mut ParserState<'a, W>,
) -> Result<graph::WriterBlankNodeOrIRI<'a, W>>
where
    W: graph::GraphWriter<'a>,
{
    Ok(match subject {
        Subject::BlankNode(blank) => {
            graph::WriterBlankNodeOrIRI::BlankNode(make_blank(&blank, state), PhantomData)
        }
        Subject::IRI(iri) => {
            state.resolve_iri(iri)?;
            graph::WriterBlankNodeOrIRI::IRI(state.writer.create_iri(&state.iri))
        }
        Subject::Collection(collection) => make_collection(collection, state)?,
    })
}

fn make_object<'a, W>(
    object: Object<'a>,
    state: &mut ParserState<'a, W>,
) -> Result<graph::WriterResource<'a, W>>
where
    W: graph::GraphWriter<'a>,
{
    Ok(match object {
        Object::IRI(iri) => {
            state.resolve_iri(iri)?;
            graph::WriterResource::IRI(state.writer.create_iri(&state.iri))
        }
        Object::BlankNode(blank) => {
            graph::WriterResource::BlankNode(make_blank(&blank, state), PhantomData)
        }
        Object::Collection(collection) => s2o(make_collection(collection, state)?),
        Object::Literal(l) => {
            unescape_literal(l.lexical, &mut state.literal)?;
            graph::WriterResource::Literal(if let Some(lang) = l.language {
                let language = state.writer.create_language(lang);
                state
                    .writer
                    .create_literal_language(&state.literal, &language)
            } else {
                let datatype = state.get_datatype(&l.datatype)?;
                state
                    .writer
                    .create_literal_datatype(&state.literal, &datatype)
            })
        }
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = state.new_blank();
            let subject = graph::WriterBlankNodeOrIRI::BlankNode(blank.clone(), PhantomData);
            add_predicated_objects(&subject, predicated_objects_list, state)?;
            graph::WriterResource::BlankNode(blank, PhantomData)
        }
    })
}

fn add_predicated_objects<'a, W>(
    subject: &graph::WriterBlankNodeOrIRI<'a, W>,
    predicated_objects_list: Vec<PredicatedObjects<'a>>,
    state: &mut ParserState<'a, W>,
) -> Result<()>
where
    W: graph::GraphWriter<'a>,
{
    for po in predicated_objects_list {
        state.resolve_iri(po.verb)?;
        let predicate = state.writer.create_iri(&state.iri);
        for o in po.objects {
            let object = make_object(o, state)?;
            state.writer.add(subject, &predicate, &object);
        }
    }
    Ok(())
}

fn add_triples<'a, W>(new_triples: Triples<'a>, state: &mut ParserState<'a, W>) -> Result<()>
where
    W: graph::GraphWriter<'a>,
{
    let subject = make_subject(new_triples.subject, state)?;
    add_predicated_objects(&subject, new_triples.predicated_objects_list, state)
}

#[test]
fn blank_node() {
    let s = "<http://a.example/s> <http://a.example/p> _:b1 .\n";
    let mut i = StatementIterator::new(s).unwrap();
    let n = i.next();
    assert!(n.is_some());
    assert!(n.unwrap().is_ok());
}

#[test]
fn test_string_literal_long_quote() {
    let s = "<http://a.example/s> <http://a.example/p> \"\"\"first long literal\"\"\" .\n";
    let mut i = StatementIterator::new(s).unwrap();
    let n = i.next();
    assert!(n.is_some());
    assert!(n.unwrap().is_ok());
}

#[test]
fn test_no_space_before_dot() {
    let s = "@prefix : <urn:> .\n:s..2 :p..2 :o.\n";
    let mut i = StatementIterator::new(s).unwrap();
    i.next();
    let n = i.next();
    assert!(n.is_some());
    assert!(n.unwrap().is_ok());
}
