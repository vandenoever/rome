use graph::*;

pub struct OntologyAdapter<'g, G: 'g>
    where G: Graph<'g>
{
    graph: &'g G,
    iris: Vec<Option<G::IRIPtr>>,
}

impl<'g, G> OntologyAdapter<'g, G>
    where G: Graph<'g>
{
    pub fn new(graph: &'g G, iris: Vec<Option<G::IRIPtr>>) -> OntologyAdapter<'g, G> {
        OntologyAdapter {
            graph: graph,
            iris: iris,
        }
    }
    pub fn class_iri(&self, i: usize) -> Option<&G::IRIPtr> {
        match self.iris.get(i) {
            Some(&Some(ref p)) => Some(p),
            _ => None,
        }
    }
    pub fn iter_s_p(&self,
                    subject: BlankNodeOrIRI<'g, G::BlankNodePtr, G::IRIPtr>,
                    predicate: G::IRIPtr)
                    -> G::SPORangeIter {
        self.graph.iter_s_p(subject, predicate)
    }
    pub fn iter_o_p(&self,
                    object: Resource<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
                    predicate: G::IRIPtr)
                    -> G::OPSRangeIter {
        self.graph.iter_o_p(object, predicate)
    }
    pub fn empty_spo_range(&self) -> G::SPORangeIter {
        self.graph.empty_spo_range()
    }
    pub fn empty_ops_range(&self) -> G::OPSRangeIter {
        self.graph.empty_ops_range()
    }/*
    pub fn object_to_subject(&self, object: G::ObjectPtr) -> Option<G::SubjectPtr> {
        self.graph.object_to_subject(object)
    }
    pub fn object_to_predicate(&self, object: G::ObjectPtr) -> Option<G::PredicatePtr> {
        self.graph.object_to_predicate(object)
    }
    pub fn subject_to_object(&self, subject: G::SubjectPtr) -> G::ObjectPtr {
        self.graph.subject_to_object(subject)
    }
    pub fn predicate_to_object(&self, predicate: G::PredicatePtr) -> G::ObjectPtr {
        self.graph.predicate_to_object(predicate)
    }*/
}
