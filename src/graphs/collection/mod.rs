//! Combine a number of graphs and access them as one graph.
use graph::*;
use std::cmp;
use std::iter::Peekable;

/// An internal enum for TripleCmpWrap which is internal to GraphCollection.
pub enum Object {
    #[doc(hidden)]
    BlankNode,
    #[doc(hidden)]
    IRI,
    #[doc(hidden)]
    Literal,
}

/// TripleCmpWrap can wrap a Triple and has no associated types.
/// This makes it possible to re-use a pointer
/// This is an internal trait for GraphCollection.
pub trait TripleCmpWrap<'g> {
    #[doc(hidden)]
    fn cmp_subject_iri(&self, o: &str) -> cmp::Ordering;
    #[doc(hidden)]
    fn cmp_predicate(&self, o: &str) -> cmp::Ordering;
    #[doc(hidden)]
    fn cmp_object_iri(&self, o: &str) -> cmp::Ordering;
    #[doc(hidden)]
    fn cmp_object_literal(&self, o: &str, datatype: &str, language: Option<&str>) -> cmp::Ordering;
    #[doc(hidden)]
    fn subject_is_blank_node(&self) -> bool;
    #[doc(hidden)]
    fn object_type(&self) -> Object;
}
fn compare_subject<'g, B: 'g, I: 'g>(a: &TripleCmpWrap,
                                     b: BlankNodeOrIRI<'g, B, I>)
                                     -> cmp::Ordering
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    match b {
        // blank nodes from different graphs are different
        // the left side is less
        BlankNodeOrIRI::BlankNode(_, _) => cmp::Ordering::Less,
        BlankNodeOrIRI::IRI(i) => a.cmp_subject_iri(i.as_str()),
    }
}
fn compare_object<'g, B: 'g, I: 'g, L: 'g>(a: &TripleCmpWrap,
                                           b: Resource<'g, B, I, L>)
                                           -> cmp::Ordering
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    match b {
        Resource::BlankNode(_, _) => cmp::Ordering::Less,
        Resource::IRI(i) => a.cmp_object_iri(i.as_str()),
        Resource::Literal(l) => a.cmp_object_literal(l.as_str(), l.datatype_str(), l.language()),
    }
}
#[doc(hidden)]
/// sort by subject, predicate, object
pub fn compare_spo<'g, B: 'g, I: 'g, L: 'g, T: 'g>(a: &TripleCmpWrap, b: &T) -> cmp::Ordering
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          T: Triple<'g, B, I, L>
{
    let mut cmp = compare_subject(a, b.subject());
    if cmp == cmp::Ordering::Equal {
        cmp = a.cmp_predicate(b.predicate().as_str());
    }
    if cmp == cmp::Ordering::Equal {
        cmp = compare_object(a, b.object());
    }
    cmp
}
#[doc(hidden)]
/// sort by object, predicate, subject
pub fn compare_ops<'g, B: 'g, I: 'g, L: 'g, T: 'g>(a: &TripleCmpWrap, b: &T) -> cmp::Ordering
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          T: Triple<'g, B, I, L>
{
    let mut cmp = compare_object(a, b.object());
    if cmp == cmp::Ordering::Less {
        cmp = a.cmp_predicate(b.predicate().as_str());
    }
    if cmp == cmp::Ordering::Less {
        cmp = compare_subject(a, b.subject());
    }
    cmp
}
#[doc(hidden)]
/// get the triple that is equal to the given triple and if needed,
/// advance the iterator
/// the iterator is forwarded one position at most
pub fn get_equal_spo<'g, K: 'g, T: 'g, B: 'g, I: 'g, L: 'g>(iter: &mut Peekable<K>,
                                                            t: &TripleCmpWrap<'g>,
                                                            n: &mut u8,
                                                            min_n: u8)
                                                            -> Option<T>
    where K: Iterator<Item = T>,
          B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          T: Triple<'g, B, I, L>
{
    *n += 1;
    if *n == min_n {
        return iter.next();
    }
    let equal = match iter.peek() {
        Some(v) => compare_spo(t, v) == cmp::Ordering::Equal,
        None => false,
    };
    if equal { iter.next() } else { None }
}
#[doc(hidden)]
// get the triple that is equal to the given triple and if needed,
// advance the iterator
// the iterator is forwarded one position at most
pub fn get_equal_ops<'g, K: 'g, T: 'g, B: 'g, I: 'g, L: 'g>(iter: &mut Peekable<K>,
                                                            t: &TripleCmpWrap<'g>,
                                                            n: &mut u8,
                                                            min_n: u8)
                                                            -> Option<T>
    where K: Iterator<Item = T>,
          B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          T: Triple<'g, B, I, L>
{
    *n += 1;
    if *n == min_n {
        return iter.next();
    }
    let equal = match iter.peek() {
        Some(v) => compare_ops(t, v) == cmp::Ordering::Equal,
        None => false,
    };
    if equal { iter.next() } else { None }
}

