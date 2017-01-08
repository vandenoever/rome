use std::fmt::Debug;
use std::marker::PhantomData;
use iter::sorted_iterator::SortedIterator;
use constants;

pub trait BlankNodePtr<'g>: Ord + Copy + Debug {
    // todo: make graph_id into a trait so ensure that it is unique at runtime
    fn graph_id(&self) -> u32;
    fn node_id(&self) -> u32;
    fn to_blank_node_or_iri<I>(&self) -> BlankNodeOrIRI<'g, Self, I>
        where I: IRIPtr<'g>
    {
        BlankNodeOrIRI::BlankNode(self.clone(), PhantomData)
    }
    fn to_resource<I, L>(&self) -> Resource<'g, Self, I, L>
        where I: IRIPtr<'g>,
              L: LiteralPtr<'g>
    {
        Resource::BlankNode(self.clone(), PhantomData)
    }
}
pub trait IRIPtr<'g>: Ord + Clone + Debug {
    fn as_str(&self) -> &str;
    fn to_blank_node_or_iri<B>(&self) -> BlankNodeOrIRI<'g, B, Self>
        where B: BlankNodePtr<'g>
    {
        BlankNodeOrIRI::IRI(self.clone())
    }
    fn to_resource<B, L>(&self) -> Resource<'g, B, Self, L>
        where B: BlankNodePtr<'g>,
              L: LiteralPtr<'g>
    {
        Resource::IRI(self.clone())
    }
}
pub trait LiteralPtr<'g>: Ord + Clone + Debug {
    fn as_str(&self) -> &str;
    fn datatype(&self) -> &str;
    fn language(&self) -> Option<&str>;
    fn to_resource<B, I>(&self) -> Resource<'g, B, I, Self>
        where B: BlankNodePtr<'g>,
              I: IRIPtr<'g>
    {
        Resource::Literal(self.clone())
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum BlankNodeOrIRI<'g, B: 'g, I: 'g>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    BlankNode(B, PhantomData<&'g u32>),
    IRI(I),
}
impl<'g, B, I> BlankNodeOrIRI<'g, B, I>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    pub fn as_iri(&self) -> Option<&I> {
        match self {
            &BlankNodeOrIRI::IRI(ref t) => Some(t),
            _ => None,
        }
    }
    pub fn to_resource<L>(&self) -> Resource<'g, B, I, L>
        where L: LiteralPtr<'g>
    {
        match self {
            &BlankNodeOrIRI::BlankNode(ref t, _) => Resource::BlankNode(t.clone(), PhantomData),
            &BlankNodeOrIRI::IRI(ref t) => Resource::IRI(t.clone()),
        }
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum IRIOrLiteral<'g, I: 'g, L: 'g>
    where I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          L: LiteralPtr<'g>
{
    IRI(I, PhantomData<&'g u32>),
    Literal(L),
}
impl<'g, I, L> IRIOrLiteral<'g, I, L>
    where I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
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
        where B: BlankNodePtr<'g>
    {
        match self {
            &IRIOrLiteral::IRI(ref t, _) => Resource::IRI(t.clone()),
            &IRIOrLiteral::Literal(ref t) => Resource::Literal(t.clone()),
        }
    }
}
#[derive(PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
pub enum Resource<'g, B: 'g, I: 'g, L: 'g>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    BlankNode(B, PhantomData<&'g u32>),
    IRI(I),
    Literal(L),
}
impl<'g, B, I, L> Resource<'g, B, I, L>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
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
    pub fn to_blank_node_or_iri(&self) -> Option<BlankNodeOrIRI<'g, B, I>> {
        match self {
            &Resource::BlankNode(ref t, _) => {
                Some(BlankNodeOrIRI::BlankNode(t.clone(), PhantomData))
            }
            &Resource::IRI(ref t) => Some(BlankNodeOrIRI::IRI(t.clone())),
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

pub trait IntoIRIPtr<'a> {
    fn iri<I>(self) -> I where I: IRIPtr<'a>;
}

pub trait GraphCreator<'g> {
    type Graph: Graph<'g>;
    fn create_blank_node(&mut self) -> <Self::Graph as Graph<'g>>::BlankNodePtr;
    fn add_triple<'h, T, B: 'h, I: 'h, L: 'h>(&mut self, triple: &T)
        where T: Triple<'h, B, I, L>,
              B: BlankNodePtr<'h>,
              I: IRIPtr<'h>,
              L: LiteralPtr<'h>;
    fn add_blank_blank<'p, P>(&mut self, subject: <Self::Graph as Graph<'g>>::BlankNodePtr,
            predicate: P,
            object: <Self::Graph as Graph<'g>>::BlankNodePtr)
        where P: IRIPtr<'p>;
    fn add_blank_iri<'p, 'o, P, O>(&mut self, subject: <Self::Graph as Graph<'g>>::BlankNodePtr, predicate: P, object: O)
        where P: IRIPtr<'p>,
              O: IRIPtr<'o>;
    fn add_blank_literal<'p, 'o, P, O>(&mut self, subject: <Self::Graph as Graph<'g>>::BlankNodePtr, predicate: P, object: O)
        where P: IRIPtr<'p>,
              O: LiteralPtr<'o>;
    fn add_iri_blank<'s, 'p, S, P>(&mut self, subject: S, predicate: P, object: <Self::Graph as Graph<'g>>::BlankNodePtr)
        where S: IRIPtr<'s>,
              P: IRIPtr<'p>;
    fn add_iri_iri<'s, 'p, 'o, S, P, O>(&mut self, subject: S, predicate: P, object: O)
        where S: IRIPtr<'s>,
              P: IRIPtr<'p>,
              O: IRIPtr<'o>;
    fn add_iri_literal<'s, 'p, 'o, S, P, O>(&mut self, subject: S, predicate: P, object: O)
        where S: IRIPtr<'s>,
              P: IRIPtr<'p>,
              O: LiteralPtr<'o>;
    fn collect(&mut self) -> Self::Graph;
}

pub trait Graph<'g> {
    type BlankNodePtr: BlankNodePtr<'g>;
    type IRIPtr: IRIPtr<'g>;
    type LiteralPtr: LiteralPtr<'g>;
    type SPOTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord;
    type SPOIter: SortedIterator<Item = Self::SPOTriple>;
    type SPORangeIter: SortedIterator<Item = Self::SPOTriple>;
    type OPSTriple: Triple<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr> + Ord;
    type OPSRangeIter: SortedIterator<Item = Self::OPSTriple>;
    fn iter(&'g self) -> Self::SPOIter;

    fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr>;

    fn iter_s_p(&'g self,
                subject: BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                predicate: Self::IRIPtr)
                -> Self::SPORangeIter;
    fn iter_o_p(&'g self,
                object: Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
                predicate: Self::IRIPtr)
                -> Self::OPSRangeIter;

    /// iterator over all triples with the same subject and predicate
    fn iter_subject_predicate(&'g self,
                              subject: BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                              predicate: &str)
                              -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_spo_range(&'g self) -> Self::SPORangeIter;
    /// iterator that returns no results
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter;

    /// return the number of triples in the graph
    fn len(&self) -> usize;
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
