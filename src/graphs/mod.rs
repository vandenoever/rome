pub mod tel;
#[macro_use]
pub mod collection;

use self::collection::graph_collection::*;

graph_collection!(test_collection(0: tel::Graph64, 1: tel::Graph128));

// graph_collection!(test_collection(0: ::graphs::tel::graph::Graph<::graphs::tel::Triple64SPO, ::graphs::tel::Triple64OPS>));