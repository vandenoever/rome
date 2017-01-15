use std::cmp::Ordering;
use std::marker::PhantomData;
use iter::sorted_iterator::SortedIterator;
use constants;

pub trait BlankNodePtr<'g> {
    fn to_blank_node_or_iri<I>(&self) -> BlankNodeOrIRI<'g, Self, I>
        where Self: Clone,
              I: IRIPtr<'g>
    {
        BlankNodeOrIRI::BlankNode(self.clone(), PhantomData)
    }
    fn to_resource<I, L>(&self) -> Resource<'g, Self, I, L>
        where Self: Clone,
              I: IRIPtr<'g>,
              L: LiteralPtr<'g>
    {
        Resource::BlankNode(self.clone(), PhantomData)
    }
}
pub trait IRIPtr<'g> {
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

pub trait BlankNodeCreator<'a, B: 'a>
    where B: BlankNodePtr<'a>
{
    fn create_blank_node(&mut self) -> B;
}
pub trait GraphCreator<'g, B: 'g>
    where B: BlankNodePtr<'g>
{
    type Graph: Graph<'g>;
    /// Add a triple.
    ///
    fn add_triple<T: 'g, I: 'g, L: 'g>(&mut self, triple: &T)
        where T: Triple<'g, B, I, L>,
              I: IRIPtr<'g>,
              L: LiteralPtr<'g>;
    fn add_blank_blank<'p, P>(&mut self, subject: B, predicate: P, object: B) where P: IRIPtr<'p>;
    fn add_blank_iri<'p, 'o, P, O>(&mut self, subject: B, predicate: P, object: O)
        where P: IRIPtr<'p>,
              O: IRIPtr<'o>;
    fn add_blank_literal<'p, 'o, P, O>(&mut self, subject: B, predicate: P, object: O)
        where P: IRIPtr<'p>,
              O: LiteralPtr<'o>;
    fn add_iri_blank<'s, 'p, S, P>(&mut self, subject: S, predicate: P, object: B)
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
