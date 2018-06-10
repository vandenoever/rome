use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;

/// Key that points to an item
pub struct UnsafeKey<T> {
    key: *const T,
}

impl<T> UnsafeKey<T> {
    pub fn new(item: &T) -> UnsafeKey<T> {
        UnsafeKey { key: item }
    }
    fn get(&self) -> &T {
        unsafe { &(*self.key) }
    }
    pub fn offset(&self, slice: &[T]) -> usize {
        let start: *const T = &slice[0];
        (self.key as usize - start as usize) / mem::size_of::<T>()
    }
}

impl<T> PartialEq for UnsafeKey<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &UnsafeKey<T>) -> bool {
        self.get().eq(other.get())
    }
}
impl<T> PartialOrd for UnsafeKey<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &UnsafeKey<T>) -> Option<Ordering> {
        self.get().partial_cmp(other.get())
    }
}
impl<T> Eq for UnsafeKey<T>
where
    T: Eq,
{
}
impl<T> Ord for UnsafeKey<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(other.get())
    }
}
impl<T> Hash for UnsafeKey<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}
