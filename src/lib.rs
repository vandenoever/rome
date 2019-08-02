#![warn(missing_docs)]
#![allow(unused_doc_comments)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
//! A crate for working with RDF
//! By implementing `graph::Graph`, one can make any data source available as RDF.
//! Ontology wrappers can be generated from RDF Schema.
//!
//! This crate is in early development.

pub mod error;
pub mod graph;
pub mod graphs;
pub mod io;
pub mod iter;
pub mod namespaces;
pub mod ontology_adapter;
#[macro_use]
pub mod resource;
/// Ontology mapping for rdf: and rdfs:
///
/// This code was generated by `examples/generate_code`.
pub mod ontology;

pub use crate::error::Result;

#[cfg(unstable)]
mod unstable;
