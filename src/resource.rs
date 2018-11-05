//! Helper traits used in generated ontology code.
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]
use graph;
use graph::Triple;
use iter;
use ontology_adapter;
use resource;
use std;

macro_rules! rb {
    ($p:ident) => {
        <<Self as resource::ResourceBase<'g>>::Graph as graph::Graph<'g>>::$p
    };
}
macro_rules!
resource{() =>
    (graph::Resource<'g, rb!(BlankNodePtr), rb!(IRIPtr), rb!(LiteralPtr)>)
}
macro_rules!
adapter{() =>
    (ontology_adapter::OntologyAdapter<'g, Self::Graph>)
}
macro_rules! g {
    ($p:ident) => {
        <G as graph::Graph<'g>>::$p
    };
}
macro_rules!
g_resource{() =>
    (graph::Resource<'g, g!(BlankNodePtr), g!(IRIPtr), g!(LiteralPtr)>)
}

/// Base trait for all ontology traits.
pub trait ResourceBase<'g>: Clone + Ord {
    /// Type of the graph that on which this resource is mapped.
    type Graph: graph::Graph<'g>;
    #[doc(hidden)]
    type SubjectIter: Iterator<Item = Self> + iter::SortedIterator;
    /// Wrap a [`Resource`][graph] with an ontology class.
    /// And link a [`ResourceBase`][resource]
    fn new(this: resource!(), graph: &'g adapter!()) -> Self;
    /// Iterate over all instances of this class
    fn iter(graph: &'g adapter!()) -> Self::SubjectIter;
    /// The Resource that underlies this wrapper.
    fn this(&self) -> &resource!();
    /// The adapter that wraps the graph.
    fn adapter(&self) -> &'g adapter!();
    /// iterate over all the objects for this subject and the given predicate
    fn iter_objects<O>(&self, predicate: Option<&rb!(IRIPtr)>) -> ObjectIter<'g, O>
    where
        O: ResourceBase<'g, Graph = Self::Graph>,
        Self: 'g,
    {
        let adapter = self.adapter();
        let iter = match predicate {
            Some(predicate) => match self.this().to_blank_node_or_iri() {
                Some(subject) => adapter.iter_s_p(&subject, predicate),
                None => adapter.empty_spo_range(),
            },
            None => adapter.empty_spo_range(),
        };
        ObjectIter { adapter, iter }
    }
    /// Return this resource as an IRI, if it is an IRI.
    fn iri(&self) -> Option<IRI<'g, Self>> {
        match *self.this() {
            graph::Resource::IRI(_) => Some(IRI {
                resource: self.clone(),
                phantom: std::marker::PhantomData,
            }),
            _ => None,
        }
    }
}

/// A wrapper around `ResourceBase` that guarantees that the resource
/// is an IRI and not a blank node or a literal.
pub struct IRI<'g, R: 'g>
where
    R: ResourceBase<'g>,
{
    resource: R,
    phantom: std::marker::PhantomData<&'g u8>,
}

impl<'g, R> IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    /// Access the IRI that underlies this instance.
    pub fn this(&self) -> &rb!(IRIPtr) {
        match self.resource.this().as_iri() {
            Some(iri) => iri,
            _ => panic!("unreachable"),
        }
    }
    /// Access the IRI that underlies this instance as a string.
    pub fn as_str(&self) -> &str {
        use graph::IRIPtr;
        match *self.resource.this() {
            graph::Resource::IRI(ref iri) => iri.as_str(),
            _ => panic!("unreachable"),
        }
    }
}
impl<'g, R: ?Sized> std::ops::Deref for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    type Target = R;

    fn deref(&self) -> &R {
        &self.resource
    }
}

impl<'g, R> Clone for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    fn clone(&self) -> Self {
        IRI {
            resource: self.resource.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'g, R> PartialEq for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    fn eq(&self, rhs: &IRI<'g, R>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<'g, R> Eq for IRI<'g, R> where R: ResourceBase<'g> {}

impl<'g, R> PartialOrd for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.resource.partial_cmp(&other.resource)
    }
}
impl<'g, R> Ord for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource.cmp(&other.resource)
    }
}

