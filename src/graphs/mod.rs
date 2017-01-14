pub mod tel;
#[macro_use]
pub mod collection;

use self::collection::graph_collection::*;

use std::cmp;
use graph::*;

impl_triple_cmp_wrap!(tel::Graph64);
impl_triple_cmp_wrap!(tel::Graph128);
graph_collection!(test_collection(0: tel::Graph64, 1: tel::Graph128));
graph_collection!(test_collection2(0: tel::Graph64, 1: tel::Graph128));
