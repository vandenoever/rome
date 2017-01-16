use std::cmp;
use std::iter::Peekable;
use graph::*;

pub enum Object {
    BlankNode,
    IRI,
    Literal,
}

/// TripleCmpWrap can wrap a Triple and has no associated types.
/// This makes it possible to re-use a pointer
pub trait TripleCmpWrap<'g> {
    fn cmp_subject_iri(&self, o: &str) -> cmp::Ordering;
    fn cmp_predicate(&self, o: &str) -> cmp::Ordering;
    fn cmp_object_iri(&self, o: &str) -> cmp::Ordering;
    fn cmp_object_literal(&self, o: &str, datatype: &str, language: Option<&str>) -> cmp::Ordering;
    fn subject_is_blank_node(&self) -> bool;
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
        Resource::Literal(l) => a.cmp_object_literal(l.as_str(), l.datatype(), l.language()),
    }
}
// sort by subject, predicate, object
pub fn compare_spo<'g, B: 'g, I: 'g, L: 'g, T: 'g>(a: &TripleCmpWrap, b: &T) -> cmp::Ordering
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          T: Triple<'g, B, I, L>
{
    let mut cmp = compare_subject(a, b.subject());
    if cmp == cmp::Ordering::Less {
        cmp = a.cmp_predicate(b.predicate().as_str());
    }
    if cmp == cmp::Ordering::Less {
        cmp = compare_object(a, b.object());
    }
    cmp
}
// sort by object, predicate, subject
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
// get the triple that is equal to the given triple and if needed,
// advance the iterator
// the iterator is forwarded one position at most
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
                            cmp = l.datatype().cmp(datatype);
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

macro_rules!
spo_ops {
    ($name:ident $names:ident($( $n:tt:$graph_type:path),+) ) => {

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

#[macro_export]
macro_rules!
graph_collection {
    ($name:ident($( $n:tt:$graph_type:path),+) ) => {


pub mod $name {
    use std::cmp;
    use std::fmt;
    use std::marker::PhantomData;
    use std::iter::Peekable;
    use $crate::iter::SortedIterator;
    use $crate::graph::*;
    use $crate::graphs::collection::graph_collection::*;

    type Graphs<'g> = ($($graph_type,)+);
    type BlankNodes<'g> = ($(Option<<$graph_type as Graph<'g>>::BlankNodePtr>,)+);
    type IRIs<'g> = ($(Option<<$graph_type as Graph<'g>>::IRIPtr>,)+);
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
    #[derive(Clone,PartialEq,Eq,PartialOrd,Ord)]
    pub struct Literal<'g> {
        literals: Literals<'g>,
    }
    impl<'g> LiteralPtr<'g> for Literal<'g> {
        fn as_str(&self) -> &str {
            $(
                if let Some(ref v) = self.literals.$n {
                    return v.as_str();
                }
            )+
            panic!("unreachable")
        }
        fn datatype(&self) -> &str {
            $(
                if let Some(ref v) = self.literals.$n {
                    return v.datatype();
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

    pub struct GraphCollection<'g> {
        graphs: Graphs<'g>,
        phantom: PhantomData<&'g u8>
    }
    impl<'g> GraphCollection<'g> {
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
        fn iter_s_p(&'g self,
                    subject: BlankNodeOrIRI<'g, Self::BlankNodePtr, Self::IRIPtr>,
                    predicate: Self::IRIPtr)
                    -> Self::SPORangeIter {
            let iters = match subject {
                BlankNodeOrIRI::BlankNode(b, _) => { ($(
                    match (b.nodes.$n, predicate.iris.$n) {
                        (Some(b), Some(p)) => {
                            self.graphs.$n.iter_s_p(BlankNodeOrIRI::BlankNode(b, PhantomData), p)
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) },
                BlankNodeOrIRI::IRI(i) => { ($(
                    match (i.iris.$n, predicate.iris.$n) {
                        (Some(i), Some(p)) => {
                            self.graphs.$n.iter_s_p(BlankNodeOrIRI::IRI(i), p)
                        },
                        _ => {
                            self.graphs.$n.empty_spo_range()
                        }
                    }.peekable()
                ,)+) }
            };
            SPORangeIter { iters: iters }
        }
        fn iter_o_p(&'g self,
                    object: Resource<'g, Self::BlankNodePtr, Self::IRIPtr, Self::LiteralPtr>,
                    predicate: Self::IRIPtr)
                    -> Self::OPSRangeIter {
            let iters = match object {
                Resource::BlankNode(b, _) => { ($(
                    match (b.nodes.$n, predicate.iris.$n) {
                        (Some(b), Some(p)) => {
                            self.graphs.$n.iter_o_p(Resource::BlankNode(b, PhantomData), p)
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                Resource::IRI(i) => { ($(
                    match (i.iris.$n, predicate.iris.$n) {
                        (Some(i), Some(p)) => {
                            self.graphs.$n.iter_o_p(Resource::IRI(i), p)
                        },
                        _ => {
                            self.graphs.$n.empty_ops_range()
                        }
                    }.peekable()
                ,)+) },
                Resource::Literal(l) => { ($(
                    match (l.literals.$n, predicate.iris.$n) {
                        (Some(l), Some(p)) => {
                            self.graphs.$n.iter_o_p(Resource::Literal(l), p)
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

// graph_collection!(test_collection(0: a::G<'g>, 1: b::G<'g>, 2: c::G<'g>));
//
// pub fn test() {
// let a = a::G{phantom: PhantomData};
// let b = b::G{phantom: PhantomData};
// let c = c::G{phantom: PhantomData};
// test_collection::GraphCollection::new((a,b,c));
// }
//
