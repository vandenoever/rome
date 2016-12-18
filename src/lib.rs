#[macro_use]
extern crate nom;
use nom::IResult;
use nom::ErrorKind;
use nom::IResult::Done;
use nom::Needed;
use std::io;
use std::io::Read;
use std::fs::File;

pub mod ast;
pub mod triple_stream;
mod grammar;
mod grammar_structs;
mod grammar_helper;

use grammar::turtle;
use grammar_structs::*;
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

struct State {
    triples: Vec<ast::Triple>,
    base: String,
    prefixes: HashMap<String, String>,
    blank_nodes: HashMap<String, usize>,
    blank_node_count: usize,
}

impl State {
    fn new() -> State {
        State {
            triples: vec![],
            base: String::new(),
            prefixes: HashMap::new(),
            blank_nodes: HashMap::new(),
            blank_node_count: 0,
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
    fn new_blank(&mut self) -> usize {
        let blank = self.blank_node_count;
        self.blank_node_count += 1;
        blank
    }
    fn get_blank(&mut self, label: String) -> usize {
        if let Some(n) = self.blank_nodes.get(&label) {
            return *n;
        }
        let n = self.new_blank();
        self.blank_nodes.insert(label, n);
        n
    }
    fn add_triple(&mut self, triple: ast::Triple) {
        self.triples.push(triple)
    }
}

fn resolve_iri(iri: &IRI, state: &State) -> Result<String, String> {
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
    Ok(i)
}

fn make_blank(blank_node: BlankNode, state: &mut State) -> usize {
    match blank_node {
        BlankNode::Anon => state.new_blank(),
        BlankNode::BlankNode(label) => state.get_blank(label),
    }
}

fn make_collection(collection: Vec<Object>, state: &mut State) -> Result<usize, String> {
    let first = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");
    let rest = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");
    let nil = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");
    let head = state.new_blank();

    let mut node = head;
    for object in collection {
        let o = try!(make_object(object, state));
        state.add_triple(ast::Triple {
            subject: ast::Subject::BlankNode(node),
            predicate: first.clone(),
            object: o,
        });
        let next = state.new_blank();
        state.add_triple(ast::Triple {
            subject: ast::Subject::BlankNode(node),
            predicate: rest.clone(),
            object: ast::Object::BlankNode(next),
        });
        node = next;
    }
    state.add_triple(ast::Triple {
        subject: ast::Subject::BlankNode(node),
        predicate: rest.clone(),
        object: ast::Object::IRI(nil),
    });
    Ok(head)
}

fn make_subject(subject: Subject, state: &mut State) -> Result<ast::Subject, String> {
    Ok(match subject {
        Subject::IRI(iri) => {
            let iri = try!(resolve_iri(&iri, state));
            ast::Subject::IRI(iri)
        }
        Subject::BlankNode(blank) => ast::Subject::BlankNode(make_blank(blank, state)),
        Subject::Collection(collection) => {
            ast::Subject::BlankNode(try!(make_collection(collection, state)))
        }
    })
}

fn make_object(object: Object, state: &mut State) -> Result<ast::Object, String> {
    Ok(match object {
        Object::IRI(ref iri) => ast::Object::IRI(try!(resolve_iri(&iri, state))),
        Object::BlankNode(blank) => ast::Object::BlankNode(make_blank(blank, state)),
        Object::Collection(collection) => {
            ast::Object::BlankNode(try!(make_collection(collection, state)))
        }
        Object::Literal(Literal::LangString(v, l)) => ast::Object::LangString(v, l),
        Object::Literal(Literal::XsdString(v)) => ast::Object::XsdString(v),
        Object::Literal(Literal::XsdInteger(v)) => ast::Object::XsdInteger(v),
        Object::Literal(Literal::XsdDecimal(v)) => ast::Object::XsdDecimal(v),
        Object::Literal(Literal::XsdDouble(v)) => ast::Object::XsdDouble(v),
        Object::Literal(Literal::XsdBoolean(v)) => ast::Object::XsdBoolean(v),
        Object::Literal(Literal::TypedLiteral(v, t)) => {
            let datatype = try!(resolve_iri(&t, state));
            ast::Object::TypedLiteral(v, datatype)
        }
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = state.new_blank();
            let subject = ast::Subject::BlankNode(blank);
            try!(add_predicated_objects(subject, predicated_objects_list, state));
            ast::Object::BlankNode(blank)
        }
    })
}

fn add_predicated_objects(subject: ast::Subject,
                          predicated_objects_list: Vec<PredicatedObjects>,
                          state: &mut State)
                          -> Result<(), String> {
    for po in predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = ast::Triple {
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

pub fn parse(data: &str) -> Result<Vec<ast::Triple>, String> {
    let statements = try!(parse_nom(data));
    let mut state = State::new();
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
    Ok(state.triples)
}

pub fn run(path: &str) -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    let r = turtle(s.as_str());
    println!("{}", s);
    if let Done(a, b) = r {
        println!("=== LEFT OVER ===");
        println!("{}", a);
        println!("=================");
        println!("{:?}", b);
    } else {
        println!("{:?}", r);
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
    assert_eq!(r1, vec![]);
    let r2 = parse("<><><>.").unwrap();
    let t = ast::Triple {
        subject: ast::Subject::IRI(String::new()),
        predicate: String::new(),
        object: ast::Object::IRI(String::new()),
    };
    assert_eq!(r2, vec![t.clone()]);
    let r3 = parse("@prefix:<>.: : :.").unwrap();
    assert_eq!(r3, vec![t]);
}
