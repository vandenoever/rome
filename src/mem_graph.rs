use std::collections::btree_map::Entry;
use std::collections::btree_set;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use graph::*;
use std::rc::Rc;
use rand;
use std::iter::FromIterator;

struct StringUsage {
    subject_count: usize,
    predicate_count: usize,
    object_count: usize,
    datatype_count: usize,
    literal_count: usize,
    langtag_count: usize,
}

impl StringUsage {
    fn new() -> StringUsage {
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

struct Blank {
    subject_count: usize,
    object_count: usize,
}

enum Which {
    Subject,
    Predicate,
    Object,
    Datatype,
    Literal,
    LangTag,
}

pub struct MemGraph {
    strings: BTreeMap<Rc<String>, StringUsage>,
    blanks: Vec<Blank>,
    unused_blanks: Vec<usize>,
    graph_id: usize,
    triples: BTreeSet<Triple>,
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

fn down_use(iri: &mut StringUsage, which: Which) {
    match which {
        Which::Subject => iri.subject_count -= 1,
        Which::Predicate => iri.predicate_count -= 1,
        Which::Object => iri.object_count -= 1,
        Which::Datatype => iri.datatype_count -= 1,
        Which::Literal => iri.literal_count -= 1,
        Which::LangTag => iri.langtag_count -= 1,
    }
}

fn use_count(u: &StringUsage) -> usize {
    u.subject_count + u.predicate_count + u.object_count + u.datatype_count + u.literal_count +
    u.langtag_count
}

fn blank_use_count(blank: &Blank) -> usize {
    blank.subject_count + blank.object_count
}

impl MemGraph {
    pub fn new() -> MemGraph {
        MemGraph {
            strings: BTreeMap::new(),
            blanks: Vec::new(),
            unused_blanks: Vec::new(),
            graph_id: rand::random::<usize>(),
            triples: BTreeSet::new(),
        }
    }
    /// deduplicate the string
    /// look up the string in the map and pass back the string from the map
    fn register_string(&mut self, str: &Rc<String>, which: Which) -> Rc<String> {
        match self.strings.entry(str.clone()) {
            Entry::Occupied(ref mut o) => {
                up_use(o.get_mut(), which);
                return o.key().clone();
            }
            Entry::Vacant(v) => {
                let key = v.key().clone();
                let usage = v.insert(StringUsage::new());
                up_use(usage, which);
                return key;
            }
        }
    }
    fn unregister_string(&mut self, str: &Rc<String>, which: Which) {
        match self.strings.entry(str.clone()) {
            Entry::Occupied(mut o) => {
                if use_count(o.get()) > 0 {
                    down_use(o.get_mut(), which);
                } else {
                    o.remove_entry();
                }
            }
            Entry::Vacant(_) => {}
        }
    }
    fn register_subject(&mut self, subject: &Subject) -> Subject {
        match *subject {
            Subject::IRI(ref iri) => Subject::IRI(self.register_string(iri, Which::Subject)),
            ref s => s.clone(),
        }
    }
    fn register_literal_extra(&mut self, extra: &LiteralExtra) -> LiteralExtra {
        match *extra {
            LiteralExtra::LanguageTag(ref langtag) => {
                LiteralExtra::LanguageTag(self.register_string(langtag, Which::LangTag))
            }
            ref e => e.clone(),
        }
    }
    fn register_object(&mut self, object: &Object) -> Object {
        match *object {
            Object::IRI(ref iri) => Object::IRI(self.register_string(iri, Which::Object)),
            Object::BlankNode(b) => Object::BlankNode(b),
            Object::Literal(ref l) => {
                Object::Literal(Literal {
                    lexical: self.register_string(&l.lexical, Which::Literal),
                    datatype: self.register_string(&l.datatype, Which::Datatype),
                    extra: self.register_literal_extra(&l.extra),
                })
            }
        }
    }
    fn register_triple(&mut self, triple: &Triple) -> Triple {
        Triple {
            subject: self.register_subject(&triple.subject),
            predicate: self.register_string(&triple.predicate, Which::Predicate),
            object: self.register_object(&triple.object),
        }
    }
    fn unregister_subject(&mut self, subject: &Subject) {
        match *subject {
            Subject::IRI(ref iri) => {
                self.unregister_string(iri, Which::Subject);
            }
            Subject::BlankNode(n) => {
                let b = &mut self.blanks[n.0];
                b.subject_count -= 1;
                if blank_use_count(b) == 0 {
                    self.unused_blanks.push(n.0);
                }
            }
        }
    }
    fn unregister_predicate(&mut self, predicate: &Rc<String>) {
        self.unregister_string(predicate, Which::Predicate);
    }
    fn unregister_literal_extra(&mut self, extra: &LiteralExtra) {
        match *extra {
            LiteralExtra::LanguageTag(ref langtag) => {
                self.unregister_string(langtag, Which::LangTag);
            }
            _ => (),
        }
    }
    fn unregister_object(&mut self, object: &Object) {
        match *object {
            Object::IRI(ref iri) => {
                self.unregister_string(iri, Which::Object);
            }
            Object::BlankNode(n) => {
                let b = &mut self.blanks[n.0];
                b.object_count -= 1;
                if blank_use_count(b) == 0 {
                    self.unused_blanks.push(n.0);
                }
            }
            Object::Literal(ref literal) => {
                self.unregister_string(&literal.lexical, Which::Literal);
                self.unregister_string(&literal.datatype, Which::Datatype);
                self.unregister_literal_extra(&literal.extra);
            }
        }
    }
}

impl Graph for MemGraph {
    fn iter<'b>(&'b self) -> Box<Iterator<Item = &Triple> + 'b> {
        Box::new(TripleIterator {
            iter: self.triples.iter(),
        })
    }
    fn len(&self) -> usize {
        self.triples.len()
    }
}

impl WritableGraph for MemGraph {
    fn add_triple_si_oi(&mut self, s: &Rc<String>, p: &Rc<String>, o: &Rc<String>) {
        let s = self.register_string(s, Which::Subject);
        let p = self.register_string(p, Which::Predicate);
        let o = self.register_string(o, Which::Object);
        self.triples.insert(Triple {
            subject: Subject::IRI(s),
            predicate: p,
            object: Object::IRI(o),
        });
    }
    fn add_triple(&mut self, triple: &Triple) {
        let t = self.register_triple(&triple);
        self.triples.insert(t);
    }
    fn remove_triple(&mut self, triple: &Triple) {
        if self.triples.remove(&triple) {
            self.unregister_subject(&triple.subject);
            self.unregister_predicate(&triple.predicate);
            self.unregister_object(&triple.object);
        }
    }
    fn create_blank_node(&mut self) -> BlankNode {
        if let Some(blank) = self.unused_blanks.pop() {
            let b = &self.blanks[blank];
            assert!(b.subject_count == 0 && b.object_count == 0);
            return (blank, self.graph_id);
        }
        let bn = (self.blanks.len(), self.graph_id);
        self.blanks.push(Blank {
            subject_count: 0,
            object_count: 0,
        });
        bn
    }
    // fn retain<F>(&mut self, f: F)
    // where F: FnMut(&Triple) -> bool
    // {
    // TODO
    // }
    //
}

pub struct TripleIterator<'a> {
    iter: btree_set::Iter<'a, Triple>,
}
impl<'a> Iterator for TripleIterator<'a> {
    type Item = &'a Triple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> FromIterator<&'a Triple> for MemGraph {
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item = &'a Triple>
    {
        let mut g = MemGraph::new();
        for triple in iter {
            g.add_triple(&triple);
        }
        g
    }
}

#[test]
fn iter() {
    let a = MemGraph::new();
    let mut b = MemGraph::new();
    let it = a.iter();
    for i in it {
        b.add_triple(&i);
        b.remove_triple(&i);
    }
}

#[test]
fn from_iter() {
    let a = MemGraph::new();
    let b = MemGraph::from_iter(a.iter());
}
