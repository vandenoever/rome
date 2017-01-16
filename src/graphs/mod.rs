#[macro_use]
pub mod collection;
pub mod tel;

use self::collection::graph_collection::*;

use std::cmp;

impl_triple_cmp_wrap!(tel::Graph64);
impl_triple_cmp_wrap!(tel::Graph128);
graph_collection!(test_collection(0: super::tel::Graph64, 1: super::tel::Graph128));
graph_collection!(test_collection2(0: super::tel::Graph64, 1: super::tel::Graph128));
