//! Traits for RDF graphs.
//!
//! RDF graphs consist of triples: a subject, a predicate, and an object.
//! Triples are also called statements: A triple says something about something.
//! For example:
//!
//! ```turtle
//! @prefix nco: <http://www.semanticdesktop.org/ontologies/2007/03/22/nco#> .
//! @prefix nfo: <http://www.semanticdesktop.org/ontologies/2007/03/22/nfo#> .
//! @prefix :    <http://example.org/> .
//!
//! <hello_world.rs>  a                        nfo:SourceCode .
//! <hello_world.rs>  nfo:programmingLanguage  "Rust"@en .
//! <hello_world.rs>  nco:creator              :alice .
//! :alice            nco:hasName              _:alice_name .
//! _:alice_name      nco:nickname             "alist" .
//! ```
//!
//! This small graph states a few things about a resource `hello_world.rs`.
//! `hello_world.rs` is a relative IRI. The first triple states that
//! `hello_world.rs` is source code. The second statement says that it is written
//! in Rust. The last three triple identify the creator by nickname.
//!
//! Basically a graph is a table with three columns: subject, predicate, object. In
//! this example the subjects are, in short form, `<hello_world.rs>`, `:alice` and
//! `_:alice_name`. The first two are IRIs. They expand to full IRIs when parsed:
//! `file:///home/alice/src/hello/hello_world.rs` and `http://example.org/alice`.
//!
//! These IRIs uniquely identify a *resource*, in this case the file
//! `hello_world.rs` and the person Alice.
//!
//! One of the subjects in the example, `_:alice_name` is not an IRI but a blank
//! node. Blank nodes are used for subjects and objects for which no identifier is
//! known or needed.
//!
//! The second column contains the predicates. Predicates are always IRIs.
//! The predicate describes a relation between a subject and an object.
//!
//! The third column contains the objects. An object can be a blank node, an IRI or
//! a literal. The value of a literal is written in quotes. A literal can have a
//! a datatype or a language. In the example, the literal value `Rust` is
//! english (`@en`).
//!
//! The format of RDF looks very verbose like this. The form of this example is
//! [Turtle](https://www.w3.org/TR/turtle/).
//! There are also binary formats for RDF graphs such as
//! [HDT](http://www.rdfhdt.org/what-is-hdt/).
//!
//! http://www.w3.org/TR/rdf-concepts
//!
//!
//! This module contains traits that correspond to concepts in RDF.
//!
//! BlankNodePtr, IRIPtr and LiteralPtr are pointers into the graph. Together they
//! form a Triple. The subject of a triple can be a blank node or an IRI. This is
//! modelled by the enum BlankNodeOrIRI. A predicate can only be an IRI. An object
//! can take any kind of resource so the enum Resource encapsulates BlankNodePtr,
//! IRIPtr and LiteralPtr.
//!
//! In this module, graphs are immutable, but an new graph can be created by
//! extending another graph (TODO).
//!
//!

use std::cmp::Ordering;
use std::marker::PhantomData;
use iter::sorted_iterator::SortedIterator;
use constants;