/// Graphs that are used in GraphCollection must implement TripleCmpWrap.
/// This macro does that.
#[macro_export]
macro_rules!
impl_triple_cmp_wrap {
    ($graph_type:path) => {
        impl_triple_cmp_wrap_spo_ops!(SPOTriple $graph_type);
        impl_triple_cmp_wrap_spo_ops!(OPSTriple $graph_type);
    }
}

/// internal macro used by impl_triple_cmp_wrap
macro_rules!
impl_triple_cmp_wrap_spo_ops {
    ($name:ident $graph_type:path) => {

        impl<'g> TripleCmpWrap<'g> for <$graph_type as $crate::graph::Graph<'g>>::$name {
            fn cmp_subject_iri(&self, o: &str) -> cmp::Ordering {
                use $crate::graph::{BlankNodeOrIRI, IRIPtr, Triple};
                match self.subject() {
                    BlankNodeOrIRI::IRI(i) => i.as_str().cmp(o),
                    _ => cmp::Ordering::Less
                }
            }
            fn cmp_predicate(&self, o: &str) -> cmp::Ordering {
                use $crate::graph::{IRIPtr, Triple};
                self.predicate().as_str().cmp(o)
            }
            fn cmp_object_iri(&self, o: &str) -> cmp::Ordering {
                use graph::{IRIPtr, Resource, Triple};
                match self.object() {
                    Resource::BlankNode(_,_) => cmp::Ordering::Less,
                    Resource::IRI(i) => i.as_str().cmp(o),
                    Resource::Literal(_) => cmp::Ordering::Greater
                }
            }
            fn cmp_object_literal(&self, o: &str, datatype: &str, language: Option<&str>) -> cmp::Ordering {
                use $crate::graph::{LiteralPtr, Resource, Triple};
                match self.object() {
                    Resource::Literal(l) => {
                        let mut cmp = l.as_str().cmp(o);
                        if cmp == cmp::Ordering::Equal {
                            cmp = l.datatype_str().cmp(datatype);
                        }
                        if cmp == cmp::Ordering::Equal {
                            cmp = l.language().cmp(&language);
                        }
                        cmp
                    },
                    _ => cmp::Ordering::Less,
                }
            }
            fn subject_is_blank_node(&self) -> bool {
                use $crate::graph::{BlankNodeOrIRI, Triple};
                match self.subject() {
                    BlankNodeOrIRI::BlankNode(_,_) => true,
                    _ => false
                }
            }
            fn object_type(&self) -> Object {
                use $crate::graph::{Resource, Triple};
                match self.object() {
                    Resource::BlankNode(_,_) => Object::BlankNode,
                    Resource::IRI(_) => Object::IRI,
                    Resource::Literal(_) => Object::Literal,
                }
            }
        }

    }
}

#[doc(hide)]
#[macro_export]
macro_rules!
spo_ops {
    ($name:ident $names:ident($( $n:tt:$graph_type:path),+) ) => {

#[doc(hidden)]
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord)]
pub struct $name<'g> {
    subject: $crate::graph::BlankNodeOrIRI<'g, BlankNode<'g>, IRI<'g>>,
    predicate: IRI<'g>,
    object: $crate::graph::Resource<'g,BlankNode<'g>,IRI<'g>,Literal<'g>>
}

