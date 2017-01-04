/// Generate rust code from an ontology
///

extern crate rdfio;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use rdfio::graph_writer;
use rdfio::graph::{Object, Graph, GraphCreator, Subject, Triple};
use rdfio::triple_stream::*;
use rdfio::triple64::*;
use rdfio::namespaces::Namespaces;
use rdfio::resource::{ResourceBase, Resource};
use rdfio::ontology::rdf::Property;
use rdfio::ontology::rdfs::{Class, Comment, Domain, Range};

type MyGraph = graph_writer::Graph<Triple64SPO, Triple64OPS>;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn read_file(path: &str) -> io::Result<String> {
    let mut f = match fs::File::open(path) {
        Err(e) => {
            println_stderr!("Cannot open file {}.", path);
            return Err(e);
        }
        Ok(f) => f,
    };
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn load_graph(data: &str, base: &str) -> rdfio::Result<(MyGraph, Namespaces)> {
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    let mut triples = try!(TripleIterator::new(data, base));
    while let Some(triple) = triples.next() {
        writer.add_triple(&try!(triple));
    }
    Ok((writer.collect().sort_blank_nodes(), triples.prefixes().clone()))
}

fn camel_case(str: &str) -> String {
    let mut s = str[0..1].to_uppercase();
    s.push_str(&str[1..]);
    s
}

fn snake_case(str: &str) -> String {
    if str == "type" {
        return String::from("a");
    }
    let mut s = str[0..1].to_lowercase();
    for c in str.chars().skip(1) {
        if c.is_uppercase() {
            s.push('_');
            for l in c.to_lowercase() {
                s.push(l);
            }
        } else {
            s.push(c);
        }
    }
    s
}

fn comment_escape(str: &str) -> String {
    str.replace("\n", "")
}

fn write_code<G, W>(classes: &Vec<Class<G>>,
                    properties: &Vec<Property<G>>,
                    prefixes: &Namespaces,
                    writer: &mut W)
                    -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    for class in classes {
        if let &Resource::IRI(ref iri) = class.this() {
            if let Some((prefix, name)) = prefixes.find_prefix(iri.as_str()) {
                try!(writer.write_all(b"\n/// "));
                try!(writer.write_all(prefix));
                try!(writer.write_all(b":"));
                try!(writer.write_all(name.as_bytes()));
                for comment in class.comment() {
                    if let &Resource::Literal(ref l) = comment.this() {
                        try!(writer.write_all(b"\n/// "));
                        try!(writer.write_all(comment_escape(l).as_bytes()));
                    }
                }
                try!(writer.write_all(b"\nclass!(\""));
                try!(writer.write_all(iri.as_bytes()));
                try!(writer.write_all(b"\", "));
                try!(writer.write_all(camel_case(name).as_bytes()));
                try!(writer.write_all(b");\n"));
            }
        }
    }
    for property in properties {
        if let &Resource::IRI(ref iri) = property.this() {
            if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri.as_str()) {
                for range in property.range() {
                    if let &Resource::IRI(ref range) = range.this() {
                        if let Some((prefix, range)) = prefixes.find_prefix(range.as_str()) {
                            try!(writer.write_all(b"\n/// "));
                            try!(writer.write_all(prop_prefix));
                            try!(writer.write_all(b":"));
                            try!(writer.write_all(prop.as_bytes()));
                            for comment in property.comment() {
                                if let &Resource::Literal(ref l) = comment.this() {
                                    try!(writer.write_all(b"\n/// "));
                                    try!(writer.write_all(comment_escape(l).as_bytes()));
                                }
                            }
                            try!(writer.write_all(b"\nproperty!(\""));
                            try!(writer.write_all(iri.as_bytes()));
                            try!(writer.write_all(b"\", "));
                            try!(writer.write_all(camel_case(prop).as_bytes()));
                            try!(writer.write_all(b", "));
                            try!(writer.write_all(snake_case(prop).as_bytes()));
                            try!(writer.write_all(b", "));
                            try!(writer.write_all(prefix));
                            try!(writer.write_all(b"::"));
                            try!(writer.write_all(camel_case(range).as_bytes()));
                            try!(writer.write_all(b"<G>);\n"));
                        }
                    }
                }
                for domain in property.domain() {
                    if let &Resource::IRI(ref domain) = domain.this() {
                        if let Some((prefix, domain)) = prefixes.find_prefix(domain.as_str()) {
                            try!(writer.write_all(b"impl<G> "));
                            try!(writer.write_all(prop_prefix));
                            try!(writer.write_all(b"::"));
                            try!(writer.write_all(camel_case(prop).as_bytes()));
                            try!(writer.write_all(b"<G> for "));
                            try!(writer.write_all(prefix));
                            try!(writer.write_all(b"::"));
                            try!(writer.write_all(camel_case(domain).as_bytes()));
                            try!(writer.write_all(b"<G> where G: graph::Graph {}\n"));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDFS_DOMAIN: &'static str = "http://www.w3.org/2000/01/rdf-schema#domain";
const RDFS_COMMENT: &'static str = "http://www.w3.org/2000/01/rdf-schema#comment";
const RDF_PROPERTY: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn infer(graph: &MyGraph) -> rdfio::Result<MyGraph> {
    // for every triple with rdfs:subClassOf infer that the subject and the
    // object are rdfs:Class instances
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    for triple in graph.iter().filter(|triple| triple.predicate() == RDFS_SUB_CLASS_OF) {
        writer.add(triple.subject(), RDF_TYPE, RDFS_CLASS);
        match triple.object() {
            Object::IRI(iri) => {
                writer.add(iri, RDF_TYPE, RDFS_CLASS);
            }
            Object::BlankNode(b) => {
                writer.add(b, RDF_TYPE, RDFS_CLASS);
            }
            _ => {}
        }
    }
    for triple in graph.iter() {
        writer.add_triple(&triple);
    }

    // temporarily add Class and Property to domain of Comment
    writer.add(RDFS_COMMENT, RDFS_DOMAIN, RDFS_CLASS);
    writer.add(RDFS_COMMENT, RDFS_DOMAIN, RDF_PROPERTY);
    Ok(writer.collect().sort_blank_nodes())
}

fn get_classes(graph: &Rc<MyGraph>) -> rdfio::Result<Vec<Class<MyGraph>>> {
    let mut classes = Vec::new();
    for t in graph.iter_object_iri_predicate(RDFS_CLASS, RDF_TYPE) {
        if let Subject::IRI(iri) = t.subject() {
            classes.push(Class::new(&Resource::IRI(Rc::new(String::from(iri))), &graph));
        }
    }
    Ok(classes)
}

fn get_properties(graph: &Rc<MyGraph>) -> rdfio::Result<Vec<Property<MyGraph>>> {
    let mut properties = Vec::new();
    for t in graph.iter_object_iri_predicate(RDF_PROPERTY, RDF_TYPE) {
        if let Subject::IRI(iri) = t.subject() {
            properties.push(Property::new(&Resource::IRI(Rc::new(String::from(iri))), &graph));
        }
    }
    Ok(properties)
}

fn run(path: &str, base: &str) -> rdfio::Result<()> {
    let data = try!(read_file(path));
    let (graph, prefixes) = try!(load_graph(data.as_str(), base));
    let graph = try!(infer(&graph));
    let graph = Rc::new(graph);
    let classes = try!(get_classes(&graph));
    let properties = try!(get_properties(&graph));
    try!(write_code(&classes, &properties, &prefixes, &mut ::std::io::stdout()));
    Ok(())
}

fn check_file<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(meta) => meta.is_file(),
        _ => false,
    }
}

fn main() {
    let mut args = args();
    args.next();
    let mut args_ok = false;
    let mut base = String::new();
    let mut path = String::new();
    if args.len() == 1 {
        path = args.next().unwrap();
        args_ok = check_file(&path);
        base.push_str("file:");
        base.push_str(&path);
    } else if args.len() == 3 {
        args_ok = args.next().unwrap() == "--base";
        base.push_str(&args.next().unwrap());
        path = args.next().unwrap();
        args_ok &= check_file(&path);
    }
    if !args_ok {
        println_stderr!("Usage: [--base BASE] infer_classes INPUT_FILE");
        std::process::exit(-1);
    }
    if let Err(e) = run(path.as_str(), base.as_str()) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
