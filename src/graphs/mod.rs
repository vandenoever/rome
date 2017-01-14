pub mod tel;
#[macro_use]
pub mod collection;

use self::collection::graph_collection::*;

graph_collection!(test_collection(0: tel::Graph64, 1: tel::Graph128));