impl<'g, R: 'g> ResourceBase<'g> for IRI<'g, R>
where
    R: ResourceBase<'g>,
{
    type Graph = R::Graph;
    type SubjectIter = IRISubjectIter<'g, R>;
    fn new(this: resource!(), graph: &'g adapter!()) -> Self {
        IRI {
            resource: R::new(this, graph),
            phantom: std::marker::PhantomData,
        }
    }
    fn iter(graph: &'g adapter!()) -> Self::SubjectIter {
        IRISubjectIter {
            iter: R::iter(graph),
        }
    }
    fn this(&self) -> &resource!() {
        self.resource.this()
    }
    fn adapter(&self) -> &'g adapter!() {
        self.resource.adapter()
    }
    fn iter_objects<O>(&self, predicate: Option<&rb!(IRIPtr)>) -> ObjectIter<'g, O>
    where
        O: ResourceBase<'g, Graph = Self::Graph>,
    {
        self.resource.iter_objects(predicate)
    }
}

/// Iterate over all objects for the given subject and predicate.
pub struct ObjectIter<'g, R: 'g>
where
    R: ResourceBase<'g>,
{
    adapter: &'g ontology_adapter::OntologyAdapter<'g, R::Graph>,
    iter: <R::Graph as graph::Graph<'g>>::SPORangeIter,
}

impl<'g, R> Iterator for ObjectIter<'g, R>
where
    R: ResourceBase<'g>,
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => Some(R::new(triple.object(), self.adapter)),
            None => None,
        }
    }
}

impl<'g, R> iter::SortedIterator for ObjectIter<'g, R> where R: ResourceBase<'g> {}

/// Iterate over all subjects for the given object and predicate.
pub struct SubjectIter<'g, R: 'g>
where
    R: ResourceBase<'g>,
{
    #[doc(hidden)]
    pub adapter: &'g ontology_adapter::OntologyAdapter<'g, R::Graph>,
    #[doc(hidden)]
    pub iter: <R::Graph as graph::Graph<'g>>::OPSRangeIter,
}

impl<'g, R> Iterator for SubjectIter<'g, R>
where
    R: ResourceBase<'g>,
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(triple) => {
                let s = triple.subject();
                let o = s.to_resource();
                Some(R::new(o, self.adapter))
            }
            None => None,
        }
    }
}

impl<'g, R> iter::SortedIterator for SubjectIter<'g, R> where R: ResourceBase<'g> {}

/// Iterate over all subjects that are an IRI for the given object and predicate.
/// In other words: no blank nodes are returned.
pub struct IRISubjectIter<'g, R>
where
    R: ResourceBase<'g>,
{
    iter: R::SubjectIter,
}

impl<'g, R: 'g> Iterator for IRISubjectIter<'g, R>
where
    R: ResourceBase<'g>,
{
    type Item = IRI<'g, R>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().filter_map(|r| r.iri()).next()
    }
}

impl<'g, R: 'g + ResourceBase<'g>> iter::SortedIterator for IRISubjectIter<'g, R> {}

#[macro_export]
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
    /// The IRI for the property.
    fn property_iri() -> &'static str {
        $iri
    }
    $(#[$meta])*
    /// The values for this property.
    fn $function<G>(&self) -> resource::ObjectIter<'g, $range>
        where $range: resource::ResourceBase<'g, Graph = Self::Graph>,
              G: graph::Graph<'g>,
              Self: 'g
    {
        resource::ResourceBase::iter_objects(self, self.adapter().preloaded_iri($pos))
    }
}
    }
}

