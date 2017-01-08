use graph;
use graph::Triple;
use ontology_adapter;
use std;
use resource;
use iter;

pub trait ResourceBase<'g>: Eq + std::marker::Sized + Clone + Ord {
    type Graph: graph::Graph<'g>;
    type SubjectIter: Iterator<Item = Self> + iter::SortedIterator;
    fn new(this: <Self::Graph as graph::Graph<'g>>::ObjectPtr,
           graph: &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>)
           -> Self;
    fn iter(graph: &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>) -> Self::SubjectIter;
    fn this(&self) -> <Self::Graph as graph::Graph<'g>>::ObjectPtr;
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>;
    fn predicate<R>(&self,
                    predicate: Option<<Self::Graph as graph::Graph<'g>>::PredicatePtr>)
                    -> ObjectIter<'g,R>
        where R: ResourceBase<'g, Graph = Self::Graph>;
    fn iri(&self) -> Option<IRI<'g, Self>> {
        self.graph().object_to_predicate(self.this().clone()).map(|_| {
            IRI {
                resource: self.clone(),
                phantom: std::marker::PhantomData,
            }
        })
    }
}

// a wrapper around 'ResourceBase' that guarantees that the resource
// is an iri and not a blank node or a literal
pub struct IRI<'g, R: 'g>
    where R: ResourceBase<'g>
{
    resource: R,
    phantom: std::marker::PhantomData<&'g R>,
}

impl<'g, R> IRI<'g, R>
    where R: ResourceBase<'g>
{
    pub fn iri_(&self) -> &'g str {
        use graph::SubjectPtr;
        self.resource.this().iri().unwrap()
    }
}
impl<'g, R: ?Sized> std::ops::Deref for IRI<'g, R>
    where R: ResourceBase<'g>
{
    type Target = R;

    #[inline(always)]
    fn deref(&self) -> &R {
        &self.resource
    }
}

impl<'g, R> Clone for IRI<'g, R>
    where R: ResourceBase<'g>
{
    fn clone(&self) -> Self {
        IRI {
            resource: self.resource.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'g, R> PartialEq for IRI<'g, R>
    where R: ResourceBase<'g>
{
    fn eq(&self, rhs: &IRI<'g, R>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<'g, R> Eq for IRI<'g, R> where R: ResourceBase<'g> {}

impl<'g, R> PartialOrd for IRI<'g, R>
    where R: ResourceBase<'g>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.resource.partial_cmp(&other.resource)
    }
}
impl<'g, R> Ord for IRI<'g, R>
    where R: ResourceBase<'g>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource.cmp(&other.resource)
    }
}

impl<'g, R: 'g> ResourceBase<'g> for IRI<'g, R>
    where R: ResourceBase<'g>
{
    type Graph = R::Graph;
    type SubjectIter = IRISubjectIter<'g, R>;
    fn new(this: <Self::Graph as graph::Graph<'g>>::ObjectPtr,
           graph: &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>)
           -> Self {
        IRI {
            resource: R::new(this, graph),
            phantom: std::marker::PhantomData,
        }
    }
    fn iter(graph: &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>) -> Self::SubjectIter {
        IRISubjectIter { iter: R::iter(graph) }
    }
    fn this(&self) -> <<R as resource::ResourceBase<'g>>::Graph as graph::Graph<'g>>::ObjectPtr {
        self.resource.this()
    }
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g, Self::Graph> {
        self.resource.graph()
    }
    fn predicate<O>(&self,
                    predicate: Option<<Self::Graph as graph::Graph<'g>>::PredicatePtr>)
                    -> ObjectIter<'g,O>
        where O: ResourceBase<'g,Graph = Self::Graph>{
        self.resource.predicate(predicate)
    }
}

pub struct ObjectIter<'g, R: 'g>
    where R: ResourceBase<'g>
{
    pub graph: &'g ontology_adapter::OntologyAdapter<'g, R::Graph>,
    pub iter: <R::Graph as graph::Graph<'g>>::SPORangeIter,
}

impl<'g, R> Iterator for ObjectIter<'g, R>
    where R: ResourceBase<'g>
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => Some(R::new(triple.object_ptr(), &self.graph)),
            None => None,
        }
    }
}

impl<'g, R> iter::SortedIterator for ObjectIter<'g, R> where R: ResourceBase<'g> {}

pub struct SubjectIter<'g, R: 'g>
    where R: ResourceBase<'g>
{
    pub graph: &'g ontology_adapter::OntologyAdapter<'g, R::Graph>,
    pub iter: <R::Graph as graph::Graph<'g>>::OPSRangeIter,
}

impl<'g, R> Iterator for SubjectIter<'g, R>
    where R: ResourceBase<'g>
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

impl<'g, R> iter::SortedIterator for SubjectIter<'g, R> where R: ResourceBase<'g> {}

pub struct IRISubjectIter<'g, R>
    where R: ResourceBase<'g>
{
    iter: R::SubjectIter,
}

impl<'g, R: 'g> Iterator for IRISubjectIter<'g, R>
    where R: ResourceBase<'g>
{
    type Item = IRI<'g, R>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(r) = self.iter.next() {
            if let Some(iri) = r.iri() {
                return Some(iri);
            }
        }
        None
    }
}

impl<'g, R: 'g + ResourceBase<'g>> iter::SortedIterator for IRISubjectIter<'g, R> {}

