mod compact_triple;
mod triple128;
mod triple64;
mod graph_writer;
mod string_collector;

use self::triple64::*;
use self::triple128::*;

pub type Graph64 = graph_writer::Graph<Triple64SPO, Triple64OPS>;
pub type Graph128 = graph_writer::Graph<Triple128SPO, Triple128OPS>;
pub type GraphCreator<A, B> = graph_writer::GraphWriter<A, B>;
