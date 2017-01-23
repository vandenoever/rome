//! A number of Graph implementations.
#[macro_use]
pub mod collection;
pub mod tel;

use self::collection::*;

use std::cmp;

impl_triple_cmp_wrap!(tel::Graph64);
impl_triple_cmp_wrap!(tel::Graph128);

#[cfg(test)]
mod test {
    graph_collection!(test_collection(0: super::super::tel::Graph64,
                                      1: super::super::tel::Graph128));

    #[test]
    fn test() {
        use graph::{Graph, GraphWriter};
        let mut gw1 = super::tel::GraphCreator::with_capacity(0);
        let b1 = gw1.create_blank_node();
        let p1 = gw1.create_iri(&"p");
        gw1.add_blank_blank(&b1, &p1, &b1);
        let g1: super::tel::Graph64 = gw1.collect();
        let mut gw2 = super::tel::GraphCreator::with_capacity(0);
        let b2 = gw2.create_blank_node();
        let p2 = gw2.create_iri(&"p");
        gw2.add_blank_blank(&b2, &p2, &b2);
        let g2: super::tel::Graph128 = gw2.collect();
        let g = test_collection::GraphCollection::new((&g1, &g2));
        assert_eq!(g.iter().count(), 2);
    }
}