macro_rules! property{(
    $(#[$meta:meta])*
    :$iri:expr,
    $trait_name:ident,
    $function:ident,
    $range:path,
    $pos:expr) => {

$(#[$meta])*
pub trait $trait_name<'g>: resource::ResourceBase<'g>
    where Self: std::marker::Sized
{
    #[doc=$iri]
    fn property_iri() -> &'static str {
        $iri
    }
    fn property_iri_ptr(&self) -> Option<<Self::Graph as graph::Graph<'g>>::PredicatePtr>
        where <Self as resource::ResourceBase<'g>>::Graph: 'g
    {
        self.graph().class_iri($pos).map(|i|i.clone())
    }
    $(#[$meta])*
    #[doc=$iri]
    fn $function<G>(&self) -> resource::ObjectIter<'g,$range>
        where $range: resource::ResourceBase<'g,Graph = Self::Graph>,
              G: graph::Graph<'g>,
              <Self as resource::ResourceBase<'g>>::Graph: 'g
    {
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
pub struct $name<'g,G:'g>
    where G: graph::Graph<'g>
{
    resource: G::ObjectPtr,
    graph: &'g ontology_adapter::OntologyAdapter<'g,G>,
}
impl<'g, G> $name<'g,G>
    where G: graph::Graph<'g>
{
    #[doc=$iri]
    pub fn class_iri() -> &'static str {
        $iri
    }
    pub fn class_iri_ptr(&self) -> Option<G::PredicatePtr> {
        self.graph.class_iri($pos).map(|i|i.clone())
    }
}
impl<'g, G> PartialEq for $name<'g,G>
    where G: graph::Graph<'g>
{
    fn eq(&self, rhs: &$name<'g,G>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<'g, G> Eq for $name<'g,G> where G: graph::Graph<'g> {}

impl<'g, G> Clone for $name<'g,G>
    where G: graph::Graph<'g>
{
    fn clone(&self) -> Self {
        $name {
            resource: self.resource.clone(),
            graph: self.graph.clone(),
        }
    }
}
impl<'g, G> PartialOrd for $name<'g,G>
     where G: graph::Graph<'g>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.resource.partial_cmp(&other.resource)
    }
}
impl<'g, G> Ord for $name<'g,G> where G: graph::Graph<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource.cmp(&other.resource)
    }
}

impl<'g, G:'g> resource::ResourceBase<'g> for $name<'g,G>
    where G: graph::Graph<'g>
{
    type Graph = G;
    type SubjectIter = resource::SubjectIter<'g,Self>;
    fn new(resource: G::ObjectPtr,
            graph: &'g ontology_adapter::OntologyAdapter<'g,G>) -> Self {
        $name {
            resource: resource,
            graph: graph,
        }
    }
    fn iter(graph: &'g ontology_adapter::OntologyAdapter<'g,G>)
            -> resource::SubjectIter<'g,Self> {
        let a = graph.class_iri(0);
        let c = graph.class_iri($pos);
        let iter = match (a, c) {
            (Some(a),Some(c)) => graph.iter_o_p(graph.predicate_to_object(c.clone()), a.clone()),
            _ => graph.empty_ops_range()
        };
        resource::SubjectIter {
            graph: graph,
            iter: iter,
        }
    }
    fn this(&self) -> G::ObjectPtr {
        self.resource.clone()
    }
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g,G> {
        &self.graph
    }

    fn predicate<R>(&self, predicate: Option<G::PredicatePtr>)
            -> resource::ObjectIter<'g,R>
        where R: resource::ResourceBase<'g,Graph = G>
    {
        let iter = match predicate {
            None => self.graph.empty_spo_range(),
            Some(ref predicate) => match self.graph.object_to_subject(self.this().clone()) {
                Some(ref subject) => self.graph.iter_s_p(subject.clone(), predicate.clone()),
                None => self.graph.empty_spo_range(),
            }
        };
        resource::ObjectIter {
            graph: self.graph,
            iter: iter,
        }
    }
}
}}

macro_rules! sub_class_of{(
    $subclass:ident,
    $class:ident) => {

impl<'g, G> From<$subclass<'g, G>> for $class<G> where G: graph::Graph<'g> {
    fn from(f: $subclass<'g, G>) -> Self {
        $class {
            iri: f.iri,
            graph: f.graph,
        }
    }
}

}}

pub fn adapter<'g, G: 'g>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
    where G: graph::Graph<'g>
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
// class!(:"http://www.w3.org/2000/01/rdf-schema#Class", Class, 1);
// class!(:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Property",
// Property,
// 2);
// class!(:"http://www.w3.org/2000/01/rdf-schema#Literal", Literal, 3);
// property!(:"http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, Literal<G>, 4);
// property!(:"http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, Class<G>, 5);
// property!(:"http://www.w3.org/2000/01/rdf-schema#range", Range, range, Class<G>, 6);
// property!(:"http://www.w3.org/2000/01/rdf-schema#subClassOf",
// SubClassOf, sub_class_of, Class<G>, 7);
//
// impl<'g,G> SubClassOf<'g> for Class<'g,G> where G: graph::Graph<'g> {}
// impl<'g,G> Comment<'g> for Class<'g,G> where G: graph::Graph<'g> {}
// impl<'g,G> Domain<'g> for Property<'g,G> where G: graph::Graph<'g> {}
// impl<'g,G> Range<'g> for Property<'g,G> where G: graph::Graph<'g> {}
// impl<'g,G> Comment<'g> for Property<'g,G> where G: graph::Graph<'g> {}
//
