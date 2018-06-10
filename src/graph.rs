//! The main module of this crate. It has traits for RDF graphs.
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
//! `BlankNodePtr`, `IRIPtr` and `LiteralPtr` are pointers into the graph. Together they
//! form a Triple. The subject of a triple can be a blank node or an IRI. This is
//! modelled by the enum `BlankNodeOrIRI`. A predicate can only be an IRI. An object
//! can take any kind of resource so the enum Resource encapsulates `BlankNodePtr`,
//! `IRIPtr` and `LiteralPtr`.
//!
//! In this module, graphs are immutable, but an new graph can be created by
//! extending another graph (TODO).
//!
//!

use constants;
use iter::SortedIterator;
use std::cmp::Ordering;
use std::marker::PhantomData;

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
    /// # use rome::graphs::tel;
    /// # use rome::graph::*;
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
    where
        Self: Clone,
        I: IRIPtr<'g>,
    {
        BlankNodeOrIRI::BlankNode(self.clone(), PhantomData)
    }
    /// Convert this `BlankNodePtr` to a `Resource`.
    ///
    /// This is a convenience wrapper around the constructor for the enum
    /// `Resource`.
    ///
    /// ```
    /// # use rome::graphs::tel;
    /// # use rome::graph::*;
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
    where
        Self: Clone,
        I: IRIPtr<'g>,
        L: LiteralPtr<'g>,
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
    /// Wrap the IRI in a BlankNodeOrIRI
    /// This is useful when using it as a subject in a triple.
    fn to_blank_node_or_iri<B>(&self) -> BlankNodeOrIRI<'g, B, Self>
    where
        Self: Clone,
        B: BlankNodePtr<'g>,
    {
        BlankNodeOrIRI::IRI(self.clone())
    }
    /// Wrap the IRI in a Resource
    /// This is useful when using it as an object in a triple.
    fn to_resource<B, L>(&self) -> Resource<'g, B, Self, L>
    where
        Self: Clone,
        B: BlankNodePtr<'g>,
        L: LiteralPtr<'g>,
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
/// A trait for a pointers to datatypes of literals in graphs.
///
/// Like literals, `DatatypePtrs` are tied to the graph to which they
/// belong. A datatype is an IRI, but a graph may use different pointers for
/// datatypes and IRIs.
pub trait DatatypePtr<'g> {
    /// Get the datatype as a string.
    fn as_str(&self) -> &str;
}
/// A trait for a pointers to literals in graphs.
///
/// The lifetime of iterals is tied to the graph to which they belong.
/// A literal always has a datatype. It has an optional language.
pub trait LiteralPtr<'g> {
    /// The type of pointer for the datatype of the literal.
    type DatatypePtr: DatatypePtr<'g> + PartialEq;
    /// Get the value of the literal (without datattype or language)
    fn as_str(&self) -> &str;
    /// Get the datatype of the literal.
    fn datatype(&self) -> Self::DatatypePtr;
    /// Get the datatype of the literal as a string.
    fn datatype_str(&self) -> &str;
    /// Get the language of the literal.
    fn language(&self) -> Option<&str>;
    /// Wrap the literal in a Resource.
    /// This is convenient when passing the literal as an object in a triple.
    fn to_resource<B, I>(&self) -> Resource<'g, B, I, Self>
    where
        Self: Clone,
        B: BlankNodePtr<'g>,
        I: IRIPtr<'g>,
    {
        Resource::Literal(self.clone())
    }
}

