use graph;
use graph::Triple;
use ontology_adapter;
use std;
use resource;
use iter;

macro_rules!
rb{($p:ident) =>
    (<<Self as resource::ResourceBase<'g>>::Graph as graph::Graph<'g>>::$p)
}
macro_rules!
resource{() =>
    (graph::Resource<'g, rb!(BlankNodePtr), rb!(IRIPtr), rb!(LiteralPtr)>)
}
macro_rules!
adapter{() =>
    (ontology_adapter::OntologyAdapter<'g, Self::Graph>)
}
macro_rules!
g{($p:ident) =>
    (<G as graph::Graph<'g>>::$p)
}
macro_rules!
g_resource{() =>
    (graph::Resource<'g, g!(BlankNodePtr), g!(IRIPtr), g!(LiteralPtr)>)
}


pub trait ResourceBase<'g>: Clone + Ord {
    type Graph: graph::Graph<'g>;
    type SubjectIter: Iterator<Item = Self> + iter::SortedIterator;
    fn new(this: resource!(), graph: &'g adapter!()) -> Self;
    fn iter(graph: &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>) -> Self::SubjectIter;
    fn this(&self) -> &resource!();
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g, Self::Graph>;
    fn predicate<R>(&self, predicate: Option<&rb!(IRIPtr)>) -> ObjectIter<'g,R>
        where R: ResourceBase<'g, Graph = Self::Graph>,
              <Self as ResourceBase<'g>>::Graph: 'g,
              Self: 'g
    {
        let graph = self.graph();
        let iter = match predicate {
            None => graph.empty_spo_range(),
            Some(ref predicate) => match self.this().to_blank_node_or_iri() {
                Some(subject) => graph.iter_s_p(subject, (*predicate).clone()),
                None => graph.empty_spo_range(),
            }
        };
        ObjectIter {
            graph: graph,
            iter: iter,
        }
    }
    fn iri(&self) -> Option<IRI<'g, Self>> {
        match self.this() {
            &graph::Resource::IRI(_) => Some(IRI {
                resource: self.clone(),
                phantom: std::marker::PhantomData,
            }),
            _ => None
        }
    }
}

// a wrapper around 'ResourceBase' that guarantees that the resource
// is an iri and not a blank node or a literal
pub struct IRI<'g, R: 'g>
    where R: ResourceBase<'g>
{
    resource: R,
    phantom: std::marker::PhantomData<&'g u8>,
}

impl<'g, R> IRI<'g, R>
    where R: ResourceBase<'g>
{
    pub fn this(&self) -> &rb!(IRIPtr) {
        match self.resource.this().as_iri() {
            Some(iri) => iri,
            _ => panic!("unreachable")
        }
    }
    pub fn as_str(&self) -> &str {
        use graph::IRIPtr;
        match self.resource.this() {
            &graph::Resource::IRI(ref iri) => iri.as_str(),
            _ => panic!("unreachable")
        }
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
    fn new(this: resource!(),
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
    fn this(&self) -> &resource!() {
        self.resource.this()
    }
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g, Self::Graph> {
        self.resource.graph()
    }
    fn predicate<O>(&self, predicate: Option<&rb!(IRIPtr)>) -> ObjectIter<'g,O>
        where O: ResourceBase<'g,Graph = Self::Graph>
    {
        self.resource.predicate(predicate)
    }
}

pub struct ObjectIter<'g, R: 'g>
    where R: ResourceBase<'g>
{
    graph: &'g ontology_adapter::OntologyAdapter<'g, R::Graph>,
    iter: <R::Graph as graph::Graph<'g>>::SPORangeIter,
}

impl<'g, R> Iterator for ObjectIter<'g, R>
    where R: ResourceBase<'g>
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => Some(R::new(triple.object(), &self.graph)),
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
                let s = triple.subject();
                let o = s.to_resource();
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
    $(#[$meta])*
    #[doc=$iri]
    fn $function<G>(&self) -> resource::ObjectIter<'g,$range>
        where $range: resource::ResourceBase<'g,Graph = Self::Graph>,
              G: graph::Graph<'g>,
              <Self as resource::ResourceBase<'g>>::Graph: 'g,
              Self: 'g
    {
        resource::ResourceBase::predicate(self, self.graph().preloaded_iri($pos))
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
    resource: g_resource!(),
    graph: &'g ontology_adapter::OntologyAdapter<'g,G>,
}
impl<'g, G> $name<'g,G>
    where G: graph::Graph<'g>
{
    #[doc=$iri]
    pub fn class_iri() -> &'static str {
        $iri
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
    type SubjectIter = resource::SubjectIter<'g, Self>;
    fn new(resource: resource!(),
            graph: &'g ontology_adapter::OntologyAdapter<'g,G>) -> Self {
        $name {
            resource: resource,
            graph: graph,
        }
    }
    fn iter(graph: &'g ontology_adapter::OntologyAdapter<'g,G>)
            -> resource::SubjectIter<'g,Self> {
        use graph::IRIPtr;
        let rdf_type = graph.preloaded_iri(0);
        let class = graph.preloaded_iri($pos);
        let iter = match (rdf_type, class) {
            (Some(rdf_type),Some(class)) => graph.iter_o_p(class.to_resource(), rdf_type.clone()),
            _ => graph.empty_ops_range()
        };
        resource::SubjectIter {
            graph: graph,
            iter: iter,
        }
    }
    fn this(&self) -> &resource!() {
        &self.resource
    }
    fn graph(&self) -> &'g ontology_adapter::OntologyAdapter<'g,G> {
        &self.graph
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

#[cfg(test)]
mod tests {
    use std;
    use graph;
    use resource;
    use ontology_adapter;
    use resource::{ResourceBase, IRI};
    use graph::GraphWriter;
    use graphs::tel;
    use constants;

    const RDF_PROPERTY: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
    const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
    const RDFS_LITERAL: &'static str = "http://www.w3.org/2000/01/rdf-schema#Literal";
    const RDFS_COMMENT: &'static str = "http://www.w3.org/2000/01/rdf-schema#comment";
    const RDFS_DOMAIN: &'static str = "http://www.w3.org/2000/01/rdf-schema#domain";
    const RDFS_RANGE: &'static str = "http://www.w3.org/2000/01/rdf-schema#range";
    const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOF";

    pub fn adapter<'g, G: 'g>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
        where G: graph::Graph<'g>
    {
        let mut iris = Vec::with_capacity(7);
        iris.push(graph.find_iri(constants::RDF_TYPE));
        iris.push(graph.find_iri(RDFS_CLASS));
        iris.push(graph.find_iri(RDF_PROPERTY));
        iris.push(graph.find_iri(RDFS_LITERAL));
        iris.push(graph.find_iri(RDFS_COMMENT));
        iris.push(graph.find_iri(RDFS_DOMAIN));
        iris.push(graph.find_iri(RDFS_RANGE));
        iris.push(graph.find_iri(RDFS_SUB_CLASS_OF));
        ontology_adapter::OntologyAdapter::new(graph, iris)
    }

    class!(:"http://www.w3.org/2000/01/rdf-schema#Class", Class, 1);
    class!(:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property, 2);
    class!(:"http://www.w3.org/2000/01/rdf-schema#Literal", Literal, 3);
    property!(:"http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, Literal<'g,G>, 4);
    property!(:"http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, Class<'g,G>, 5);
    property!(:"http://www.w3.org/2000/01/rdf-schema#range", Range, range, Class<'g,G>, 6);
    property!(:"http://www.w3.org/2000/01/rdf-schema#subClassOf", SubClassOf, sub_class_of, Class<'g,G>, 7);

    impl<'g,G> SubClassOf<'g> for Class<'g,G> where G: graph::Graph<'g> {}
    impl<'g,G> Comment<'g> for Class<'g,G> where G: graph::Graph<'g> {}
    impl<'g,G> Domain<'g> for Property<'g,G> where G: graph::Graph<'g> {}
    impl<'g,G> Range<'g> for Property<'g,G> where G: graph::Graph<'g> {}
    impl<'g,G> Comment<'g> for Property<'g,G> where G: graph::Graph<'g> {}
    impl<'g,G> Comment<'g> for Literal<'g,G> where G: graph::Graph<'g> {}

    #[test]
    fn instantiate_ontology_classes() {
        let bnc = tel::BlankNodeCreator::new();
        let creator = tel::GraphCreator::with_capacity(1000, &bnc);
        let graph: tel::Graph64 = creator.collect();
        let ontology = adapter(&graph);
        for class in IRI::<Class<_>>::iter(&ontology) {
            class.comment();
        }
        for property in IRI::<Property<_>>::iter(&ontology) {
            property.comment();
        }
        for literal in IRI::<Literal<_>>::iter(&ontology) {
            literal.comment();
        }
    }
    #[test]
    fn test_class_iri() {
        assert_eq!(Class::<tel::Graph64>::class_iri(), RDFS_CLASS);
        assert_eq!(Property::<tel::Graph64>::class_iri(), RDF_PROPERTY);
        assert_eq!(Literal::<tel::Graph64>::class_iri(), RDFS_LITERAL);
    }
}
