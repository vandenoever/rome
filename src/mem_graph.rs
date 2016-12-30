use std::collections::btree_map::Entry;
use std::collections::btree_set;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use rand;
use graph;
use graph::Triple;
use std::rc::Rc;
#[cfg(test)]
use graph::{Graph, WritableGraph};

pub type BlankNode = (usize, usize);

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct MemGraphTriple {
    pub subject: Subject,
    pub predicate: Rc<String>,
    pub object: Object,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub enum Subject {
    IRI(Rc<String>),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct Literal {
    pub lexical: Rc<String>,
    pub datatype: Rc<String>,
    pub language: Option<Rc<String>>,
}

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum Object {
    IRI(Rc<String>),
    BlankNode(BlankNode),
    Literal(Literal),
}

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
    triples: BTreeSet<MemGraphTriple>,
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

impl MemGraphTriple {
    fn new<T>(triple: &T) -> MemGraphTriple
        where T: graph::Triple
    {
        MemGraphTriple {
            subject: match triple.subject() {
                graph::Subject::IRI(iri) => Subject::IRI(Rc::new(String::from(iri))),
                graph::Subject::BlankNode(n) => Subject::BlankNode(n),
            },
            predicate: Rc::new(String::from(triple.predicate())),
            object: match triple.object() {
                graph::Object::IRI(iri) => Object::IRI(Rc::new(String::from(iri))),
                graph::Object::BlankNode(n) => Object::BlankNode(n),
                graph::Object::Literal(ref l) => {
                    Object::Literal(Literal {
                        lexical: Rc::new(String::from(l.lexical)),
                        datatype: Rc::new(String::from(l.datatype)),
                        language: l.language
                            .map(|l| Rc::new(String::from(l))),
                    })
                }
            },
        }
    }
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
    fn register_string(&mut self, str: &str, which: Which) -> Rc<String> {
        match self.strings.entry(Rc::new(String::from(str))) {
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
    fn register_subject(&mut self, subject: &graph::Subject) -> Subject {
        match *subject {
            graph::Subject::IRI(ref iri) => Subject::IRI(self.register_string(iri, Which::Subject)),
            graph::Subject::BlankNode(b) => Subject::BlankNode(b),
        }
    }
    fn register_object(&mut self, object: &graph::Object) -> Object {
        match *object {
            graph::Object::IRI(ref iri) => Object::IRI(self.register_string(iri, Which::Object)),
            graph::Object::BlankNode(b) => Object::BlankNode(b),
            graph::Object::Literal(ref l) => {
                Object::Literal(Literal {
                    lexical: self.register_string(&l.lexical, Which::Literal),
                    datatype: self.register_string(&l.datatype, Which::Datatype),
                    language: l.language.as_ref().map(|l| self.register_string(l, Which::LangTag)),
                })
            }
        }
    }
    fn register_triple<T>(&mut self, triple: &T) -> MemGraphTriple
        where T: graph::Triple
    {
        MemGraphTriple {
            subject: self.register_subject(&triple.subject()),
            predicate: self.register_string(&triple.predicate(), Which::Predicate),
            object: self.register_object(&triple.object()),
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
                literal.language.as_ref().map(|l| self.unregister_string(l, Which::LangTag));
            }
        }
    }
}

impl<'a> graph::Graph<'a> for MemGraph {
    type Triple = &'a MemGraphTriple;
    fn iter(&'a self) -> Box<Iterator<Item = Self::Triple> + 'a> {
        let i = TripleIterator { iter: self.triples.iter() };
        Box::new(i)
    }
    fn len(&self) -> usize {
        self.triples.len()
    }
}

impl graph::WritableGraph for MemGraph {
    fn add_triple_si_oi(&mut self, s: &String, p: &String, o: &String) {
        let s = self.register_string(s, Which::Subject);
        let p = self.register_string(p, Which::Predicate);
        let o = self.register_string(o, Which::Object);
        self.triples.insert(MemGraphTriple {
            subject: Subject::IRI(s),
            predicate: p,
            object: Object::IRI(o),
        });
    }
    fn add_triple<T>(&mut self, triple: &T)
        where T: graph::Triple
    {
        let t = self.register_triple(triple);
        self.triples.insert(t);
    }
    fn remove_triple<T>(&mut self, triple: &T)
        where T: graph::Triple
    {
        let triple = MemGraphTriple::new(triple);
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
}

pub struct TripleIterator<'a> {
    iter: btree_set::Iter<'a, MemGraphTriple>,
}

impl Triple for MemGraphTriple {
    fn subject(&self) -> graph::Subject {
        match self.subject {
            Subject::IRI(ref iri) => graph::Subject::IRI(iri.as_str()),
            Subject::BlankNode(n) => graph::Subject::BlankNode(n),
        }
    }
    fn predicate(&self) -> &str {
        self.predicate.as_str()
    }
    fn object(&self) -> graph::Object {
        match self.object {
            Object::IRI(ref iri) => graph::Object::IRI(iri.as_str()),
            Object::BlankNode(n) => graph::Object::BlankNode(n),
            Object::Literal(ref l) => {
                graph::Object::Literal(graph::Literal {
                    lexical: l.lexical.as_str(),
                    datatype: l.datatype.as_str(),
                    language: l.language.as_ref().map(|l| l.as_str()),
                })
            }
        }
    }
}

impl<'a> Triple for &'a MemGraphTriple {
    fn subject(&self) -> graph::Subject {
        match self.subject {
            Subject::IRI(ref iri) => graph::Subject::IRI(iri.as_str()),
            Subject::BlankNode(n) => graph::Subject::BlankNode(n),
        }
    }
    fn predicate(&self) -> &str {
        self.predicate.as_str()
    }
    fn object(&self) -> graph::Object {
        match self.object {
            Object::IRI(ref iri) => graph::Object::IRI(iri.as_str()),
            Object::BlankNode(n) => graph::Object::BlankNode(n),
            Object::Literal(ref l) => {
                graph::Object::Literal(graph::Literal {
                    lexical: l.lexical.as_str(),
                    datatype: l.datatype.as_str(),
                    language: l.language.as_ref().map(|l| l.as_str()),
                })
            }
        }
    }
}

impl<'a> Iterator for TripleIterator<'a> {
    type Item = &'a MemGraphTriple;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[test]
fn iter() {
    let a = MemGraph::new();
    let mut b = MemGraph::new();
    let it = a.iter();
    for i in it {
        b.add_triple(i);
        b.remove_triple(i);
    }
}

// impl<'a, T> FromIterator<T> for MemGraph
//    where T: Triple
//
//    fn from_iter<I>(iter: I) -> Self
//        where I: IntoIterator<Item = T>
//    {
//        let mut g = MemGraph::new();
//        for triple in iter {
//            g.add_triple(&triple);
//        }
//        g
//    }
//
// [test]
// fn from_iter() {
//    let a = MemGraph::new();
//    let b = MemGraph::from_iter(a.iter());
//    assert_eq!(b.len(), 0);
//