/// An enum that contains a blank node or an IRI
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum BlankNodeOrIRI<'g, B: 'g, I: 'g>
where
    B: BlankNodePtr<'g>,
    I: IRIPtr<'g>,
{
    /// This is a blank node.
    BlankNode(B, PhantomData<&'g u8>),
    /// This is an IRI.
    IRI(I),
}
impl<'g, B, I> BlankNodeOrIRI<'g, B, I>
where
    B: BlankNodePtr<'g> + Clone,
    I: IRIPtr<'g> + Clone,
{
    /// Is the BlankNodeOrIRI a blank node?
    pub fn is_blank_node(&self) -> bool {
        match *self {
            BlankNodeOrIRI::BlankNode(_, _) => true,
            _ => false,
        }
    }
    /// Is the BlankNodeOrIRI an IRI?
    pub fn is_iri(&self) -> bool {
        match *self {
            BlankNodeOrIRI::IRI(_) => true,
            _ => false,
        }
    }
    /// Cast BlankNodeOrIRI to a blank node, if applicable
    pub fn as_blank_node(&self) -> Option<&B> {
        match *self {
            BlankNodeOrIRI::BlankNode(ref b, _) => Some(b),
            _ => None,
        }
    }
    /// Cast BlankNodeOrIRI to an IRI, if applicable
    pub fn as_iri(&self) -> Option<&I> {
        match *self {
            BlankNodeOrIRI::IRI(ref i) => Some(i),
            _ => None,
        }
    }
    /// Cast BlankNodeOrIRI to a Resource
    pub fn to_resource<L>(&self) -> Resource<'g, B, I, L>
    where
        Self: Clone,
        L: LiteralPtr<'g>,
    {
        match *self {
            BlankNodeOrIRI::BlankNode(ref b, _) => Resource::BlankNode(b.clone(), PhantomData),
            BlankNodeOrIRI::IRI(ref i) => Resource::IRI(i.clone()),
        }
    }
}
/// A Resource is a blank node, an IRI or a literal.
///
/// Resources are used in the object position of a triple.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Resource<'g, B: 'g, I: 'g, L: 'g>
where
    B: BlankNodePtr<'g>,
    I: IRIPtr<'g>,
    L: LiteralPtr<'g>,
{
    /// This is a blank node.
    BlankNode(B, PhantomData<&'g u8>),
    /// This is an IRI.
    IRI(I),
    /// This is a literal.
    Literal(L),
}
impl<'g, B, I, L> Resource<'g, B, I, L>
where
    B: BlankNodePtr<'g>,
    I: IRIPtr<'g>,
    L: LiteralPtr<'g>,
{
    /// Is this a blank node?
    pub fn is_blank_node(&self) -> bool {
        match *self {
            Resource::BlankNode(_, _) => true,
            _ => false,
        }
    }
    /// Is this an IRI?
    pub fn is_iri(&self) -> bool {
        match *self {
            Resource::IRI(_) => true,
            _ => false,
        }
    }
    /// Is this a literal?
    pub fn is_literal(&self) -> bool {
        match *self {
            Resource::Literal(_) => true,
            _ => false,
        }
    }
    /// Cast Resource to a blank node, if applicable
    pub fn as_blank_node(&self) -> Option<&B> {
        match *self {
            Resource::BlankNode(ref b, _) => Some(b),
            _ => None,
        }
    }
    /// Cast Resource to an IRI, if applicable
    pub fn as_iri(&self) -> Option<&I> {
        match *self {
            Resource::IRI(ref t) => Some(t),
            _ => None,
        }
    }
    /// Cast Resource to a literal, if applicable
    pub fn as_literal(&self) -> Option<&L> {
        match *self {
            Resource::Literal(ref t) => Some(t),
            _ => None,
        }
    }
    /// Cast Resource to a BlankNodeOrIRI, if applicable
    pub fn to_blank_node_or_iri(&self) -> Option<BlankNodeOrIRI<'g, B, I>>
    where
        B: Clone,
        I: Clone,
    {
        match *self {
            Resource::BlankNode(ref b, _) => {
                Some(BlankNodeOrIRI::BlankNode(b.clone(), PhantomData))
            }
            Resource::IRI(ref i) => Some(BlankNodeOrIRI::IRI(i.clone())),
            _ => None,
        }
    }
}

/// Triples are fundamental to RDF.
///
/// Each triple has a subject, a predicate and an object.
pub trait Triple<'g, B, I, L>
where
    B: BlankNodePtr<'g>,
    I: IRIPtr<'g>,
    L: LiteralPtr<'g>,
{
    /// Get the subject of this triple.
    fn subject(&self) -> BlankNodeOrIRI<'g, B, I>;
    /// Get the predicate of this triple.
    fn predicate(&self) -> I;
    /// Get the object of this triple.
    fn object(&self) -> Resource<'g, B, I, L>;
}

/// `WriterBlankNodeOrIRI` is like `BlankNodeOrIRI` but for writing graphs.
pub enum WriterBlankNodeOrIRI<'g, W>
where
    W: GraphWriter<'g>,
{
    /// This is a blank node.
    BlankNode(W::BlankNode, PhantomData<&'g u8>),
    /// This is an IRI.
    IRI(W::IRI),
}
/// `WriterResource` is like `Resource` but for writing graphs.
pub enum WriterResource<'g, W>
where
    W: GraphWriter<'g>,
{
    /// This is a blank node.
    BlankNode(W::BlankNode, PhantomData<&'g u8>),
    /// This is an IRI.
    IRI(W::IRI),
    /// This is a literal.
    Literal(W::Literal),
}

/// translate from one graph to another
/// useful for inferencing
/// there can be a general implemenation as wel as an optimized one that's
/// used when extending a graph by inferencing from its own content
pub trait ResourceTranslator<'g> {
    /// The source graph from which this translator translates.
    type Graph: Graph<'g>;
    /// The type of the writer into which this translator translates.
    type GraphWriter: GraphWriter<'g>;
    /// Translate a blank node from the source graph to the graph writer.
    fn translate_blank_node(
        &mut self,
        w: &mut Self::GraphWriter,
        blank_node: &<Self::Graph as Graph<'g>>::BlankNodePtr,
    ) -> <Self::GraphWriter as GraphWriter<'g>>::BlankNode;
    /// Translate a blank node or iri from the source graph to the graph writer.
    fn translate_blank_node_or_iri(
        &mut self,
        w: &mut Self::GraphWriter,
        blank_node_or_iri: &BlankNodeOrIRI<
            'g,
            <Self::Graph as Graph<'g>>::BlankNodePtr,
            <Self::Graph as Graph<'g>>::IRIPtr,
        >,
    ) -> WriterBlankNodeOrIRI<'g, Self::GraphWriter>
    where
        Self: 'g,
    {
        match *blank_node_or_iri {
            BlankNodeOrIRI::BlankNode(ref b, p) => {
                WriterBlankNodeOrIRI::BlankNode(self.translate_blank_node(w, b), p)
            }
            BlankNodeOrIRI::IRI(ref i) => WriterBlankNodeOrIRI::IRI(w.create_iri(i)),
        }
    }
    /// Translate a Resource from the source graph to the graph writer.
    fn translate_resource(
        &mut self,
        w: &mut Self::GraphWriter,
        resource: &Resource<
            'g,
            <Self::Graph as Graph<'g>>::BlankNodePtr,
            <Self::Graph as Graph<'g>>::IRIPtr,
            <Self::Graph as Graph<'g>>::LiteralPtr,
        >,
    ) -> WriterResource<'g, Self::GraphWriter>
    where
        Self: 'g,
    {
        match *resource {
            Resource::BlankNode(ref b, p) => {
                WriterResource::BlankNode(self.translate_blank_node(w, b), p)
            }
            Resource::IRI(ref i) => WriterResource::IRI(w.create_iri(i)),
            Resource::Literal(ref l) => WriterResource::Literal(w.create_literal(l)),
        }
    }
}

/// Trait for writing into a graph.
pub trait GraphWriter<'g> {
    /// The blank node type that is accepted by this writer.
    type BlankNode: Clone;
    /// The IRI type that is accepted by this writer.
    type IRI: Clone;
    /// The literal type that is accepted by this writer.
    type Literal;
    /// The datatype type that is accepted by this writer.
    type Datatype: Clone;
    /// The language type that is accepted by this writer.
    type Language;
    /// The type of the graph into which this writer writes.
    type Graph: Graph<'g>;

    /// Create a new blank node for the graph.
    fn create_blank_node(&mut self) -> Self::BlankNode;
    /// Create a new IRI from an existing IRI for the graph.
    fn create_iri<'a, I: 'a>(&mut self, &I) -> Self::IRI
    where
        I: IRIPtr<'a>;
    /// Create a new literal from an existing literal for the graph.
    fn create_literal<'a, L: 'a>(&mut self, &L) -> Self::Literal
    where
        L: LiteralPtr<'a>;
    /// Create a new datatype for the graph.
    fn create_datatype(&mut self, &str) -> Self::Datatype;
    /// Create a new language for the graph.
    fn create_language(&mut self, &str) -> Self::Language;
    /// Create a new literal with the given datatype for the graph.
    fn create_literal_datatype(&mut self, value: &str, datatype: &Self::Datatype) -> Self::Literal;
    /// Create a new literal with the given language for the graph.
    fn create_literal_language(&mut self, value: &str, language: &Self::Language) -> Self::Literal;

    /// Add a new triple to the graph.
    fn add(
        &mut self,
        subject: &WriterBlankNodeOrIRI<'g, Self>,
        predicate: &Self::IRI,
        object: &WriterResource<'g, Self>,
    ) where
        Self: Sized,
    {
        match *subject {
            WriterBlankNodeOrIRI::BlankNode(ref subject, _) => match *object {
                WriterResource::BlankNode(ref object, _) => {
                    self.add_blank_blank(subject, predicate, object);
                }
                WriterResource::IRI(ref object) => {
                    GraphWriter::add_blank_iri(self, subject, predicate, object);
                }
                WriterResource::Literal(ref object) => {
                    GraphWriter::add_blank_literal(self, subject, predicate, object);
                }
            },
            WriterBlankNodeOrIRI::IRI(ref subject) => match *object {
                WriterResource::BlankNode(ref object, _) => {
                    GraphWriter::add_iri_blank(self, subject, predicate, object);
                }
                WriterResource::IRI(ref object) => {
                    GraphWriter::add_iri_iri(self, subject, predicate, object);
                }
                WriterResource::Literal(ref object) => {
                    GraphWriter::add_iri_literal(self, subject, predicate, object);
                }
            },
        }
    }

    /// Add a new triple with blank node as subject and object to the graph.
    fn add_blank_blank(
        &mut self,
        subject: &Self::BlankNode,
        predicate: &Self::IRI,
        object: &Self::BlankNode,
    );
    /// Add a new triple with a blank node as subject and an IRI as object to the graph.
    fn add_blank_iri(
        &mut self,
        subject: &Self::BlankNode,
        predicate: &Self::IRI,
        object: &Self::IRI,
    );
    /// Add a new triple with a blank node as subject and a literal as object to the graph.
    fn add_blank_literal(
        &mut self,
        subject: &Self::BlankNode,
        predicate: &Self::IRI,
        bject: &Self::Literal,
    );
    /// Add a new triple with an IRI as subject and a blank node as object to the graph.
    fn add_iri_blank(
        &mut self,
        subject: &Self::IRI,
        predicate: &Self::IRI,
        object: &Self::BlankNode,
    );
    /// Add a new triple with an IRI as subject and an IRI as object to the graph.
    fn add_iri_iri(&mut self, subject: &Self::IRI, predicate: &Self::IRI, object: &Self::IRI);
    /// Add a new triple with an IRI as subject and a literal as object to the graph.
    fn add_iri_literal(
        &mut self,
        subject: &Self::IRI,
        predicate: &Self::IRI,
        object: &Self::Literal,
    );
    /// Close the GraphWriter and return the resulting graph.
    fn collect(self) -> Self::Graph;
}

/// An RDF graph.
///
/// An RDF graph contains triples (subject, predicate, object).
/// The triples can be filered and iterated over.
/// This trait can be used to make data accessible as RDF.
pub trait Graph<'g> {
    /// The blank node type of this graph.
    type BlankNodePtr: BlankNodePtr<'g> + Ord + Clone + 'g;
    /// The IRI type of this graph.
    type IRIPtr: IRIPtr<'g> + Ord + Clone + 'g;
    /// The literal type of this graph.
    type LiteralPtr: LiteralPtr<'g> + Ord + Clone;
    /// The implementation of triples that is ordered by subject, predicate, object.
    type SPOTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord + Clone;
    /// Iterator for iterating over all triples.
    type SPOIter: SortedIterator<Item = Self::SPOTriple>;
    /// Iterator for iterating over a range for triples sorted by subject, predicate, object.
    type SPORangeIter: SortedIterator<Item = Self::SPOTriple>;
    /// The implementation of triples that is ordered by object, predicate, subject.
    type OPSTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord + Clone;
    /// Iterator for iterating over a range for triples sorted by object, predicate, subject.
    type OPSRangeIter: SortedIterator<Item = Self::OPSTriple>;
    /// Iterate over all triples sorted by subject, predicate, object.
    fn iter(&'g self) -> Self::SPOIter;
    /// Find the DatatypePtr for the given datatype.
    fn find_datatype<'a>(
        &'g self,
        datatype: &'a str,
    ) -> Option<<Self::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>;
    /// Find the IRIPtr for the given IRI.
    fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr>;
    /// Find the LiteralPtr for the given literal.
    fn find_literal<'a>(
        &'g self,
        literal: &'a str,
        datatype: &'a str,
        language: Option<&'a str>,
    ) -> Option<Self::LiteralPtr>;
    /// Iterate over the triples that have the given subject.
    fn iter_s(
        &'g self,
        subject: &BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
    ) -> Self::SPORangeIter;
    /// Iterate over the triples that have the given subject and predicate.
    fn iter_s_p(
        &'g self,
        subject: &BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
        predicate: &Self::IRIPtr,
    ) -> Self::SPORangeIter;
    /// Iterate over the triples that have the given object.
    fn iter_o(
        &'g self,
        object: &Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
    ) -> Self::OPSRangeIter;
    /// Iterate over the triples that have the given object and predicate.
    fn iter_o_p(
        &'g self,
        object: &Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
        predicate: &Self::IRIPtr,
    ) -> Self::OPSRangeIter;

    /// Iterator that returns no results.
    fn empty_spo_range(&'g self) -> Self::SPORangeIter;
    /// Iterator that returns no results.
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
impl<'g> DatatypePtr<'g> for &'g str {
    fn as_str(&self) -> &str {
        *self
    }
}
impl<'g> LiteralPtr<'g> for &'g str {
    type DatatypePtr = &'g str;
    fn as_str(&self) -> &str {
        *self
    }
    fn datatype(&self) -> &'g str {
        constants::XSD_STRING
    }
    fn datatype_str(&self) -> &str {
        constants::XSD_STRING
    }
    fn language(&self) -> Option<&str> {
        None
    }
}
impl<'g> LiteralPtr<'g> for String {
    type DatatypePtr = &'g str;
    fn as_str(&self) -> &str {
        self.as_str()
    }
    fn datatype(&self) -> &'g str {
        constants::XSD_STRING
    }
    fn datatype_str(&self) -> &str {
        constants::XSD_STRING
    }
    fn language(&self) -> Option<&str> {
        None
    }
}
