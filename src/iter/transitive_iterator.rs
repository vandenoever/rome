//! Iterator that is a collection of iterators.

use iter::SortedIterator;
use std::collections::BTreeMap;
use std::iter::Peekable;

/// Iterator that is a collection of iterators.
/// For each iterated item, another iterator is created that is also iterated.
/// This is needed for iterating through transitive properties.
pub struct TransitiveIterator<I, J, F>
    where I: SortedIterator,
          I::Item: Ord + Clone,
          J: SortedIterator<Item = I::Item>,
          F: Fn(&J::Item) -> J
{
    iter: Peekable<I>,
    iters: BTreeMap<I::Item, Peekable<J>>,
    f: F,
}

impl<I, J, F> TransitiveIterator<I, J, F>
    where I: SortedIterator,
          I::Item: Ord + Clone,
          J: SortedIterator<Item = I::Item>,
          J::Item: Ord + Clone,
          F: Fn(&J::Item) -> J
{
    /// Create a new TransitiveIterator.
    pub fn new(iter: I, f: F) -> TransitiveIterator<I, J, F> {
        TransitiveIterator {
            iter: iter.peekable(),
            iters: BTreeMap::new(),
            f: f,
        }
    }
    fn min_next(&mut self) -> Option<usize> {
        let mut min = self.iter.peek();
        let mut min_pos = if min.is_some() { Some(0) } else { None };
        for (pos, item) in self.iters.iter_mut().enumerate() {
            match (item.1.peek(), min) {
                (Some(m), Some(n)) if m < n => {
                    min_pos = Some(pos + 1);
                    min = Some(m)
                }
                (Some(m), None) => {
                    min_pos = Some(pos + 1);
                    min = Some(m)
                }
                _ => {}
            }
        }
        min_pos
    }
    fn take_next(&mut self, pos: usize) -> Option<I::Item> {
        let mut iters = self.iters.iter_mut();
        let r = if pos == 0 {
            self.iter.next()
        } else {
            iters.nth(pos - 1).unwrap().1.next()
        };
        for i in iters {
            if i.1.peek() == r.as_ref() {
                i.1.next();
            }
        }
        r
    }
    fn add_iterator(&mut self, item: &I::Item) {
        // if an iterator is empty and the associated key has been traversed
        // already, it can be culled
        let present_and_empty = if let Some(i) = self.iters.get_mut(item) {
            if i.peek().is_some() {
                // this iterator is already present and not empty
                return;
            }
            true
        } else {
            false
        };
        if present_and_empty {
            self.iters.remove(item);
            return;
        }
        let mut new_iterator = (self.f)(item).peekable();
        // make sure the iterator has no outdated items
        loop {
            match new_iterator.peek() {
                Some(i) if i > item => break,
                None => return,
                _ => {}
            }
            new_iterator.next();
        }
        self.iters.insert(item.clone(), new_iterator);
    }
}

impl<I, J, F> Iterator for TransitiveIterator<I, J, F>
    where I: SortedIterator,
          I::Item: Ord + Clone,
          J: SortedIterator<Item = I::Item>,
          J::Item: Ord + Clone,
          F: Fn(&J::Item) -> J
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.min_next() {
            let next = self.take_next(next);
            self.add_iterator(next.as_ref().unwrap());
            next
        } else {
            None
        }
    }
}

impl<I, J, F> SortedIterator for TransitiveIterator<I, J, F>
    where I: SortedIterator,
          I::Item: Ord + Clone,
          J: SortedIterator<Item = I::Item>,
          J::Item: Ord + Clone,
          F: Fn(&J::Item) -> J
{
}