impl<'g> $name<'g> {
    fn new(triple_ref: &TripleCmpWrap<'g>, triples: $names<'g>) -> $name<'g> {
        use std::marker::PhantomData;
        $name {
            subject: if triple_ref.subject_is_blank_node() {
                $crate::graph::BlankNodeOrIRI::BlankNode(BlankNode {
                    nodes: ($(triples.$n.as_ref().map(|t|t.subject().as_blank_node().unwrap().clone()),)+)
                }, PhantomData)
            } else {
                $crate::graph::BlankNodeOrIRI::IRI(IRI {
                    iris: ($(triples.$n.as_ref().map(|t|t.subject().as_iri().unwrap().clone()),)+)
                })
            },
            predicate: IRI {
                iris: ($(triples.$n.as_ref().map(|t|t.predicate()),)+)
            },
            object: match triple_ref.object_type() {
                Object::BlankNode => Resource::BlankNode(BlankNode {
                    nodes: ($(triples.$n.map(|t|t.subject().as_blank_node().unwrap().clone()),)+)
                }, PhantomData),
                Object::IRI => Resource::IRI(IRI {
                    iris: ($(triples.$n.map(|t|t.subject().as_iri().unwrap().clone()),)+)
                }),
                Object::Literal => Resource::Literal(Literal {
                    literals: ($(triples.$n.map(|t|t.object().as_literal().unwrap().clone()),)+)
                })
            }
        }
    }
}
impl<'g> Triple<'g, BlankNode<'g>, IRI<'g>, Literal<'g>> for $name<'g> {
    fn subject(&self) -> BlankNodeOrIRI<'g, BlankNode<'g>, IRI<'g>> {
        self.subject.clone()
    }
    fn predicate(&self) -> IRI<'g> {
        self.predicate.clone()
    }
    fn object(&self) -> Resource<'g, BlankNode<'g>, IRI<'g>, Literal<'g>> {
        self.object.clone()
    }
}

    }
}

/// Create a module for a collection of graphs.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate rome;
/// use rome::graph::{Graph, GraphWriter};
/// use rome::graphs::tel::{GraphCreator, Graph64, Graph128};

