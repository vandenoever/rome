#![feature(ptr_eq)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate rand;
extern crate regex;

use nom::ErrorKind;

pub mod error;
pub mod graph;
pub mod graph_writer;
pub mod triple_stream;
pub mod turtle_writer;
pub mod namespaces;

pub use error::Result;

mod grammar;
mod grammar_helper;
mod grammar_structs;
mod string_collector;
mod compact_triple;
pub mod triple64;
pub mod triple128;

#[cfg(unstable)]
mod string_store;
#[cfg(unstable)]
mod unsafe_graph;
#[cfg(unstable)]
mod unsafe_key;
#[cfg(unstable)]
mod index_graph;
#[cfg(unstable)]
mod mem_graph;
#[cfg(unstable)]
mod ntriples_writer;
