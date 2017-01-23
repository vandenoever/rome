//! OntologyAdapter allows accessing of a graph via an ontology.
use graph::*;

/// OntologyAdapter allows accessing of a graph via an ontology.
pub struct OntologyAdapter<'g, G: 'g>
    where G: Graph<'g>
{
    graph: &'g G,
    preloaded_iris: Vec<Option<G::IRIPtr>>,
}

impl<'g, G> OntologyAdapter<'g, G>
    where G: Graph<'g>
{
    /// Create a new OntologyAdapter.
    pub fn new(graph: &'g G, preloaded_iris: Vec<Option<G::IRIPtr>>) -> OntologyAdapter<'g, G> {
        OntologyAdapter {
            graph: graph,
            preloaded_iris: preloaded_iris,
        }
    }
    #[doc(hidden)]
    pub fn preloaded_iri(&self, i: usize) -> Option<&G::IRIPtr> {
        self.preloaded_iris[i].as_ref()
    }
    #[doc(hidden)]
    pub fn iter_s_p(&self,
                    subject: BlankNodeOrIRI<'g, G::BlankNodePtr, G::IRIPtr>,
                    predicate: G::IRIPtr)
                    -> G::SPORangeIter {
        self.graph.iter_s_p(subject, predicate)
    }
    #[doc(hidden)]
    pub fn iter_o_p(&self,
                    object: Resource<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
                    predicate: G::IRIPtr)
                    -> G::OPSRangeIter {
        self.graph.iter_o_p(object, predicate)
    }
    #[doc(hidden)]
    pub fn empty_spo_range(&self) -> G::SPORangeIter {
        self.graph.empty_spo_range()
    }
    #[doc(hidden)]
    pub fn empty_ops_range(&self) -> G::OPSRangeIter {
        self.graph.empty_ops_range()
    }
}
