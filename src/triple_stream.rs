use grammar_structs::*;
use grammar::*;
use grammar_helper::*;
use std::collections::HashMap;
use nom::IResult;
use graph;
use std::rc::Rc;

struct StatementIterator<'a> {
    src: &'a str,
    done: bool,
}

impl<'a> StatementIterator<'a> {
    pub fn new(src: &str) -> Result<StatementIterator, String> {
        match tws(src) {
            IResult::Done(left, _) => {
                Ok(StatementIterator {
                    src: left,
                    done: false,
                })
            }
            IResult::Error(_) => return Err(String::from("cannot start parsing")),
            IResult::Incomplete(_) => {
                return Ok(StatementIterator {
                    src: src,
                    done: false,
                })
            }
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<Statement<'a>, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match statement(self.src) {
            IResult::Done(left, s) => {
                r = Some(Ok(s));
                self.src = left;
            }
            IResult::Error(_) => {
                r = Some(Err(String::from("error parsing")));
                self.done = true;
            }
            IResult::Incomplete(_) => {
                self.done = true;
                r = None;
            }
        }
        match tws(self.src) {
            IResult::Done(left, _) => {
                self.src = left;
            }
            IResult::Error(_) => {
                r = Some(Err(String::from("error parsing")));
                self.done = true;
            }
            IResult::Incomplete(_) => {
                self.done = true;
            }
        }
        r
    }
}

struct BlankNodes<'a> {
    blank_nodes: HashMap<&'a str, usize>,
    next_blank: usize,
}

#[derive (Clone)]
pub struct IteratorTriple {
    pub subject: IteratorSubject,
    pub predicate: Rc<String>,
    pub object: IteratorObject,
}

#[derive (Clone)]
pub enum IteratorSubject {
    IRI(Rc<String>),
    BlankNode(graph::BlankNode),
}

#[derive (Clone)]
pub struct IteratorLiteral {
    pub lexical: Rc<String>,
    pub datatype: Rc<String>,
    pub language: Option<Rc<String>>,
}

#[derive (Clone)]
pub enum IteratorObject {
    IRI(Rc<String>),
    BlankNode(graph::BlankNode),
    Literal(IteratorLiteral),
}

impl graph::Triple for IteratorTriple {
    fn subject(&self) -> graph::Subject {
        match self.subject {
            IteratorSubject::IRI(ref iri) => graph::Subject::IRI(iri.as_str()),
            IteratorSubject::BlankNode(n) => graph::Subject::BlankNode(n),
        }
    }
    fn predicate(&self) -> &str {
        self.predicate.as_str()
    }
    fn object(&self) -> graph::Object {
        match self.object {
            IteratorObject::IRI(ref iri) => graph::Object::IRI(iri.as_str()),
            IteratorObject::BlankNode(n) => graph::Object::BlankNode(n),
            IteratorObject::Literal(ref l) => {
                graph::Object::Literal(graph::Literal {
                    lexical: l.lexical.as_str(),
                    datatype: l.datatype.as_str(),
                    language: l.language.as_ref().map(|l| l.as_str()),
                })
            }
        }
    }
}

struct Strings {
    subject: Rc<String>,
    predicate: Rc<String>,
    object: Rc<String>,
    datatype: Rc<String>,
    language: Rc<String>,
}

pub struct TripleIterator<'a> {
    statement_iterator: StatementIterator<'a>,
    base: String,
    prefixes: HashMap<&'a str, String>,
    blank_nodes: BlankNodes<'a>,
    triple_buffer: Vec<Triple<'a>>,
    done: bool,
    strings: Strings,
}

