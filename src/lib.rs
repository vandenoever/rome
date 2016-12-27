#[macro_use]
extern crate nom;
extern crate rand;
use nom::ErrorKind;
use std::io;
use std::io::Read;
use std::fs::File;

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
pub mod graph_writer;
mod string_collector;
pub mod triple_to_uint;

use triple_stream::*;
use graph::{WritableGraph, Graph};
use mem_graph::MemGraph;
use std::collections::HashMap;

pub fn parse(data: &str) -> Result<(MemGraph, HashMap<&str, String>), String> {
    let mut graph = MemGraph::new();
    let mut triples = try!(TripleIterator::new(data));
    while let Some(triple) = triples.next() {
        graph.add_triple(&try!(triple));
    }
    Ok((graph, triples.prefixes().clone()))
}

pub fn run(path: &str) -> io::Result<MemGraph> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    match parse(s.as_str()) {
        Ok((graph, _)) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            try!(ntriples_writer::write_ntriples(&graph, &mut handle));
            Ok(graph)
        }
        Err(e) => {
            println!("{}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}

#[test]
fn test_run() {
    let path = "/tmp/tracker/tests/libtracker-data/update/delete-insert-where-1.ontology";
    match run(&path) {
        Err(e) => {
            println!("{:?}", e);
        }
        Ok(graph) => {
            println!("got graph with {} triples", graph.len());
        }
    }
}
