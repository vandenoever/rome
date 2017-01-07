
/// Iterator that gives out items in ascending order
/// This trait only indicates that the iterator works like this.
/// The implementations of the trait are responsible for ensuring that
/// items are sorted and that each value occurs only once.
pub trait SortedIterator: Iterator where <Self as Iterator>::Item: Ord {}