impl<'a> TripleIterator<'a> {
    pub fn new(src: &str) -> Result<TripleIterator, String> {
        let rc = Rc::new(String::new());
        Ok(TripleIterator {
            statement_iterator: try!(StatementIterator::new(src)),
            base: String::new(),
            prefixes: HashMap::new(),
            blank_nodes: BlankNodes {
                blank_nodes: HashMap::new(),
                next_blank: 0,
            },
            triple_buffer: Vec::new(),
            done: false,
            strings: Strings {
                subject: rc.clone(),
                predicate: rc.clone(),
                object: rc.clone(),
                datatype: rc.clone(),
                language: rc.clone(),
            },
        })
    }
    pub fn prefixes(&self) -> &HashMap<&'a str, String> {
        &self.prefixes
    }
    fn set_prefix(&mut self, prefix: &'a str, value: String) {
        let value = resolve_iri_ref(value);
        self.prefixes.insert(prefix, value);
    }
    fn fill_buffer(&mut self) -> Result<usize, String> {
        while let Some(statement) = self.statement_iterator.next() {
            match statement {
                Ok(Statement::Prefix(prefix, iri)) => {
                    let mut result = String::with_capacity(iri.len());
                    try!(unescape(iri, &mut result));
                    self.set_prefix(prefix, result);
                }
                Ok(Statement::Base(new_base)) => {
                    try!(unescape(new_base, &mut self.base));
                }
                Ok(Statement::Triples(new_triples)) => {
                    try!(add_triples(new_triples, &mut self.blank_nodes, &mut self.triple_buffer));
                    return Ok(self.triple_buffer.len());
                }
                Err(e) => return Err(e),
            }
        }
        Ok(0)
    }
}

fn resolve_triple(triple: Triple,
                  prefixes: &HashMap<&str, String>,
                  strings: &mut Strings)
                  -> Result<IteratorTriple, String> {
    Ok(IteratorTriple {
        subject: match triple.subject {
            SingleSubject::IRI(iri) => {
                try!(resolve_iri(iri, prefixes, &mut strings.subject));
                IteratorSubject::IRI(strings.subject.clone())
            }
            SingleSubject::BlankNode(n) => IteratorSubject::BlankNode((n, 0)),
        },
        predicate: {
            try!(resolve_iri(triple.predicate, prefixes, &mut strings.predicate));
            strings.predicate.clone()
        },
        object: match triple.object {
            SingleObject::IRI(iri) => {
                try!(resolve_iri(iri, prefixes, &mut strings.object));
                IteratorObject::IRI(strings.object.clone())
            }
            SingleObject::BlankNode(n) => IteratorObject::BlankNode((n, 0)),
            SingleObject::Literal(l) => {
                IteratorObject::Literal(IteratorLiteral {
                    lexical: {
                        try!(unescape_literal(l.lexical, &mut strings.object));
                        strings.object.clone()
                    },
                    datatype: {
                        try!(resolve_iri(l.datatype, prefixes, &mut strings.datatype));
                        strings.datatype.clone()
                    },
                    language: match l.language {
                        Some(l) => {
                            {
                                let s = Rc::make_mut(&mut strings.language);
                                s.clear();
                                for c in l.chars() {
                                    s.extend(c.to_lowercase());
                                }
                            }
                            Some(strings.language.clone())
                        }
                        None => None,
                    },
                })
            }
        },
    })
}
fn unescape_literal(string: &str, to: &mut Rc<String>) -> Result<(), String> {
    let p = Rc::make_mut(to);
    p.clear();
    try!(unescape(string, p));
    Ok(())
}
fn resolve_iri(iri: IRI,
               prefixes: &HashMap<&str, String>,
               to: &mut Rc<String>)
               -> Result<(), String> {
    let p = Rc::make_mut(to);
    p.clear();
    match iri {
        IRI::IRI(iri) => {
            try!(unescape(iri, p));
        }
        IRI::PrefixedName(ns, local) => {
            match prefixes.get(ns) {
                Some(prefix) => {
                    p.push_str(prefix);
                    try!(pn_local_unescape(local, p));
                }
                None => return Err(String::from("Cannot find prefix.")),
            }
        }
    }
    Ok(())
}

fn resolve_iri_ref(iri: String) -> String {
    iri
}

impl<'a> BlankNodes<'a> {
    fn new_blank(&mut self) -> usize {
        let b = self.next_blank;
        self.next_blank += 1;
        b
    }
    fn get_blank(&mut self, label: &'a str) -> usize {
        if let Some(n) = self.blank_nodes.get(label) {
            return *n;
        }
        let n = self.new_blank();
        self.blank_nodes.insert(label, n);
        n
    }
}

