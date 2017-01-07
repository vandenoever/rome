use graph;
use graph::Triple;
use std::rc::Rc;
use ontology_adapter;
use std;
use resource;

pub trait ResourceBase: Eq + std::marker::Sized {
    type Graph: graph::Graph;
    type SubjectIter: Iterator<Item = Self>;
    fn new(this: <Self::Graph as graph::Graph>::ObjectPtr,
           graph: &Rc<ontology_adapter::OntologyAdapter<Self::Graph>>)
           -> Self;
    fn iter(graph: &Rc<ontology_adapter::OntologyAdapter<Self::Graph>>) -> Self::SubjectIter;
    fn this(&self) -> &<Self::Graph as graph::Graph>::ObjectPtr;
    fn graph(&self) -> &ontology_adapter::OntologyAdapter<Self::Graph>;
    fn predicate<R>(&self,
                    predicate: Option<<Self::Graph as graph::Graph>::PredicatePtr>)
                    -> ObjectIter<R>
        where R: ResourceBase<Graph = Self::Graph>;
    fn iri(self) -> Option<IRI<Self>> {
        self.graph().object_to_predicate(self.this().clone()).map(|_| IRI { resource: self })
    }
}

// a wrapper around 'ResourceBase' that guarantees that the resource
// is an iri and not a blank node or a literal
pub struct IRI<R>
    where R: ResourceBase
{
    resource: R,
}

impl<R> IRI<R>
    where R: ResourceBase
{
    pub fn iri(&self) -> &str {
        use std::ops::Deref;
        use graph::SubjectPtr;
        self.deref().this().iri().unwrap()
    }
    pub fn this(&self) -> () {}
}

impl<R: ?Sized> std::ops::Deref for IRI<R>
    where R: ResourceBase
{
    type Target = R;

    #[inline(always)]
    fn deref(&self) -> &R {
        &self.resource
    }
}

impl<R> PartialEq for IRI<R>
    where R: ResourceBase
{
    fn eq(&self, rhs: &IRI<R>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<R> Eq for IRI<R> where R: ResourceBase {}

impl<R> ResourceBase for IRI<R>
    where R: ResourceBase
{
    type Graph = R::Graph;
    type SubjectIter = IRISubjectIter<R>;
    fn new(this: <Self::Graph as graph::Graph>::ObjectPtr,
           graph: &Rc<ontology_adapter::OntologyAdapter<Self::Graph>>)
           -> Self {
        IRI { resource: R::new(this, graph) }
    }
    fn iter(graph: &Rc<ontology_adapter::OntologyAdapter<Self::Graph>>) -> Self::SubjectIter {
        IRISubjectIter { iter: R::iter(graph) }
    }
    fn this(&self) -> &<Self::Graph as graph::Graph>::ObjectPtr {
        self.resource.this()
    }
    fn graph(&self) -> &ontology_adapter::OntologyAdapter<Self::Graph> {
        self.resource.graph()
    }
    fn predicate<O>(&self,
                    predicate: Option<<Self::Graph as graph::Graph>::PredicatePtr>)
                    -> ObjectIter<O>
        where O: ResourceBase<Graph = Self::Graph>{
        self.resource.predicate(predicate)
    }
}

pub struct ObjectIter<R>
    where R: ResourceBase
{
    pub graph: Rc<ontology_adapter::OntologyAdapter<R::Graph>>,
    pub iter: <R::Graph as graph::Graph>::SPORangeIter,
}

impl<R> Iterator for ObjectIter<R>
    where R: ResourceBase
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => Some(R::new(triple.object_ptr(), &self.graph)),
            None => None,
        }
    }
}

pub struct SubjectIter<R>
    where R: ResourceBase
{
    pub graph: Rc<ontology_adapter::OntologyAdapter<R::Graph>>,
    pub iter: <R::Graph as graph::Graph>::OPSRangeIter,
}

impl<R> Iterator for SubjectIter<R>
    where R: ResourceBase
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

pub struct IRISubjectIter<R>
    where R: ResourceBase
{
    iter: R::SubjectIter,
}

impl<R> Iterator for IRISubjectIter<R>
    where R: ResourceBase
{
    type Item = IRI<R>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(r) = self.iter.next() {
            if let Some(iri) = r.iri() {
                return Some(iri);
            }
        }
        None
    }
}

macro_rules! property{(
    $(#[$meta:meta])*
    :$iri:expr,
    $trait_name:ident,
    $function:ident,
    $range:path,
    $pos:expr) => {

$(#[$meta])*
pub trait $trait_name: resource::ResourceBase where
          Self: std::marker::Sized
{
    #[doc=$iri]
    fn property_iri() -> &'static str {
        $iri
    }
    fn property_iri_ptr(&self) -> Option<<Self::Graph as graph::Graph>::PredicatePtr> {
        self.graph().class_iri($pos).map(|i|i.clone())
    }
    $(#[$meta])*
    #[doc=$iri]
    fn $function<G>(&self) -> resource::ObjectIter<$range> where $range: resource::ResourceBase<Graph = Self::Graph>, G:graph::Graph {
        resource::ResourceBase::predicate(self, self.property_iri_ptr())
    }
}
    }
}

macro_rules! class{(
    $(#[$meta:meta])*
    :$iri:expr,
    $name:ident,
    $pos:expr) => {

$(#[$meta])*
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
    #[doc=$iri]
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
impl<'a, G> resource::ResourceBase for $name<G>
    where G: graph::Graph
{
    type Graph = G;
    type SubjectIter = resource::SubjectIter<Self>;
    fn new(resource: G::ObjectPtr,
            graph: &std::rc::Rc<ontology_adapter::OntologyAdapter<G>>) -> Self {
        $name {
            resource: resource,
            graph: graph.clone(),
            phantom: std::marker::PhantomData,
        }
    }
    fn iter(graph: &std::rc::Rc<ontology_adapter::OntologyAdapter<G>>)
            -> resource::SubjectIter<Self> {
        let a = graph.class_iri(0);
        let c = graph.class_iri($pos);
        let iter = match (a, c) {
            (Some(a),Some(c)) => graph.iter_o_p(graph.predicate_to_object(c.clone()), a.clone()),
            _ => graph.empty_ops_range()
        };
        resource::SubjectIter {
            graph: graph.clone(),
            iter: iter,
        }
    }
    fn this(&self) -> &G::ObjectPtr {
        &self.resource
    }
    fn graph(&self) -> &ontology_adapter::OntologyAdapter<G> {
        &self.graph
    }

    fn predicate<R>(&self, predicate: Option<G::PredicatePtr>)
            -> resource::ObjectIter<R>
        where R: resource::ResourceBase<Graph = G>
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

class!(:"http://www.w3.org/2000/01/rdf-schema#Class", Class, 1);
class!(:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Property",
       Property,
       2);
class!(:"http://www.w3.org/2000/01/rdf-schema#Literal", Literal, 3);
property!(:"http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, Literal<G>, 4);
property!(:"http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, Class<G>, 5);
property!(:"http://www.w3.org/2000/01/rdf-schema#range", Range, range, Class<G>, 6);
property!(:"http://www.w3.org/2000/01/rdf-schema#subClassOf",
    SubClassOf, sub_class_of, Class<G>, 7);

impl<G> SubClassOf for Class<G> where G: graph::Graph {}
impl<G> Comment for Class<G> where G: graph::Graph {}
impl<G> Domain for Property<G> where G: graph::Graph {}
impl<G> Range for Property<G> where G: graph::Graph {}
impl<G> Comment for Property<G> where G: graph::Graph {}
