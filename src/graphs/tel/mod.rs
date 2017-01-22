mod compact_triple;
mod triple128;
mod triple64;
mod graph_creator;
mod string_collector;
mod graph;
mod iter;
mod triple;

pub use self::triple128::{Triple128SPO, Triple128OPS};
pub use self::triple64::{Triple64SPO, Triple64OPS};

pub type Graph64 = graph::Graph<Triple64SPO, Triple64OPS>;
pub type Graph128 = graph::Graph<Triple128SPO, Triple128OPS>;
pub type GraphCreator<A, B> = graph_creator::GraphCreator<A, B>;