impl<'a> Iterator for TripleIterator<'a> {
    type Item = Result<IteratorTriple, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.triple_buffer.len() == 0 {
            match self.fill_buffer() {
                Ok(0) => {
                    self.done = true;
                    return None;
                }
                Ok(_) => {}
                Err(e) => return Some(Err(e)),
            }
        }
        match self.triple_buffer.pop() {
            Some(t) => Some(resolve_triple(t, &self.prefixes, &mut self.strings)),
            None => None,
        }
    }
}

fn make_blank<'a>(blank_node: BlankNode<'a>, blank_nodes: &mut BlankNodes<'a>) -> usize {
    match blank_node {
        BlankNode::Anon => blank_nodes.new_blank(),
        BlankNode::BlankNode(label) => blank_nodes.get_blank(label),
    }
}

const RDF_FIRST: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
const RDF_REST: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
const RDF_NIL: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";

fn make_collection<'a>(collection: Vec<Object<'a>>,
                       blank_nodes: &mut BlankNodes<'a>,
                       triple_buffer: &mut Vec<Triple<'a>>)
                       -> Result<usize, String> {
    let head = blank_nodes.new_blank();

    let mut node = head;
    for object in collection {
        let o = try!(make_object(object, blank_nodes, triple_buffer));
        triple_buffer.push(Triple {
            subject: SingleSubject::BlankNode(node),
            predicate: IRI::IRI(RDF_FIRST),
            object: o,
        });
        let next = blank_nodes.new_blank();
        triple_buffer.push(Triple {
            subject: SingleSubject::BlankNode(node),
            predicate: IRI::IRI(RDF_REST),
            object: SingleObject::BlankNode(next),
        });
        node = next;
    }
    triple_buffer.push(Triple {
        subject: SingleSubject::BlankNode(node),
        predicate: IRI::IRI(RDF_REST),
        object: SingleObject::IRI(IRI::IRI(RDF_NIL)),
    });
    Ok(head)
}

fn make_subject<'a>(subject: Subject<'a>,
                    blank_nodes: &mut BlankNodes<'a>,
                    triple_buffer: &mut Vec<Triple<'a>>)
                    -> Result<SingleSubject<'a>, String> {
    Ok(match subject {
        Subject::IRI(iri) => SingleSubject::IRI(iri),
        Subject::BlankNode(blank) => SingleSubject::BlankNode(make_blank(blank, blank_nodes)),
        Subject::Collection(collection) => {
            SingleSubject::BlankNode(try!(make_collection(collection, blank_nodes, triple_buffer)))
        }
    })
}

fn make_object<'a>(object: Object<'a>,
                   blank_nodes: &mut BlankNodes<'a>,
                   triple_buffer: &mut Vec<Triple<'a>>)
                   -> Result<SingleObject<'a>, String> {
    Ok(match object {
        Object::IRI(iri) => SingleObject::IRI(iri),
        Object::BlankNode(blank) => SingleObject::BlankNode(make_blank(blank, blank_nodes)),
        Object::Collection(collection) => {
            SingleObject::BlankNode(try!(make_collection(collection, blank_nodes, triple_buffer)))
        }
        Object::Literal(l) => SingleObject::Literal(l),
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = blank_nodes.new_blank();
            let subject = SingleSubject::BlankNode(blank);
            try!(add_predicated_objects(subject,
                                        predicated_objects_list,
                                        blank_nodes,
                                        triple_buffer));
            SingleObject::BlankNode(blank)
        }
    })
}

fn add_predicated_objects<'a>(subject: SingleSubject<'a>,
                              predicated_objects_list: Vec<PredicatedObjects<'a>>,
                              blank_nodes: &mut BlankNodes<'a>,
                              triple_buffer: &mut Vec<Triple<'a>>)
                              -> Result<(), String> {
    for po in predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = Triple {
                subject: subject.clone(),
                predicate: po.verb.clone(),
                object: try!(make_object(o, blank_nodes, triple_buffer)),
            };
            triple_buffer.push(triple);
        }
    }
    Ok(())
}

fn add_triples<'a>(new_triples: Triples<'a>,
                   blank_nodes: &mut BlankNodes<'a>,
                   triple_buffer: &mut Vec<Triple<'a>>)
                   -> Result<(), String> {
    let subject = try!(make_subject(new_triples.subject, blank_nodes, triple_buffer));
    add_predicated_objects(subject,
                           new_triples.predicated_objects_list,
                           blank_nodes,
                           triple_buffer)
}