/// Instances of this trait point to a blank node in a graph.
///
/// Different graph implementations can represent blank nodes in a different
/// way. The essense of the blank node is the same in all graphs and is capured
/// by this trait.
///
/// Blank nodes are tied to their graph. Their lifetime `'g` is the same as the
/// lifetime of the graph to which they belong.
pub trait BlankNodePtr<'g> {
    /// Convert this `BlankNodePtr` to a `BlankNodeOrIRI`.
    ///
    /// This is a convenience wrapper around the constructor for the enum
    /// `BlankNodeOrIRI`.
    ///
    /// ```
    /// # use rdfio::graphs::tel;
    /// # use rdfio::graph::*;
    /// #
    /// # let mut creator = tel::GraphCreator::with_capacity(0);
    /// # let ok = creator.create_iri(&"ok");
    /// let blank_node = creator.create_blank_node();
    /// let blank_node_or_iri = blank_node.to_blank_node_or_iri();
    /// assert_eq!(Some(&blank_node), blank_node_or_iri.as_blank_node());
    /// # let typed: BlankNodeOrIRI<_, &str> = blank_node_or_iri;
    /// # creator.add_blank_blank(&blank_node, &ok, &blank_node);
    /// # let graph: tel::Graph64 = creator.collect();
    /// ```
    fn to_blank_node_or_iri<I>(&self) -> BlankNodeOrIRI<'g, Self, I>
        where Self: Clone,
              I: IRIPtr<'g>
    {
        BlankNodeOrIRI::BlankNode(self.clone(), PhantomData)
    }
    /// Convert this `BlankNodePtr` to a `Resource`.
    ///
    /// This is a convenience wrapper around the constructor for the enum
    /// `Resource`.
    ///
    /// ```
    /// # use rdfio::graphs::tel;
    /// # use rdfio::graph::*;
    /// #
    /// # let mut creator = tel::GraphCreator::with_capacity(0);
    /// # let iri = creator.create_iri(&"");
    /// # const XSD_STRING: &'static str = "http://www.w3.org/2001/XMLSchema#string";
    /// # let xsd_string = creator.create_datatype(&XSD_STRING);
    /// # let hello = creator.create_literal_datatype(&"hello", &xsd_string);
    /// # creator.add_iri_literal(&iri, &iri, &hello);
    /// # let graph: tel::Graph64 = creator.collect();
    /// let literal = graph.find_literal("hello", XSD_STRING, None).unwrap();
    /// let resource = literal.to_resource();
    /// assert_eq!(Some(&literal), resource.as_literal());
    /// # let resource_option = graph.iter().next().map(|t|t.object());
    /// # resource_option.or(Some(resource));
    /// ```
    fn to_resource<I, L>(&self) -> Resource<'g, Self, I, L>
        where Self: Clone,
              I: IRIPtr<'g>,
              L: LiteralPtr<'g>
    {
        Resource::BlankNode(self.clone(), PhantomData)
    }
}
/// A trait for a pointers to IRI in graphs.
///
/// Like blank nodes and literals, IRIs are tied to the graph to which they
/// belong.
pub trait IRIPtr<'g> {
    /// Get a string representation of the IRI.
    fn as_str(&self) -> &str;
    fn to_blank_node_or_iri<B>(&self) -> BlankNodeOrIRI<'g, B, Self>
        where Self: Clone,
              B: BlankNodePtr<'g>
    {
        BlankNodeOrIRI::IRI(self.clone())
    }
    fn to_resource<B, L>(&self) -> Resource<'g, B, Self, L>
        where Self: Clone,
              B: BlankNodePtr<'g>,
              L: LiteralPtr<'g>
    {
        Resource::IRI(self.clone())
    }
}
impl<'g> PartialEq for IRIPtr<'g> {
    fn eq(&self, other: &IRIPtr<'g>) -> bool {
        self.as_str() == other.as_str()
    }
}
impl<'g> Eq for IRIPtr<'g> {}
impl<'g> PartialOrd for IRIPtr<'g> {
    fn partial_cmp(&self, other: &IRIPtr<'g>) -> Option<Ordering> {
        Some(self.as_str().cmp(other.as_str()))
    }
}
impl<'g> Ord for IRIPtr<'g> {
    fn cmp(&self, other: &IRIPtr<'g>) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}
pub trait LiteralPtr<'g> {
    fn as_str(&self) -> &str;
    fn datatype(&self) -> &str;
    fn language(&self) -> Option<&str>;
    fn to_resource<B, I>(&self) -> Resource<'g, B, I, Self>
        where Self: Clone,
              B: BlankNodePtr<'g>,
              I: IRIPtr<'g>
    {
        Resource::Literal(self.clone())
    }
}
impl<'g> PartialEq for LiteralPtr<'g> {
    fn eq(&self, other: &LiteralPtr<'g>) -> bool {
        self.as_str() == other.as_str() && self.datatype() == other.datatype() &&
        self.language() == other.language()
    }
}
impl<'g> Eq for LiteralPtr<'g> {}
impl<'g> PartialOrd for LiteralPtr<'g> {
    fn partial_cmp(&self, other: &LiteralPtr<'g>) -> Option<Ordering> {
        let mut cmp = self.as_str().cmp(&other.as_str());
        if cmp == Ordering::Equal {
            cmp = self.datatype().cmp(&other.datatype())
        }
        if cmp == Ordering::Equal {
            cmp = self.language().cmp(&other.language())
        }
        Some(cmp)
    }
}
impl<'g> Ord for LiteralPtr<'g> {
    fn cmp(&self, other: &LiteralPtr<'g>) -> Ordering {
        let mut cmp = self.as_str().cmp(&other.as_str());
        if cmp == Ordering::Equal {
            cmp = self.datatype().cmp(&other.datatype())
        }
        if cmp == Ordering::Equal {
            cmp = self.language().cmp(&other.language())
        }
        cmp
    }
}

#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum BlankNodeOrIRI<'g, B: 'g, I: 'g>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    BlankNode(B, PhantomData<&'g u8>),
    IRI(I),
}
impl<'g, B, I> BlankNodeOrIRI<'g, B, I>
    where B: BlankNodePtr<'g> + Clone,
          I: IRIPtr<'g> + Clone
{
    pub fn as_blank_node(&self) -> Option<&B> {
        match self {
            &BlankNodeOrIRI::BlankNode(ref b, _) => Some(b),
            _ => None,
        }
    }
    pub fn as_iri(&self) -> Option<&I> {
        match self {
            &BlankNodeOrIRI::IRI(ref i) => Some(i),
            _ => None,
        }
    }
    pub fn to_resource<L>(&self) -> Resource<'g, B, I, L>
        where Self: Clone,
              L: LiteralPtr<'g>
    {
        match self {
            &BlankNodeOrIRI::BlankNode(ref b, _) => Resource::BlankNode(b.clone(), PhantomData),
            &BlankNodeOrIRI::IRI(ref i) => Resource::IRI(i.clone()),
        }
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum IRIOrLiteral<'g, I: 'g, L: 'g>
    where I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    IRI(I, PhantomData<&'g u8>),
    Literal(L),
}
impl<'g, I, L> IRIOrLiteral<'g, I, L>
    where I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    pub fn as_iri(&self) -> Option<&I> {
        match self {
            &IRIOrLiteral::IRI(ref t, _) => Some(t),
            _ => None,
        }
    }
    pub fn as_literal(&self) -> Option<&L> {
        match self {
            &IRIOrLiteral::Literal(ref t) => Some(t),
            _ => None,
        }
    }
    pub fn to_resource<B>(&self) -> Resource<'g, B, I, L>
        where B: BlankNodePtr<'g>,
              I: Clone,
              L: Clone
    {
        match self {
            &IRIOrLiteral::IRI(ref b, _) => Resource::IRI(b.clone()),
            &IRIOrLiteral::Literal(ref l) => Resource::Literal(l.clone()),
        }
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum Resource<'g, B: 'g, I: 'g, L: 'g>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    BlankNode(B, PhantomData<&'g u8>),
    IRI(I),
    Literal(L),
}
impl<'g, B, I, L> Resource<'g, B, I, L>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    pub fn as_blank_node(&self) -> Option<&B> {
        match self {
            &Resource::BlankNode(ref b, _) => Some(b),
            _ => None,
        }
    }
    pub fn as_iri(&self) -> Option<&I> {
        match self {
            &Resource::IRI(ref t) => Some(t),
            _ => None,
        }
    }
    pub fn as_literal(&self) -> Option<&L> {
        match self {
            &Resource::Literal(ref t) => Some(t),
            _ => None,
        }
    }
    pub fn to_blank_node_or_iri(&self) -> Option<BlankNodeOrIRI<'g, B, I>>
        where B: Clone,
              I: Clone
    {
        match self {
            &Resource::BlankNode(ref b, _) => {
                Some(BlankNodeOrIRI::BlankNode(b.clone(), PhantomData))
            }
            &Resource::IRI(ref i) => Some(BlankNodeOrIRI::IRI(i.clone())),
            _ => None,
        }
    }
}

pub trait Triple<'g, B, I, L>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    fn subject(&self) -> BlankNodeOrIRI<'g, B, I>;
    fn predicate(&self) -> I;
    fn object(&self) -> Resource<'g, B, I, L>;
}

pub enum WriterBlankNodeOrIRI<'g, W>
    where W: GraphWriter<'g>
{
    BlankNode(W::BlankNode, PhantomData<&'g u8>),
    IRI(W::IRI),
}
pub enum WriterResource<'g, W>
    where W: GraphWriter<'g>
{
    BlankNode(W::BlankNode, PhantomData<&'g u8>),
    IRI(W::IRI),
    Literal(W::Literal),
}

pub trait GraphWriter<'g> {
    type BlankNode: Clone;
    type IRI: Clone;
    type Literal;
    type Datatype: Clone;
    type Language;
    type Graph: Graph<'g>;

    fn create_blank_node(&mut self) -> Self::BlankNode;
    fn create_iri<'a, I: 'a>(&mut self, &I) -> Self::IRI where I: IRIPtr<'a>;
    fn create_literal<'a, L: 'a>(&mut self, &L) -> Self::Literal where L: LiteralPtr<'a>;
    fn create_datatype(&mut self, &str) -> Self::Datatype;
    fn create_language(&mut self, &str) -> Self::Language;
    fn create_literal_datatype<'a>(&mut self,
                                   value: &str,
                                   datatype: &Self::Datatype)
                                   -> Self::Literal;
    fn create_literal_language<'a>(&mut self,
                                   value: &str,
                                   language: &Self::Language)
                                   -> Self::Literal;

    fn add(&mut self,
           subject: &WriterBlankNodeOrIRI<'g, Self>,
           predicate: &Self::IRI,
           object: &WriterResource<'g, Self>)
        where Self: Sized
    {
        match subject {
            &WriterBlankNodeOrIRI::BlankNode(ref subject, _) => {
                match object {
                    &WriterResource::BlankNode(ref object, _) => {
                        self.add_blank_blank(subject, predicate, object);
                    }
                    &WriterResource::IRI(ref object) => {
                        GraphWriter::add_blank_iri(self, subject, predicate, object);
                    }
                    &WriterResource::Literal(ref object) => {
                        GraphWriter::add_blank_literal(self, subject, predicate, object);
                    }
                }
            }
            &WriterBlankNodeOrIRI::IRI(ref subject) => {
                match object {
                    &WriterResource::BlankNode(ref object, _) => {
                        GraphWriter::add_iri_blank(self, subject, predicate, object);
                    }
                    &WriterResource::IRI(ref object) => {
                        GraphWriter::add_iri_iri(self, subject, predicate, object);
                    }
                    &WriterResource::Literal(ref object) => {
                        GraphWriter::add_iri_literal(self, subject, predicate, object);
                    }
                }
            }
        }
    }

    fn add_blank_blank(&mut self,
                       subject: &Self::BlankNode,
                       predicate: &Self::IRI,
                       object: &Self::BlankNode);
    fn add_blank_iri(&mut self,
                     subject: &Self::BlankNode,
                     predicate: &Self::IRI,
                     object: &Self::IRI);
    fn add_blank_literal(&mut self,
                         subject: &Self::BlankNode,
                         predicate: &Self::IRI,
                         bject: &Self::Literal);
    fn add_iri_blank(&mut self,
                     subject: &Self::IRI,
                     predicate: &Self::IRI,
                     object: &Self::BlankNode);
    fn add_iri_iri(&mut self, subject: &Self::IRI, predicate: &Self::IRI, object: &Self::IRI);
    fn add_iri_literal(&mut self,
                       subject: &Self::IRI,
                       predicate: &Self::IRI,
                       object: &Self::Literal);
    fn collect(self) -> Self::Graph;
}

pub trait Graph<'g> {
    type BlankNodePtr: BlankNodePtr<'g> + Ord + Clone;
    type IRIPtr: IRIPtr<'g> + Ord + Clone;
    type LiteralPtr: LiteralPtr<'g> + Ord + Clone;
    type SPOTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord + Clone;
    type SPOIter: SortedIterator<Item = Self::SPOTriple>;
    type SPORangeIter: SortedIterator<Item = Self::SPOTriple>;
    type OPSTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord + Clone;
    type OPSRangeIter: SortedIterator<Item = Self::OPSTriple>;
    fn iter(&'g self) -> Self::SPOIter;

    fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr>;
    fn find_literal<'a>(&'g self,
                        literal: &'a str,
                        datatype: &'a str,
                        language: Option<&'a str>)
                        -> Option<Self::LiteralPtr>;

    fn iter_s_p(&'g self,
                subject: BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                predicate: Self::IRIPtr)
                -> Self::SPORangeIter;
    fn iter_o_p(&'g self,
                object: Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
                predicate: Self::IRIPtr)
                -> Self::OPSRangeIter;

    /// iterator that returns no results
    fn empty_spo_range(&'g self) -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter;
}

impl<'g> IRIPtr<'g> for &'g str {
    fn as_str(&self) -> &str {
        *self
    }
}
impl<'g> IRIPtr<'g> for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}
impl<'g> LiteralPtr<'g> for &'g str {
    fn as_str(&self) -> &str {
        *self
    }
    fn datatype(&self) -> &str {
        constants::XSD_STRING
    }
    fn language(&self) -> Option<&str> {
        None
    }
}
impl<'g> LiteralPtr<'g> for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
    fn datatype(&self) -> &str {
        constants::XSD_STRING
    }
    fn language(&self) -> Option<&str> {
        None
    }
}
