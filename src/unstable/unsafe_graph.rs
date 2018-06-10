use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::mem;
use unsafe_key::UnsafeKey;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    one: String,
    two: String,
}

pub struct Store {
    store: Vec<Item>,
    index: BTreeSet<UnsafeKey<Item>>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            store: Vec::new(),
            index: BTreeSet::new(),
        }
    }
    fn get_index(&self, item: &Item) -> Option<usize> {
        if let Some(entry) = self.index.get(&UnsafeKey::new(item)) {
            Some(entry.offset(&self.store[..]))
        } else {
            None
        }
    }
    pub fn add(&mut self, item: &Item) {}
    pub fn remove(&mut self, key: &Item) {}
}

#[test]
fn add() {
    let mut store = Store::new();
    let item1 = Item {
        one: String::from("x"),
        two: String::from("y"),
    };
    let item2 = Item {
        one: String::from("a"),
        two: String::from("b"),
    };
    store.add(&item1);
    println!("TESTED!!");
}
