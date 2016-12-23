#[macro_use]
extern crate nom;
extern crate rand;
use nom::IResult;
use nom::ErrorKind;
use nom::Needed;
use std::io;
use std::io::Read;
use std::fs::File;
use std::rc::Rc;

mod unsafe_key;
pub mod triple_stream;
mod grammar;
mod grammar_structs;
mod grammar_helper;
pub mod graph;
pub mod mem_graph;
pub mod index_graph;
pub mod ntriples_writer;
mod string_store;
mod unsafe_graph;
mod graph_writer;
mod string_collector;

use grammar::turtle;
use grammar_structs::*;
use graph::{WritableGraph, Triple};
use mem_graph::MemGraph;
use std::collections::HashMap;

fn parse_nom(data: &str) -> Result<Vec<Statement>, String> {
    match turtle(data) {
        IResult::Done("", statements) => Ok(statements),
        IResult::Done(left, _) => Err(format!("Not all data was parsed. '{}'", left)),
        IResult::Error(e) => Err(String::from(e.description())),
        IResult::Incomplete(Needed::Unknown) => Err(format!("More data is needed.")),
        IResult::Incomplete(Needed::Size(n)) => Err(format!("{} more characters are needed.", n)),
    }
}

fn resolve_iri_ref(iri: &String, base: &str) -> String {
    iri.clone()
}

struct State<'a> {
    graph: &'a mut WritableGraph,
    base: String,
    prefixes: HashMap<String, String>,
    blank_nodes: HashMap<String, graph::BlankNode>,
}

impl<'a> State<'a> {
    fn new(graph: &'a mut WritableGraph) -> State {
        State {
            graph: graph,
            base: String::new(),
            prefixes: HashMap::new(),
            blank_nodes: HashMap::new(),
        }
    }
    fn base(&self) -> &str {
        &self.base
    }
    fn prefixes(&self) -> &HashMap<String, String> {
        &self.prefixes
    }
    fn set_base(&mut self, base: String) {
        self.base = base
    }
    fn set_prefix(&mut self, prefix: String, value: String) {
        let value = resolve_iri_ref(&value, &self.base);
        self.prefixes.insert(prefix, value);
    }
    fn new_blank(&mut self) -> graph::BlankNode {
        self.graph.create_blank_node()
    }
    fn get_blank(&mut self, label: String) -> graph::BlankNode {
        if let Some(n) = self.blank_nodes.get(&label) {
            return *n;
        }
        let n = self.new_blank();
        self.blank_nodes.insert(label, n);
        n
    }
    fn add_triple(&mut self, triple: Triple) {
        self.graph.add_triple(&triple)
    }
}

fn resolve_iri(iri: &IRI, state: &State) -> Result<Rc<String>, String> {
    let i = match *iri {
        IRI::IRI(ref iri) => resolve_iri_ref(iri, state.base()),
        IRI::PrefixedName(ref prefix, ref local) => {
            let base = match state.prefixes().get(prefix) {
                Some(base) => base.clone(),
                None => return Err(format!("Prefix {} was not defined.", prefix)),
            };
            base + local
        }
    };
    Ok(Rc::new(i))
}

fn make_blank(blank_node: BlankNode, state: &mut State) -> graph::BlankNode {
    match blank_node {
        BlankNode::Anon => state.new_blank(),
        BlankNode::BlankNode(label) => state.get_blank(label),
    }
}

fn r(str: &str) -> Rc<String> {
    Rc::new(String::from(str))
}
fn rdf_first() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#first")
}
fn rdf_rest() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest")
}
fn rdf_nil() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil")
}

fn make_collection(collection: Vec<Object>, state: &mut State) -> Result<graph::BlankNode, String> {
    let head = state.new_blank();

    let mut node = head;
    for object in collection {
        let o = try!(make_object(object, state));
        state.add_triple(Triple {
            subject: graph::Subject::BlankNode(node),
            predicate: rdf_first(),
            object: o,
        });
        let next = state.new_blank();
        state.add_triple(Triple {
            subject: graph::Subject::BlankNode(node),
            predicate: rdf_rest(),
            object: graph::Object::BlankNode(next),
        });
        node = next;
    }
    state.add_triple(Triple {
        subject: graph::Subject::BlankNode(node),
        predicate: rdf_rest(),
        object: graph::Object::IRI(rdf_nil()),
    });
    Ok(head)
}