#[macro_export]
macro_rules! class{(
    $(#[$meta:meta])*
    :$iri:expr,
    $name:ident,
    $pos:expr) => {

$(#[$meta])*
pub struct $name<'g, G: 'g>
    where G: graph::Graph<'g>
{
    resource: g_resource!(),
    adapter: &'g ontology_adapter::OntologyAdapter<'g, G>,
}
impl<'g, G> $name<'g, G>
    where G: graph::Graph<'g>
{
    /// Return the IRI for this class.
    pub fn class_iri() -> &'static str {
        $iri
    }
}
impl<'g, G> PartialEq for $name<'g, G>
    where G: graph::Graph<'g>
{
    fn eq(&self, rhs: &$name<'g, G>) -> bool {
        self.resource.eq(&rhs.resource)
    }
}
impl<'g, G> Eq for $name<'g, G> where G: graph::Graph<'g> {}

impl<'g, G> Clone for $name<'g, G>
    where G: graph::Graph<'g>
{
    fn clone(&self) -> Self {
        $name {
            resource: self.resource.clone(),
            adapter: self.adapter.clone(),
        }
    }
}
impl<'g, G> PartialOrd for $name<'g, G>
     where G: graph::Graph<'g>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.resource.partial_cmp(&other.resource)
    }
}
impl<'g, G> Ord for $name<'g, G> where G: graph::Graph<'g> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource.cmp(&other.resource)
    }
}
impl<'g, G: 'g> resource::ResourceBase<'g> for $name<'g, G>
    where G: graph::Graph<'g>
{
    type Graph = G;
    type SubjectIter = resource::SubjectIter<'g, Self>;
    fn new(resource: resource!(), adapter: &'g adapter!()) -> Self {
        $name {
            resource: resource,
            adapter: adapter,
        }
    }
    fn iter(adapter: &'g adapter!()) -> resource::SubjectIter<'g, Self> {
        use graph::IRIPtr;
        let rdf_type = adapter.preloaded_iri(0);
        let class = adapter.preloaded_iri($pos);
        let iter = match (rdf_type, class) {
            (Some(rdf_type),Some(class)) => adapter.iter_o_p(&class.to_resource(), rdf_type),
            _ => adapter.empty_ops_range()
        };
        resource::SubjectIter {
            adapter: adapter,
            iter: iter,
        }
    }
    fn this(&self) -> &resource!() {
        &self.resource
    }
    fn adapter(&self) -> &'g adapter!() {
        &self.adapter
    }
}
}}

#[cfg(test)]
mod tests {
    use graph;
    use graph::GraphWriter;
    use graphs::tel;
    use ontology::iri::{rdf, rdfs};
    use ontology_adapter;
    use resource;
    use resource::{ResourceBase, IRI};
    use std;

    pub fn adapter<'g, G: 'g>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
    where
        G: graph::Graph<'g>,
    {
        let mut iris = Vec::with_capacity(7);
        iris.push(graph.find_iri(rdf::TYPE));
        iris.push(graph.find_iri(rdfs::CLASS));
        iris.push(graph.find_iri(rdf::PROPERTY));
        iris.push(graph.find_iri(rdfs::LITERAL));
        iris.push(graph.find_iri(rdfs::COMMENT));
        iris.push(graph.find_iri(rdfs::DOMAIN));
        iris.push(graph.find_iri(rdfs::RANGE));
        iris.push(graph.find_iri(rdfs::SUB_CLASS_OF));
        ontology_adapter::OntologyAdapter::new(graph, iris)
    }

    class!(:rdfs::CLASS, Class, 1);
    class!(:rdf::PROPERTY, Property, 2);
    class!(:rdfs::LITERAL, Literal, 3);
    property!(:rdfs::COMMENT, Comment, comment, Literal<'g, G>, 4);
    property!(:rdfs::DOMAIN, Domain, domain, Class<'g, G>, 5);
    property!(:rdfs::RANGE, Range, range, Class<'g, G>, 6);
    property!(:rdfs::SUB_CLASS_OF, SubClassOf, sub_class_of, Class<'g, G>, 7);

    impl<'g, G> SubClassOf<'g> for Class<'g, G> where G: graph::Graph<'g> {}
    impl<'g, G> Comment<'g> for Class<'g, G> where G: graph::Graph<'g> {}
    impl<'g, G> Domain<'g> for Property<'g, G> where G: graph::Graph<'g> {}
    impl<'g, G> Range<'g> for Property<'g, G> where G: graph::Graph<'g> {}
    impl<'g, G> Comment<'g> for Property<'g, G> where G: graph::Graph<'g> {}
    impl<'g, G> Comment<'g> for Literal<'g, G> where G: graph::Graph<'g> {}

    #[test]
    fn instantiate_ontology_classes() {
        let creator = tel::GraphCreator::with_capacity(1000);
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
        assert_eq!(Class::<tel::Graph64>::class_iri(), rdfs::CLASS);
        assert_eq!(Property::<tel::Graph64>::class_iri(), rdf::PROPERTY);
        assert_eq!(Literal::<tel::Graph64>::class_iri(), rdfs::LITERAL);
    }
}
