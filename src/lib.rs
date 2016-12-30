#![feature(ptr_eq)]
#[macro_use]
extern crate nom;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate regex;
use nom::ErrorKind;
use std::io;
use std::io::Read;
use std::fs::File;

mod unsafe_key;
pub mod triple_stream;
pub mod error;
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
pub use error::Result;

pub fn parse(data: &str) -> Result<(MemGraph, HashMap<String, String>)> {
    let mut graph = MemGraph::new();
    let mut triples = try!(TripleIterator::new(data, ""));
    while let Some(triple) = triples.next() {
        graph.add_triple(&try!(triple));
    }
    Ok((graph, triples.prefixes()))
}

pub fn parse2(data: &str) -> Result<(graph_writer::Graph, HashMap<String, String>)> {
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    let mut triples = try!(TripleIterator::new(data, ""));
    while let Some(triple) = triples.next() {
        writer.add_triple(&try!(triple));
    }
    Ok((writer.collect(), triples.prefixes()))
}

pub fn run(path: &str) -> Result<graph_writer::Graph> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    let (graph, _) = try!(parse(s.as_str()));
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    try!(ntriples_writer::write_ntriples(graph.iter(), &mut handle));
    run2(s)
}
fn run2(s: String) -> Result<graph_writer::Graph> {
    let (graph, _) = try!(parse2(s.as_str()));
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    try!(ntriples_writer::write_ntriples(graph.iter(), &mut handle));
    Ok(graph)
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
