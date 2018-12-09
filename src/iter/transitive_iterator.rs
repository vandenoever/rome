//! Iterator that is a collection of iterators.

use crate::iter::SortedIterator;
use std::collections::BTreeSet;

/// Iterator that is a collection of iterators.
/// For each iterated item, another iterator is created that is also iterated.
/// This is needed for iterating through transitive properties.
pub struct TransitiveIterator<I>
where
    I: SortedIterator,
    I::Item: Ord + Clone,
{
    items: BTreeSet<I::Item>,
}

impl<I> TransitiveIterator<I>
where
    I: SortedIterator,
    I::Item: Ord + Clone,
{
    /// Create a new TransitiveIterator.
    pub fn new<F, J>(iter: I, f: F) -> TransitiveIterator<I>
    where
        F: Fn(&I::Item) -> J,
        J: Iterator<Item = I::Item>,
    {
        // acquire the items
        let mut todo: BTreeSet<_> = iter.collect();
        let mut done = BTreeSet::new();
        while !todo.is_empty() {
            let next = todo.iter().next().unwrap().clone();
            done.insert(todo.take(&next).unwrap());
            for i in f(&next) {
                if !done.contains(&i) {
                    todo.insert(i);
                }
            }
        }
        TransitiveIterator { items: done }
    }
}

impl<I> Iterator for TransitiveIterator<I>
where
    I: SortedIterator,
    I::Item: Ord + Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            None
        } else {
            let next = self.items.iter().next().unwrap().clone();
            Some(self.items.take(&next).unwrap())
        }
    }
}

impl<I> SortedIterator for TransitiveIterator<I>
where
    I: SortedIterator,
    I::Item: Ord + Clone,
{
}
