use rand;
use std::collections::btree_map::Entry;
use std::collections::btree_set;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::rc::Rc;
use string_store::*;

/// Graph implementation that allows for multiple indexes
/// All data is stored in vectors.

pub struct VecStore<T> {
    store: Vec<T>,
    unused: Vec<usize>,
}

struct StringUsage {
    subject_count: usize,
    predicate_count: usize,
    object_count: usize,
    datatype_count: usize,
    literal_count: usize,
    langtag_count: usize,
}

impl Default for StringUsage {
    fn default() -> StringUsage {
        StringUsage {
            subject_count: 0,
            predicate_count: 0,
            object_count: 0,
            datatype_count: 0,
            literal_count: 0,
            langtag_count: 0,
        }
    }
}

pub trait TIRI<'a> {
    fn iri(&self) -> &'a str;
}

pub enum TSubject<'a> {
    IRI(&'a TIRI<'a>),
    BlankNode(String),
}

pub struct VecStoreId<T> {
    index: usize,
    phantom: PhantomData<T>,
}

pub struct IndexGraph {
    strings: StringStore<StringUsage>,
    triples: BTreeSet<Triple>,
}

impl<T> VecStore<T> {
    fn new() -> VecStore<T> {
        VecStore {
            store: Vec::new(),
            unused: Vec::new(),
        }
    }
}

enum Which {
    Subject,
    Predicate,
    Object,
    Datatype,
    Literal,
    LangTag,
}

fn up_use(iri: &mut StringUsage, which: Which) {
    match which {
        Which::Subject => iri.subject_count += 1,
        Which::Predicate => iri.predicate_count += 1,
        Which::Object => iri.object_count += 1,
        Which::Datatype => iri.datatype_count += 1,
        Which::Literal => iri.literal_count += 1,
        Which::LangTag => iri.langtag_count += 1,
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Triple {
    subject: StringId,
    predicate: StringId,
}

impl IndexGraph {
    pub fn new() -> IndexGraph {
        IndexGraph {
            strings: StringStore::new(),
            triples: BTreeSet::new(),
        }
    }
    fn register_string(&mut self, s: &Rc<String>, which: Which) -> StringId {
        let id = self.strings.register_string(s);
        up_use(self.strings.get_mut_item(id), which);
        id
    }
    fn add_triple_si_oi(&mut self, s: &Rc<String>, p: &Rc<String>) {
        let s = self.register_string(s, Which::Subject);
        let p = self.register_string(p, Which::Predicate);
        self.triples.insert(Triple {
            subject: s,
            predicate: p,
        });
    }
}

#[test]
fn insert() {
    let mut graph = IndexGraph::new();
}
