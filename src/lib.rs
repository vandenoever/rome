#![warn(missing_docs)]
/// A crate for working with RDF
/// By implementing graph::Graph, one can make any data source available as RDF.
/// Ontology wrappers can be generated from RDF Schema.
///
/// This crate is in early development.
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate rand;
extern crate regex;

pub mod error;
pub mod graph;
pub mod graphs;
pub mod io;
pub mod namespaces;

pub use error::Result;

pub mod iter;
#[macro_use]
pub mod resource;
pub mod ontology;
pub mod ontology_adapter;
mod constants;

#[cfg(unstable)]
mod unstable;
