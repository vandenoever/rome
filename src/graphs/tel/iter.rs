use iter::sorted_iterator::*;
use std::marker::PhantomData;
use super::compact_triple::*;
use super::graph::*;
use super::triple::*;

pub struct GraphIterator<'g, SPO: 'g, OPS: 'g, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    pub graph: &'g GraphData<SPO, OPS>,
    pub pos: usize,
    pub phantom: PhantomData<(T, F)>,
}

impl<'g, SPO, OPS, T, F> Iterator for GraphIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
    type Item = Triple<'g, SPO, OPS, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < F::index(&self.graph).len() {
            let triple = F::index(&self.graph)[self.pos];
            self.pos += 1;
            Some(Triple {
                graph: self.graph,
                triple: triple,
            })
        } else {
            None
        }
    }
}
impl<'g, SPO, OPS, T, F> SortedIterator for GraphIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
}

pub struct TripleRangeIterator<'g, SPO: 'g, OPS: 'g, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    pub graph: &'g GraphData<SPO, OPS>,
    pub pos: usize,
    pub end: T,
    pub phantom: PhantomData<F>,
}

impl<'g, SPO, OPS, T, F> Iterator for TripleRangeIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
    type Item = Triple<'g, SPO, OPS, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < F::index(&self.graph).len() {
            let triple = F::index(&self.graph)[self.pos];
            self.pos += 1;
            if triple < self.end {
                Some(Triple {
                    graph: &self.graph,
                    triple: triple,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl<'g, SPO, OPS, T, F> SortedIterator for TripleRangeIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
}

type S<'g, SPO, OPS> = TripleRangeIterator<'g, SPO, OPS, SPO, SPOIndex<SPO, OPS>>;
