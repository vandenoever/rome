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

fn resolve_iri_ref(iri: &String, base: &String) -> String {
    iri.clone()
}

struct State {
    triples: Vec<ast::Triple>,
    base: String,
    prefixes: HashMap<String, String>,
    blank_nodes: HashMap<String, usize>,
    blank_node_count: usize,
}

fn resolve_iri(iri: &IRI,
               base: &String,
               prefixes: &HashMap<String, String>)
               -> Result<String, String> {
    let i = match *iri {
        IRI::IRI(ref iri) => resolve_iri_ref(iri, base),
        IRI::PrefixedName(ref prefix, ref local) => {
            let base = match prefixes.get(prefix) {
                Some(base) => base.clone(),
                None => return Err(format!("Prefix {} was not defined.", prefix)),
            };
            base + local
        }
    };
    Ok(i)
}

fn new_blank(blank_node_count: &mut usize) -> usize {
    let blank = *blank_node_count;
    *blank_node_count += 1;
    blank
}

fn make_blank(blank_node: BlankNode,
              blank_nodes: &mut HashMap<String, usize>,
              blank_node_count: &mut usize)
              -> usize {
    match blank_node {
        BlankNode::Anon => new_blank(blank_node_count),
        BlankNode::BlankNode(label) => {
            let blank = blank_nodes.entry(label).or_insert_with(|| new_blank(blank_node_count));
            *blank
        }
    }
}

fn make_collection(collection: Vec<Object>,
                   triples: &mut Vec<ast::Triple>,
                   base: &String,
                   prefixes: &HashMap<String, String>,
                   blank_nodes: &mut HashMap<String, usize>,
                   blank_node_count: &mut usize)
                   -> Result<usize, String> {
    let first = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");
    let rest = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");
    let nil = String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");
    let head = new_blank(blank_node_count);

    let mut node = head;
    for object in collection {
        let o = try!(make_object(triples,
                                 object,
                                 base,
                                 prefixes,
                                 blank_nodes,
                                 blank_node_count));
        triples.push(ast::Triple {
            subject: ast::Subject::BlankNode(node),
            predicate: first.clone(),
            object: o,
        });
        let next = new_blank(blank_node_count);
        triples.push(ast::Triple {
            subject: ast::Subject::BlankNode(node),
            predicate: rest.clone(),
            object: ast::Object::BlankNode(next),
        });
        node = next;
    }
    triples.push(ast::Triple {
        subject: ast::Subject::BlankNode(node),
        predicate: rest.clone(),
        object: ast::Object::IRI(nil),
    });
    Ok(head)
}

fn make_subject(subject: Subject,
                triples: &mut Vec<ast::Triple>,
                base: &String,
                prefixes: &HashMap<String, String>,
                blank_nodes: &mut HashMap<String, usize>,
                blank_node_count: &mut usize)
                -> Result<ast::Subject, String> {
    Ok(match subject {
        Subject::IRI(iri) => {
            let iri = try!(resolve_iri(&iri, base, prefixes));
            ast::Subject::IRI(iri)
        }
        Subject::BlankNode(blank) => {
            ast::Subject::BlankNode(make_blank(blank, blank_nodes, blank_node_count))
        }
        Subject::Collection(collection) => {
            ast::Subject::BlankNode(try!(make_collection(collection,
                                                         triples,
                                                         base,
                                                         prefixes,
                                                         blank_nodes,
                                                         blank_node_count)))
        }
    })
}

fn make_object(triples: &mut Vec<ast::Triple>,
               object: Object,
               base: &String,
               prefixes: &HashMap<String, String>,
               blank_nodes: &mut HashMap<String, usize>,
               blank_node_count: &mut usize)
               -> Result<ast::Object, String> {
    Ok(match object {
        Object::IRI(ref iri) => ast::Object::IRI(try!(resolve_iri(&iri, base, prefixes))),
        Object::BlankNode(blank) => {
            ast::Object::BlankNode(make_blank(blank, blank_nodes, blank_node_count))
        }
        Object::Collection(collection) => {
            ast::Object::BlankNode(try!(make_collection(collection,
                                                        triples,
                                                        base,
                                                        prefixes,
                                                        blank_nodes,
                                                        blank_node_count)))
        }
        Object::Literal(Literal::LangString(v, l)) => ast::Object::LangString(v, l),
        Object::Literal(Literal::XsdString(v)) => ast::Object::XsdString(v),
        Object::Literal(Literal::XsdInteger(v)) => ast::Object::XsdInteger(v),
        Object::Literal(Literal::XsdDecimal(v)) => ast::Object::XsdDecimal(v),
        Object::Literal(Literal::XsdDouble(v)) => ast::Object::XsdDouble(v),
        Object::Literal(Literal::XsdBoolean(v)) => ast::Object::XsdBoolean(v),
        Object::Literal(Literal::TypedLiteral(v, t)) => {
            let datatype = try!(resolve_iri(&t, base, prefixes));
            ast::Object::TypedLiteral(v, datatype)
        }
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = new_blank(blank_node_count);
            let subject = ast::Subject::BlankNode(blank);
            try!(add_predicated_objects(triples,
                                        subject,
                                        predicated_objects_list,
                                        base,
                                        prefixes,
                                        blank_nodes,
                                        blank_node_count));
            ast::Object::BlankNode(blank)
        }
    })
}

fn add_predicated_objects(triples: &mut Vec<ast::Triple>,
                          subject: ast::Subject,
                          predicated_objects_list: Vec<PredicatedObjects>,
                          base: &String,
                          prefixes: &HashMap<String, String>,
                          blank_nodes: &mut HashMap<String, usize>,
                          blank_node_count: &mut usize)
                          -> Result<(), String> {
    for po in predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = ast::Triple {
                subject: subject.clone(),
                predicate: try!(resolve_iri(&po.verb, base, prefixes)),
                object: try!(make_object(triples,
                                         o,
                                         base,
                                         prefixes,
                                         blank_nodes,
                                         blank_node_count)),
            };
            triples.push(triple);
        }
    }
    Ok(())
}

fn add_triples(triples: &mut Vec<ast::Triple>,
               new_triples: Triples,
               base: &String,
               prefixes: &HashMap<String, String>,
               blank_nodes: &mut HashMap<String, usize>,
               blank_node_count: &mut usize)
               -> Result<(), String> {
    let subject = try!(make_subject(new_triples.subject,
                                    triples,
                                    base,
                                    prefixes,
                                    blank_nodes,
                                    blank_node_count));
    add_predicated_objects(triples,
                           subject,
                           new_triples.predicated_objects_list,
                           base,
                           prefixes,
                           blank_nodes,
                           blank_node_count)
}

pub fn parse(data: &str) -> Result<Vec<ast::Triple>, String> {
    let statements = try!(parse_nom(data));
    let mut triples = vec![];
    let mut base = String::new();
    let mut prefixes = HashMap::new();
    let mut blank_nodes = HashMap::new();
    let mut blank_node_count = 0;
    for statement in statements {
        match statement {
            Statement::Prefix(prefix, iri) => {
                prefixes.insert(prefix, resolve_iri_ref(&iri, &base));
            }
            Statement::Base(new_base) => {
                base = new_base;
            }
            Statement::Triples(new_triples) => {
                try!(add_triples(&mut triples,
                                 new_triples,
                                 &base,
                                 &prefixes,
                                 &mut blank_nodes,
                                 &mut blank_node_count));
            }
        }
    }
    Ok(triples)
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
