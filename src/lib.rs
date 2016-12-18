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

fn make_subject(subject: &IRI,
                base: &String,
                prefixes: &HashMap<String, String>)
                -> Result<ast::Subject, String> {
    let iri = try!(resolve_iri(subject, base, prefixes));
    Ok(ast::Subject::IRI(iri))
}

fn make_object(object: Object,
               base: &String,
               prefixes: &HashMap<String, String>)
               -> Result<ast::Object, String> {
    let o = match object {
        Object::IRI(ref iri) => ast::Object::IRI(try!(resolve_iri(&iri, base, prefixes))),
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
    };
    Ok(o)
}

fn add_triples(triples: &mut Vec<ast::Triple>,
               new_triples: Triples,
               base: &String,
               prefixes: &HashMap<String, String>)
               -> Result<(), String> {
    let subject = try!(make_subject(&new_triples.subject, base, prefixes));
    for po in new_triples.predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = ast::Triple {
                subject: subject.clone(),
                predicate: try!(resolve_iri(&po.verb, base, prefixes)),
                object: try!(make_object(o, base, prefixes)),
            };
            triples.push(triple);
        }
    }
    Ok(())
}

pub fn parse(data: &str) -> Result<Vec<ast::Triple>, String> {
    let statements = try!(parse_nom(data));
    let mut triples = vec![];
    let mut base = String::new();
    let mut prefixes = HashMap::new();
    for statement in statements {
        match statement {
            Statement::Prefix(prefix, iri) => {
                prefixes.insert(prefix, resolve_iri_ref(&iri, &base));
            }
            Statement::Base(new_base) => {
                base = new_base;
            }
            Statement::Triples(new_triples) => {
                try!(add_triples(&mut triples, new_triples, &base, &prefixes));
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
