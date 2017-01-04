use graph;
use graph::Triple;
use std::marker::PhantomData;
use std::rc::Rc;


#[derive (Clone,Debug)]
pub enum Resource {
    IRI(Rc<String>),
    Literal(Rc<String>),
}

impl Resource {
    pub fn subject(&self) -> Option<graph::Subject> {
        match *self {
            Resource::IRI(ref iri) => Some(graph::Subject::IRI(iri.as_str())),
            _ => None,
        }
    }
    pub fn object(&self) -> Option<graph::Object> {
        match *self {
            Resource::IRI(ref iri) => Some(graph::Object::IRI(iri.as_str())),
            Resource::Literal(ref l) => {
                Some(graph::Object::Literal(graph::Literal {
                    lexical: l.as_str(),
                    datatype: "",
                    language: None,
                }))
            }
        }
    }
}

pub struct ObjectIter<G, R>
    where G: graph::Graph
{
    pub graph: Rc<G>,
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
            Some(triple) => {
                match triple.object() {
                    graph::Object::IRI(iri) => {
                        Some(R::new(&Resource::IRI(Rc::new(String::from(iri))), &self.graph))
                    }
                    graph::Object::Literal(l) => {
                        Some(R::new(&Resource::Literal(Rc::new(String::from(l.lexical))),
                                    &self.graph))
                    }
                    _ => None,
                }
            }
            None => None,
        }
    }
}

pub trait ResourceBase<G>
    where G: graph::Graph
{
    fn new(iri: &Resource, graph: &Rc<G>) -> Self;
    fn this(&self) -> &Resource;
    fn graph(&self) -> &G;
    fn predicate<R>(&self, predicate: &str) -> ObjectIter<G, R> where R: ResourceBase<G>;
}

macro_rules! property{(
    $iri:expr,
    $trait_name:ident,
    $function:ident,
    $range:path) => {

pub trait $trait_name<G>: resource::ResourceBase<G>
    where G: graph::Graph
{
    fn property_iri() -> &'static str {
        $iri
    }
    fn $function(&self) -> resource::ObjectIter<G, $range> {
        resource::ResourceBase::predicate(self, Self::property_iri())
    }
}
    }
}

macro_rules! class{(
    $iri:expr,
    $name:ident) => {

pub struct $name<G>
    where G: graph::Graph
{
    resource: resource::Resource,
    graph: std::rc::Rc<G>,
}
impl<G> $name<G>
    where G: graph::Graph
{
    pub fn class_iri() -> &'static str {
        $iri
    }
}
impl<'a, G> resource::ResourceBase<G> for $name<G>
    where G: graph::Graph
{
    fn new(resource: &resource::Resource, graph: &std::rc::Rc<G>) -> Self {
        $name {
            resource: resource.clone(),
            graph: graph.clone(),
        }
    }
    fn this(&self) -> &resource::Resource {
        &self.resource
    }
    fn graph(&self) -> &G {
        &self.graph
    }

    fn predicate<R>(&self, predicate: &str) -> resource::ObjectIter<G, R>
        where R: resource::ResourceBase<G>
    {
        let iter = match self.this().subject() {
            Some(subject) => self.graph.iter_subject_predicate(&subject, predicate),
            None => self.graph.empty_spo_range(),
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