fn make_subject(subject: Subject, state: &mut State) -> Result<graph::Subject, String> {
    Ok(match subject {
        Subject::IRI(iri) => {
            let iri = try!(resolve_iri(&iri, state));
            graph::Subject::IRI(iri)
        }
        Subject::BlankNode(blank) => graph::Subject::BlankNode(make_blank(blank, state)),
        Subject::Collection(collection) => {
            graph::Subject::BlankNode(try!(make_collection(collection, state)))
        }
    })
}

fn make_object(object: Object, state: &mut State) -> Result<graph::Object, String> {
    Ok(match object {
        Object::IRI(ref iri) => graph::Object::IRI(try!(resolve_iri(&iri, state))),
        Object::BlankNode(blank) => graph::Object::BlankNode(make_blank(blank, state)),
        Object::Collection(collection) => {
            graph::Object::BlankNode(try!(make_collection(collection, state)))
        }
        Object::Literal(l) => {
            graph::Object::Literal(graph::Literal {
                lexical: Rc::new(l.lexical),
                datatype: try!(resolve_iri(&l.datatype, state)),
                extra: match l.extra {
                    LiteralExtra::None => graph::LiteralExtra::None,
                    LiteralExtra::LanguageTag(v) => graph::LiteralExtra::LanguageTag(Rc::new(v)),
                    LiteralExtra::XsdBoolean(v) => graph::LiteralExtra::XsdBoolean(v),
                    LiteralExtra::XsdDecimal(v) => graph::LiteralExtra::XsdDecimal(v),
                    LiteralExtra::XsdDouble(v) => graph::LiteralExtra::XsdDouble(v),
                    LiteralExtra::XsdInteger(v) => graph::LiteralExtra::XsdInteger(v),
                },
            })
        }
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = state.new_blank();
            let subject = graph::Subject::BlankNode(blank);
            try!(add_predicated_objects(subject, predicated_objects_list, state));
            graph::Object::BlankNode(blank)
        }
    })
}

fn add_predicated_objects(subject: graph::Subject,
                          predicated_objects_list: Vec<PredicatedObjects>,
                          state: &mut State)
                          -> Result<(), String> {
    for po in predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = Triple {
                subject: subject.clone(),
                predicate: try!(resolve_iri(&po.verb, state)),
                object: try!(make_object(o, state)),
            };
            state.add_triple(triple);
        }
    }
    Ok(())
}

fn add_triples(new_triples: Triples, state: &mut State) -> Result<(), String> {
    let subject = try!(make_subject(new_triples.subject, state));
    add_predicated_objects(subject, new_triples.predicated_objects_list, state)
}

pub fn parse(data: &str) -> Result<(MemGraph, HashMap<String, String>), String> {
    let statements = try!(parse_nom(data));
    let mut graph = MemGraph::new();
    let prefixes;
    {
        let mut state = State::new(&mut graph);
        for statement in statements {
            match statement {
                Statement::Prefix(prefix, iri) => {
                    state.set_prefix(prefix, iri);
                }
                Statement::Base(new_base) => {
                    state.set_base(new_base);
                }
                Statement::Triples(new_triples) => {
                    try!(add_triples(new_triples, &mut state));
                }
            }
        }
        prefixes = state.prefixes;
    }
    Ok((graph, prefixes))
}

pub fn run(path: &str) -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    match parse(s.as_str()) {
        Ok((graph, _)) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            try!(ntriples_writer::write_ntriples(&graph, &mut handle));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
    Ok(())
}

#[test]
fn test_run() {
    let path = "/tmp/tracker/tests/libtracker-data/update/delete-insert-where-1.ontology";
    if let Err(e) = run(&path) {
        println!("{:?}", e);
    }
}

#[test]
fn test_short() {
    let r1 = parse("@prefix:<>.").unwrap();
    // assert_eq!(r1, vec![]);
    // let r2 = parse("<><><>.").unwrap();
    // let t = ast::Triple {
    // subject: ast::Subject::IRI(Rc::new(String::new())),
    // predicate: String::new(),
    // object: ast::Object::IRI(String::new()),
    // };
    // assert_eq!(r2, vec![t.clone()]);
    // let r3 = parse("@prefix:<>.: : :.").unwrap();
    // assert_eq!(r3, vec![t]);
    //
}
