//! An memory-based RDF graph.
//!
//! This module contains two implementations of `rome::graph::Graph`.
//! One is based on a 64 bit representation of triples and the other on a
//! 128 bit representation of the triples. To make sure that all triples fit,
//! it is recommended to use `Graph128`.

mod compact_triple;
mod graph;
mod graph_creator;
mod iter;
mod string_collector;
mod triple;
mod triple128;
mod triple64;

pub use self::triple128::{Triple128OPS, Triple128SPO};
pub use self::triple64::{Triple64OPS, Triple64SPO};

/// Implementation of `rome::graph::Graph` that stores triples in 64 bits.
pub type Graph64 = graph::Graph<Triple64SPO, Triple64OPS>;
/// Implementation of `rome::graph::Graph` that stores triples in 128 bits.
pub type Graph128 = graph::Graph<Triple128SPO, Triple128OPS>;
/// Implementation of `rome::graph::GraphCreator` that can create Graph64
/// and Graph128.
pub type GraphCreator<A, B> = graph_creator::GraphCreator<A, B>;
