use graph;
use graph::Triple;
use std::marker::PhantomData;
use std::rc::Rc;
use ontology_adapter;
use std;
use resource;

pub struct ObjectIter<G, R>
    where G: graph::Graph
{
    pub graph: Rc<ontology_adapter::OntologyAdapter<G>>,
    pub iter: G::SPORangeIter,
    pub phantom: PhantomData<R>,
}

impl<G, R> Iterator for ObjectIter<G, R>
    where G: graph::Graph,
          R: ResourceBase<G>
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => Some(R::new(triple.object_ptr(), &self.graph)),
            None => None,
        }
    }
}


pub struct SubjectIter<G, R>
    where G: graph::Graph
{
    pub graph: Rc<ontology_adapter::OntologyAdapter<G>>,
    pub iter: G::OPSRangeIter,
    pub phantom: PhantomData<R>,
}

impl<G, R> Iterator for SubjectIter<G, R>
    where G: graph::Graph,
          R: ResourceBase<G>
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => {
                let s = triple.subject_ptr();
                let o = self.graph.subject_to_object(s);
                Some(R::new(o, &self.graph))
            }
            None => None,
        }
    }
}

pub trait ResourceBase<G>: Eq + std::marker::Sized
    where G: graph::Graph
{
    fn new(this: <G::SPOTriple as graph::Triple>::ObjectPtr,
           graph: &Rc<ontology_adapter::OntologyAdapter<G>>)
           -> Self;
    fn iter(graph: &Rc<ontology_adapter::OntologyAdapter<G>>) -> SubjectIter<G, Self>;
    fn this(&self) -> &<G::SPOTriple as graph::Triple>::ObjectPtr;
    fn graph(&self) -> &ontology_adapter::OntologyAdapter<G>;
    fn predicate<R>(&self, predicate: Option<<G::SPOTriple as graph::Triple>::PredicatePtr>)
            -> ObjectIter<G, R> where R: ResourceBase<G>;
}

macro_rules! property{(
    $iri:expr,
    $trait_name:ident,
    $function:ident,
    $range:path,
    $pos:expr) => {

pub trait $trait_name<G>: resource::ResourceBase<G>
    where G: graph::Graph,
          Self: std::marker::Sized
{
    fn property_iri() -> &'static str {
        $iri
    }
    fn property_iri_ptr(&self) -> Option<<G::SPOTriple as graph::Triple>::PredicatePtr> {
        self.graph().class_iri($pos).map(|i|i.clone())
    }
    fn $function(&self) -> resource::ObjectIter<G, $range> {
        resource::ResourceBase::predicate(self, self.property_iri_ptr())
    }
}
    }
}

macro_rules! class{(
    $iri:expr,
    $name:ident,
    $pos:expr) => {

pub struct $name<G>
    where G: graph::Graph
{
    resource: G::ObjectPtr,
    graph: std::rc::Rc<ontology_adapter::OntologyAdapter<G>>,
    phantom: std::marker::PhantomData<G>,
}
impl<G> $name<G>
    where G: graph::Graph
{
    pub fn class_iri() -> &'static str {
        $iri
    }
    pub fn class_iri_ptr(&self) -> Option<G::PredicatePtr> {
        self.graph.class_iri($pos).map(|i|i.clone())
    }
}
impl<G> PartialEq for $name<G>
    where G: graph::Graph
{
    fn eq(&self, rhs: &$name<G>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<G> Eq for $name<G> where G: graph::Graph {}
impl<'a, G> resource::ResourceBase<G> for $name<G>
    where G: graph::Graph
{
    fn new(resource: G::ObjectPtr,
            graph: &std::rc::Rc<ontology_adapter::OntologyAdapter<G>>) -> Self {
        $name {
            resource: resource,
            graph: graph.clone(),
            phantom: std::marker::PhantomData,
        }
    }
    fn iter(graph: &std::rc::Rc<ontology_adapter::OntologyAdapter<G>>)
            -> resource::SubjectIter<G, Self> {
        let a = graph.class_iri(0);
        let c = graph.class_iri($pos);
        let iter = match (a, c) {
            (Some(a),Some(c)) => graph.iter_o_p(graph.predicate_to_object(c.clone()), a.clone()),
            _ => graph.empty_ops_range()
        };
        resource::SubjectIter {
            graph: graph.clone(),
            iter: iter,
            phantom: std::marker::PhantomData,
        }
    }
    fn this(&self) -> &<G::SPOTriple as graph::Triple>::ObjectPtr {
        &self.resource
    }
    fn graph(&self) -> &ontology_adapter::OntologyAdapter<G> {
        &self.graph
    }

    fn predicate<R>(&self, predicate: Option<G::PredicatePtr>)
            -> resource::ObjectIter<G, R>
        where R: resource::ResourceBase<G>
    {
        let iter = match predicate {
            None => self.graph.empty_spo_range(),
            Some(ref predicate) => match self.graph().object_to_subject(self.this().clone()) {
                Some(ref subject) => self.graph.iter_s_p(subject.clone(), predicate.clone()),
                None => self.graph.empty_spo_range(),
            }
        };
        resource::ObjectIter {
            graph: self.graph.clone(),
            iter: iter,
            phantom: std::marker::PhantomData,
        }
    }
}
}}

macro_rules! sub_class_of{(
    $subclass:ident,
    $class:ident) => {

impl<G> From<$subclass<G>> for $class<G> where G: graph::Graph {
    fn from(f: $subclass<G>) -> Self {
        $class {
            iri: f.iri,
            graph: f.graph,
        }
    }
}

}}

pub fn adapter<G>(graph: &std::rc::Rc<G>) -> ontology_adapter::OntologyAdapter<G>
    where G: graph::Graph
{
    let mut iris = Vec::with_capacity(7);
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Class"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Literal"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#comment"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#domain"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#range"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#subClassOf"));
    ontology_adapter::OntologyAdapter::new(graph, iris)
}

class!("http://www.w3.org/2000/01/rdf-schema#Class", Class, 1);
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property",
       Property,
       2);
class!("http://www.w3.org/2000/01/rdf-schema#Literal", Literal, 3);
property!("http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, Literal<G>, 4);
property!("http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, Class<G>, 5);
property!("http://www.w3.org/2000/01/rdf-schema#range", Range, range, Class<G>, 6);
property!("http://www.w3.org/2000/01/rdf-schema#subClassOf",
    SubClassOf, sub_class_of, Class<G>, 7);

impl<G> SubClassOf<G> for Class<G> where G: graph::Graph {}
impl<G> Comment<G> for Class<G> where G: graph::Graph {}
impl<G> Domain<G> for Property<G> where G: graph::Graph {}
impl<G> Range<G> for Property<G> where G: graph::Graph {}
impl<G> Comment<G> for Property<G> where G: graph::Graph {}
