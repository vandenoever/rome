use graph;
use std;

pub struct OntologyAdapter<G>
    where G: graph::Graph
{
    graph: std::rc::Rc<G>,
    iris: Vec<Option<<G::SPOTriple as graph::Triple>::PredicatePtr>>,
}

impl<G> OntologyAdapter<G>
    where G: graph::Graph
{
    pub fn new(graph: &std::rc::Rc<G>,
           iris: Vec<Option<<G::SPOTriple as graph::Triple>::PredicatePtr>>)
            -> OntologyAdapter<G> {
        OntologyAdapter {
            graph: graph.clone(),
            iris: iris,
        }

    }
    pub fn class_iri(&self, i: usize) -> Option<&<G::SPOTriple as graph::Triple>::PredicatePtr> {
        match self.iris.get(i) {
            Some(&Some(ref p)) => Some(p),
            _ => None,
        }
    }
    pub fn iter_s_p(&self,
                subject: <G::SPOTriple as graph::Triple>::SubjectPtr,
                predicate: <G::SPOTriple as graph::Triple>::PredicatePtr)
                -> G::SPORangeIter {
        self.graph.iter_s_p(subject, predicate)
    }
    pub fn empty_spo_range(&self) -> G::SPORangeIter {
        self.graph.empty_spo_range()
    }
    pub fn object_to_subject(&self, object: <G::SPOTriple as graph::Triple>::ObjectPtr)
            -> Option<<G::SPOTriple as graph::Triple>::SubjectPtr> {
        self.graph.object_to_subject(object)
    }
}
