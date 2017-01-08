/// A simple example program that infers classes
///
/// The program reads a Turtle or N-Triples file and infers Class instances by
/// using rdf:subClassOf.

extern crate rdfio;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use rdfio::io::{TurtleParser, write_turtle};
use rdfio::graph::{Object, Graph, GraphCreator, Triple};
use rdfio::graphs::tel;
use rdfio::namespaces::Namespaces;

type MyGraph = tel::Graph64;

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

fn load_graph(data: &str, base: &str) -> rdfio::Result<MyGraph> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    let mut triples = try!(TurtleParser::new(data, base));
    while let Some(triple) = triples.next() {
        writer.add_triple(&try!(triple));
    }
    Ok(writer.collect().sort_blank_nodes())
}

fn output_as_turtle(graph: &MyGraph) -> rdfio::Result<()> {
    let mut ns = Namespaces::new();
    ns.set(b"rdfs", "http://www.w3.org/2000/01/rdf-schema#");
    try!(write_turtle(&ns, graph.iter(), &mut ::std::io::stdout()));
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn infer(graph: &MyGraph) -> rdfio::Result<MyGraph> {
    // for every triple with rdfs:subClassOf infer that the subject and the
    // object are rdfs:Class instances
    let mut writer = tel::GraphCreator::with_capacity(65000);
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
    Ok(writer.collect().sort_blank_nodes())
}

fn run(path: &str, base: &str) -> rdfio::Result<()> {
    let data = try!(read_file(path));
    let graph = try!(load_graph(data.as_str(), base));
    let result = try!(infer(&graph));
    try!(output_as_turtle(&result));
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
