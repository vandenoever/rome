mod compact_triple;
mod triple128;
mod triple64;
mod graph_creator;
mod string_collector;
mod graph;
mod iter;
mod triple;

use self::triple64::*;
use self::triple128::*;

pub type Graph64 = graph::Graph<Triple64SPO, Triple64OPS>;
pub type Graph128 = graph::Graph<Triple128SPO, Triple128OPS>;
pub type BlankNodeCreator<A, B> = graph_creator::BlankNodeCreator<A, B>;
pub type GraphCreator<A, B> = graph_creator::GraphCreator<A, B>;