/// /// define a new collection type that has a Graph64 and a Graph128
/// graph_collection!(my_collection(0: ::rome::graphs::tel::Graph64,
///                                 1: ::rome::graphs::tel::Graph128));
///
/// # fn main() {
/// let mut gw1 = GraphCreator::with_capacity(0);
/// let b1 = gw1.create_blank_node();
/// let p1 = gw1.create_iri(&"p");
/// gw1.add_blank_blank(&b1, &p1, &b1);
/// let g1: Graph64 = gw1.collect();
/// assert_eq!(g1.iter().count(), 1);
/// let mut gw2 = GraphCreator::with_capacity(0);
/// let b2 = gw2.create_blank_node();
/// let p2 = gw2.create_iri(&"p");
/// gw2.add_blank_blank(&b2, &p2, &b2);
/// let g2: Graph128 = gw2.collect();
/// assert_eq!(g2.iter().count(), 1);
///
/// // combine two graphs into one
/// let g = my_collection::GraphCollection::new((&g1, &g2));
/// assert_eq!(g.iter().count(), 2);
/// # }
/// ```
#[macro_export]
macro_rules!
graph_collection {
    ($name:ident($( $n:tt:$graph_type:path),+) ) => {

#[doc(hidden)]
pub mod $name {
    use std::cmp;
    use std::fmt;
    use std::marker::PhantomData;
    use std::iter::Peekable;
    use $crate::iter::SortedIterator;
    use $crate::graph::*;
    use $crate::graphs::collection::*;

    type Graphs<'g> = ($(&'g $graph_type,)+);
    type BlankNodes<'g> = ($(Option<<$graph_type as Graph<'g>>::BlankNodePtr>,)+);
    type IRIs<'g> = ($(Option<<$graph_type as Graph<'g>>::IRIPtr>,)+);
    type Datatypes<'g> = ($(Option<<<$graph_type as Graph<'g>>::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,)+);
    type Literals<'g> = ($(Option<<$graph_type as Graph<'g>>::LiteralPtr>,)+);
    type SPOTriples<'g> = ($(Option<<$graph_type as Graph<'g>>::SPOTriple>,)+);
    type SPOIters<'g> = ($(Peekable<<$graph_type as Graph<'g>>::SPOIter>,)+);
    type SPORangeIters<'g> = ($(Peekable<<$graph_type as Graph<'g>>::SPORangeIter>,)+);
    type OPSTriples<'g> = ($(Option<<$graph_type as Graph<'g>>::OPSTriple>,)+);
    type OPSRangeIters<'g> = ($(Peekable<<$graph_type as Graph<'g>>::OPSRangeIter>,)+);

    spo_ops!(SPOTriple SPOTriples($( $n:$graph_type),+));
    spo_ops!(OPSTriple OPSTriples($( $n:$graph_type),+));

    #[derive(Clone,PartialEq,Eq,PartialOrd,Ord)]
    pub struct BlankNode<'g> {
        nodes: BlankNodes<'g>
    }
    impl<'g> fmt::Display for BlankNode<'g> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "_")?;
            $(
                match self.nodes.$n {
                    Some(b) => {
                        write!(f, "._{}", b)?;
                    }
                    None => {
                        write!(f, "._")?;
                    }
                }
            )*
            Ok(())
        }
    }
    impl<'g> BlankNodePtr<'g> for BlankNode<'g> {}
    #[derive(Clone,PartialEq,Eq,PartialOrd,Ord)]
    pub struct IRI<'g> {
        iris: IRIs<'g>,
    }
    impl<'g> IRIPtr<'g> for IRI<'g> {
        fn as_str(&self) -> &str {
            $(
                if let Some(ref v) = self.iris.$n {
                    return v.as_str();
                }
            )+
            panic!("unreachable")
        }
    }
    #[derive(Clone,PartialEq)]
    pub struct Datatype<'g> {
        datatypes: Datatypes<'g>
    }
    impl<'g> DatatypePtr<'g> for Datatype<'g> {
        fn as_str(&self) -> &str {
            $(
                if let Some(ref v) = self.datatypes.$n {
                    return v.as_str();
                }
            )+
            panic!("unreachable")
        }
    }
    #[derive(Clone,PartialEq,Eq,PartialOrd,Ord)]
    pub struct Literal<'g> {
        literals: Literals<'g>,
    }
    impl<'g> LiteralPtr<'g> for Literal<'g> {
        type DatatypePtr = Datatype<'g>;
        fn as_str(&self) -> &str {
            $(
                if let Some(ref v) = self.literals.$n {
                    return v.as_str();
                }
            )+
            panic!("unreachable")
        }
        fn datatype(&self) -> Self::DatatypePtr {
            Datatype {
                datatypes: ($( self.literals.$n.as_ref().map(|l|l.datatype()), )+)
            }
        }
        fn datatype_str(&self) -> &str {
            $(
                if let Some(ref v) = self.literals.$n {
                    return v.datatype_str();
                }
            )+
            panic!("unreachable")
        }
        fn language(&self) -> Option<&str> {
            $(
                if let Some(ref v) = self.literals.$n {
                    return v.language();
                }
            )+
            panic!("unreachable")
        }
    }
    pub struct SPOIter<'g> {
        iters: SPOIters<'g>
    }
    impl<'g> Iterator for SPOIter<'g> {
        type Item = SPOTriple<'g>;
        fn next(&mut self) -> Option<SPOTriple<'g>> {
            let triples = ($(self.iters.$n.peek().map(|v|v.clone()),)+);
            let mut n = 0;
            let mut min_n = 0;
            let mut min = None;
            $(
                n += 1;
                if let Some(t) = triples.$n.as_ref() {
                    if min.is_none() || compare_spo(min.unwrap(), t) == cmp::Ordering::Greater {
                        min_n = n;
                        min = Some(t as &TripleCmpWrap);
                    }
                }
            )+
            if let Some(t) = min {
                n = 0;
                Some(SPOTriple::new(t, ($(
                    get_equal_spo(&mut self.iters.$n, t, &mut n, min_n),
                )+)))
            } else {
                None
            }
        }
    }
    impl<'g> SortedIterator for SPOIter<'g> {}
    pub struct SPORangeIter<'g> {
        iters: SPORangeIters<'g>
    }
    impl<'g> Iterator for SPORangeIter<'g> {
        type Item = SPOTriple<'g>;
        fn next(&mut self) -> Option<SPOTriple<'g>> {
            let triples = ($(self.iters.$n.peek().map(|v|v.clone()),)+);
            let mut n = 0;
            let mut min_n = 0;
            let mut min = None;
            $(
                n += 1;
                if let Some(t) = triples.$n.as_ref() {
                    if min.is_none() || compare_spo(min.unwrap(), t) == cmp::Ordering::Greater {
                        min_n = n;
                        min = Some(t as &TripleCmpWrap);
                    }
                }
            )+
            if let Some(t) = min {
                n = 0;
                Some(SPOTriple::new(t, ($(
                    get_equal_spo(&mut self.iters.$n, t, &mut n, min_n),
                )+)))
            } else {
                None
            }
        }
    }
    impl<'g> SortedIterator for SPORangeIter<'g> {}
    pub struct OPSRangeIter<'g> {
        iters: OPSRangeIters<'g>
    }
    impl<'g> Iterator for OPSRangeIter<'g> {
        type Item = OPSTriple<'g>;
        fn next(&mut self) -> Option<OPSTriple<'g>> {
            let triples = ($(self.iters.$n.peek().map(|v|v.clone()),)+);
            let mut n = 0;
            let mut min_n = 0;
            let mut min = None;
            $(
                n += 1;
                if let Some(t) = triples.$n.as_ref() {
                    if min.is_none() || compare_ops(min.unwrap(), t) == cmp::Ordering::Greater {
                        min_n = n;
                        min = Some(t as &TripleCmpWrap);
                    }
                }
            )+
            if let Some(t) = min {
                n = 0;
                Some(OPSTriple::new(t, ($(
                    get_equal_ops(&mut self.iters.$n, t, &mut n, min_n),
                )+)))
            } else {
                None
            }
        }
    }
    impl<'g> SortedIterator for OPSRangeIter<'g> {}

    /// Access a number of graphs as one graph.
    pub struct GraphCollection<'g> {
        graphs: Graphs<'g>,
        phantom: PhantomData<&'g u8>
    }
    impl<'g> GraphCollection<'g> {
        /// Create a new graph collection.
        pub fn new(graphs: Graphs<'g>) -> GraphCollection<'g> {
            GraphCollection {
                graphs: graphs,
                phantom: PhantomData
            }
        }
    }
    impl<'g> Graph<'g> for GraphCollection<'g> {
        type BlankNodePtr = BlankNode<'g>;
        type IRIPtr = IRI<'g>;
        type LiteralPtr = Literal<'g>;
        type SPOTriple = SPOTriple<'g>;
        type SPOIter = SPOIter<'g>;
        type SPORangeIter = SPORangeIter<'g>;
        type OPSTriple = OPSTriple<'g>;
        type OPSRangeIter = OPSRangeIter<'g>;
        fn iter(&'g self) -> Self::SPOIter {
            SPOIter {
                iters: ($(self.graphs.$n.iter().peekable(),)+)
            }
        }
        fn find_iri<'a>(&'g self, iri: &'a str) -> Option<Self::IRIPtr> {
            let iris = ($( self.graphs.$n.find_iri(iri) ),+);
            let any = $( iris.$n.is_some() )||+;
            if any {
                Some(IRI{
                    iris: iris
                })
            } else {
                None
            }
        }
        fn find_literal<'a>(&'g self,
                            literal: &'a str,
                            datatype: &'a str,
                            language: Option<&'a str>)
                            -> Option<Self::LiteralPtr> {
            let literals = ($( self.graphs.$n.find_literal(literal, datatype, language) ),+);
            let any = $( literals.$n.is_some() )||+;
            if any {
                Some(Literal{
                    literals: literals
                })
            } else {
                None
            }
        }
        fn find_datatype<'a>(&'g self,
                            datatype: &'a str)
                            -> Option<Datatype> {
            let datatypes = ($( self.graphs.$n.find_datatype(datatype) ),+);
            let any = $( datatypes.$n.is_some() )||+;
            if any {
                Some(Datatype{
                    datatypes: datatypes
                })
            } else {
                None
            }
        }
        fn iter_s(&'g self,
                    subject: &BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>)
                    -> Self::SPORangeIter {
            let iters = match subject {
                &BlankNodeOrIRI::BlankNode(ref b, _) => { ($(
                    match b.nodes.$n {
                        Some(b) => {
                            self.graphs.$n.iter_s(&BlankNodeOrIRI::BlankNode(b, PhantomData))
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) },
                &BlankNodeOrIRI::IRI(ref i) => { ($(
                    match i.iris.$n {
                        Some(ref i) => {
                            self.graphs.$n.iter_s(&BlankNodeOrIRI::IRI(i.clone()))
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) }
            };
            SPORangeIter { iters: iters }
        }
        fn iter_s_p(&'g self,
                    subject: &BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                    predicate: &Self::IRIPtr)
                    -> Self::SPORangeIter {
            let iters = match subject {
                &BlankNodeOrIRI::BlankNode(ref b, _) => { ($(
                    match (b.nodes.$n, &predicate.iris.$n) {
                        (Some(b), &Some(ref p)) => {
                            self.graphs.$n.iter_s_p(&BlankNodeOrIRI::BlankNode(b, PhantomData), p)
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) },
                &BlankNodeOrIRI::IRI(ref i) => { ($(
                    match (&i.iris.$n, &predicate.iris.$n) {
                        (&Some(ref i), &Some(ref p)) => {
                            self.graphs.$n.iter_s_p(&BlankNodeOrIRI::IRI(i.clone()), p)
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) }
            };
            SPORangeIter { iters: iters }
        }
        fn iter_o(&'g self,
                    object: &Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>)
                    -> Self::OPSRangeIter {
            let iters = match object {
                &Resource::BlankNode(ref b, _) => { ($(
                    match b.nodes.$n {
                        Some(b) => {
                            self.graphs.$n.iter_o(&Resource::BlankNode(b, PhantomData))
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                &Resource::IRI(ref i) => { ($(
                    match i.iris.$n {
                        Some(ref i) => {
                            self.graphs.$n.iter_o(&Resource::IRI(i.clone()))
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                &Resource::Literal(ref l) => { ($(
                    match l.literals.$n {
                        Some(ref l) => {
                            self.graphs.$n.iter_o(&Resource::Literal(l.clone()))
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) }
            };
            OPSRangeIter { iters: iters }
        }
        fn iter_o_p(&'g self,
                    object: &Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
                    predicate: &Self::IRIPtr)
                    -> Self::OPSRangeIter {
            let iters = match object {
                &Resource::BlankNode(ref b, _) => { ($(
                    match (b.nodes.$n, &predicate.iris.$n) {
                        (Some(b), &Some(ref p)) => {
                            self.graphs.$n.iter_o_p(&Resource::BlankNode(b, PhantomData), p)
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                &Resource::IRI(ref i) => { ($(
                    match (&i.iris.$n, &predicate.iris.$n) {
                        (&Some(ref i), &Some(ref p)) => {
                            self.graphs.$n.iter_o_p(&Resource::IRI(i.clone()), p)
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                &Resource::Literal(ref l) => { ($(
                    match (&l.literals.$n, &predicate.iris.$n) {
                        (&Some(ref l), &Some(ref p)) => {
                            self.graphs.$n.iter_o_p(&Resource::Literal(l.clone()), p)
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) }
            };
            OPSRangeIter { iters: iters }
        }
        /// iterator that returns no results
        fn empty_spo_range(&'g self) -> Self::SPORangeIter {
            SPORangeIter {
                iters: ($( self.graphs.$n.empty_spo_range().peekable(), )+)
            }
        }
        /// iterator that returns no results
        fn empty_ops_range(&'g self) -> Self::OPSRangeIter {
            OPSRangeIter {
                iters: ($( self.graphs.$n.empty_ops_range().peekable(), )+)
            }
        }
    }
}
    }
}