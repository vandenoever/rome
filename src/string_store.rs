use std::rc::Rc;
use std::collections::BTreeMap;
use std::cmp::Ord;
use std::borrow::Borrow;
use std::default::Default;

pub struct StringStoreItem<T>
    where T: Default
{
    string: StoreString,
    item: T,
}

pub struct StringStore<T>
    where T: Default
{
    store: Vec<StringStoreItem<T>>,
    unused: Vec<usize>,
    index: BTreeMap<StoreString, StringId>,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
struct StoreString {
    string: Rc<String>,
}

impl Borrow<str> for StoreString {
    fn borrow(&self) -> &str {
        self.string.as_str()
    }
}

impl Borrow<Rc<String>> for StoreString {
    fn borrow(&self) -> &Rc<String> {
        &self.string
    }
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct StringId {
    index: usize,
}

impl<T> StringStore<T>
    where T: Default
{
    pub fn new() -> StringStore<T> {
        StringStore {
            store: Vec::new(),
            unused: Vec::new(),
            index: BTreeMap::new(),
        }
    }
    fn add_string(&mut self, s: Rc<String>) -> StringId {
        let item = StringStoreItem {
            string: StoreString { string: s },
            item: T::default(),
        };
        let index;
        if let Some(i) = self.unused.pop() {
            index = i;
            self.store[i] = item;
        } else {
            index = self.store.len();
            self.store.push(item);
        }
        StringId { index: index }
    }
    pub fn register_string(&mut self, s: &Rc<String>) -> StringId {
        if let Some(s) = self.index.get(s) {
            return *s;
        }
        self.add_string(s.clone())
    }
    pub fn register_str(&mut self, s: &str) -> StringId {
        if let Some(s) = self.index.get(s) {
            return *s;
        }
        self.add_string(Rc::new(String::from(s)))
    }
    pub fn unregister_string(&mut self, id: StringId) {
        self.index.remove(&self.store[id.index].string);
        self.unused.push(id.index);
    }
    pub fn get_string(&self, id: StringId) -> &Rc<String> {
        &self.store[id.index].string.string
    }
    pub fn get_item(&self, id: StringId) -> &T {
        &self.store[id.index].item
    }
    pub fn get_mut_item(&mut self, id: StringId) -> &mut T {
        &mut self.store[id.index].item
    }
}
