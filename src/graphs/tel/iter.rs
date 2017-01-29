use super::compact_triple::*;
use super::graph::*;
use super::triple::*;
use graph;
use iter::SortedIterator;
use std::marker::PhantomData;

pub struct SubjectIter<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub pos: usize,
    pub graph: &'g GraphData<SPO, OPS>,
}
impl<'g, SPO, OPS> Iterator for SubjectIter<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    type Item = graph::BlankNodeOrIRI<'g, BlankNodePtr<'g, SPO, OPS>, IRIPtr<'g, SPO, OPS>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.graph.spo.len() {
            return None;
        }
        let current = self.graph.spo[self.pos];
        let is_iri = current.subject_is_iri();
        let subject = current.subject();
        // advance to the next position
        while self.pos < self.graph.spo.len() {
            let t = self.graph.spo[self.pos];
            if t.subject() != subject || t.subject_is_iri() != is_iri {
                break;
            }
            self.pos += 1;
        }
        Some(if is_iri {
            graph::BlankNodeOrIRI::IRI(IRIPtr {
                graph: self.graph,
                iri: subject,
            })
        } else {
            graph::BlankNodeOrIRI::BlankNode(BlankNodePtr {
                                                 graph_id: self.graph.graph_id,
                                                 node_id: subject,
                                                 phantom: PhantomData,
                                             },
                                             PhantomData)
        })
    }
}
impl<'g, SPO, OPS> SortedIterator for SubjectIter<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}

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
