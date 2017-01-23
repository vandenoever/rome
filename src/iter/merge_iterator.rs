//! Iterator that merges other iterators.

use iter::SortedIterator;
use std::iter::Peekable;

/// Iterator that merges other iterators.
///
/// Each unique value in the merged iterators is returned only once.
/// The input iterators must be sorted.
pub struct MergeIterator<I>
    where I: SortedIterator,
          I::Item: Ord
{
    iters: Vec<Peekable<I>>,
}

impl<I> MergeIterator<I>
    where I: SortedIterator,
          I::Item: Ord
{
    /// Create a new MergeIterator.
    pub fn new() -> MergeIterator<I> {
        MergeIterator { iters: Vec::new() }
    }
    /// Create a new MergeIterator that expects `capacity` iterators.
    pub fn with_capacity(capacity: usize) -> MergeIterator<I> {
        MergeIterator { iters: Vec::with_capacity(capacity) }
    }
    /// Add another iterator to the MergeIterator.
    pub fn push(&mut self, i: I) {
        self.iters.push(i.peekable())
    }
    /// return position of the iterator with next the smallest item
    /// that any of the iterators would return on the next iteration
    fn min_next(&mut self) -> Option<usize> {
        let mut min_pos = None;
        let mut min = None;
        let mut pos = 0;
        for i in self.iters.iter_mut() {
            match (i.peek(), min) {
                (Some(m), Some(n)) if m < n => {
                    min_pos = Some(pos);
                    min = Some(m)
                }
                (Some(m), None) => {
                    min_pos = Some(pos);
                    min = Some(m)
                }
                _ => {}
            }
            pos += 1;
        }
        min_pos
    }
    /// take the next item, that item should be determined by min_next
    fn take_next(&mut self, pos: usize) -> Option<I::Item> {
        let r = self.iters[pos].next();
        for i in self.iters.iter_mut().skip(pos + 1) {
            if i.peek() == r.as_ref() {
                i.next();
            }
        }
        r
    }
}

impl<I> Iterator for MergeIterator<I>
    where I: SortedIterator,
          I::Item: Ord
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.min_next() {
            self.take_next(next)
        } else {
            None
        }
    }
}
impl<I> SortedIterator for MergeIterator<I>
    where I: SortedIterator,
          I::Item: Ord
{
}
