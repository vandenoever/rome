use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_set;
use graph::*;
use std::rc::Rc;
use std::borrow::Borrow;
use std::hash::{Hash, Hasher};
use rand;

struct StringKey {
    key: Rc<String>,
}

impl Hash for StringKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state)
    }
}
impl PartialEq for StringKey {
    fn eq(&self, rhs: &StringKey) -> bool {
        self.key.eq(&rhs.key)
    }
}
impl Eq for StringKey {}

impl Borrow<str> for StringKey {
    fn borrow(&self) -> &str {
        self.key.as_str()
    }
}
impl Borrow<Rc<String>> for StringKey {
    fn borrow(&self) -> &Rc<String> {
        &self.key
    }
}

struct Iri {
    iri: Rc<String>,
    subject_count: usize,
    predicate_count: usize,
    object_count: usize,
}

impl Iri {
    fn new(iri: &str) -> Iri {
        Iri {
            iri: Rc::new(String::from(iri)),
            subject_count: 0,
            predicate_count: 0,
            object_count: 0,
        }
    }
}

struct Literal {
    literal: Rc<String>,
    count: usize,
}
impl Literal {
    fn new(literal: &str) -> Literal {
        Literal {
            literal: Rc::new(String::from(literal)),
            count: 0,
        }
    }
}

enum Which {
    Subject,
    Predicate,
    Object,
}

type MemTriple = (Node1, Rc<String>, Node2);

pub struct MemGraph {
    iris: HashMap<StringKey, Iri>,
    literals: HashMap<StringKey, Literal>,
    graph_id: usize,
    triples: HashSet<MemTriple>,
    blank_node_count: usize,
}

fn up_use(iri: &mut Iri, which: Which) {
    match which {
        Which::Subject => iri.subject_count += 1,
        Which::Predicate => iri.predicate_count += 1,
        Which::Object => iri.object_count += 1,
    }
}

fn use_count(iri: &Iri) -> usize {
    iri.subject_count + iri.predicate_count + iri.object_count
}

impl MemGraph {
    pub fn new() -> MemGraph {
        MemGraph {
            iris: HashMap::new(),
            literals: HashMap::new(),
            graph_id: rand::random::<usize>(),
            triples: HashSet::new(),
            blank_node_count: 0,
        }
    }
    fn register_iri(&mut self, iri: &str, which: Which) -> Rc<String> {
        if let Some(iri) = self.iris.get_mut(iri) {
            up_use(iri, which);
            return iri.iri.clone();
        }
        let mut value = Iri::new(iri);
        up_use(&mut value, which);
        let r = value.iri.clone();
        self.iris.insert(StringKey { key: value.iri.clone() }, value);
        r
    }
    fn register_literal(&mut self, literal: &str) -> Rc<String> {
        if let Some(literal) = self.literals.get_mut(literal) {
            literal.count += 1;
            return literal.literal.clone();
        }
        let value = Literal::new(literal);
        let r = value.literal.clone();
        self.literals.insert(StringKey { key: value.literal.clone() }, value);
        r
    }
    fn from_subject(&mut self, subject: &Subject) -> Node1 {
        match *subject {
            Subject::IRI(iri) => Node1::IRI(self.register_iri(iri, Which::Subject)),
            Subject::BlankNode((n, _)) => Node1::BlankNode(n),
        }
    }
    fn from_object(&mut self, object: &Object) -> Node2 {
        match *object {
            Object::IRI(iri) => Node2::IRI(self.register_iri(iri, Which::Object)),
            Object::BlankNode((n, _)) => Node2::BlankNode(n),
            Object::Literal(l) => Node2::Literal(self.register_literal(l)),
        }
    }
    fn to_subject<'a>(&self, s: &'a Node1) -> Subject<'a> {
        match *s {
            Node1::IRI(ref n) => Subject::IRI(n.as_str()),
            Node1::BlankNode(n) => Subject::BlankNode((n, self.graph_id)),
        }
    }
    fn to_object<'a>(&self, s: &'a Node2) -> Object<'a> {
        match *s {
            Node2::IRI(ref n) => Object::IRI(n.as_str()),
            Node2::BlankNode(n) => Object::BlankNode((n, self.graph_id)),
            Node2::Literal(ref n) => Object::Literal(n.as_str()),
        }
    }
    fn to_triple<'a>(&self, o: &'a MemTriple) -> Triple<'a> {
        Triple {
            subject: self.to_subject(&o.0),
            predicate: &o.1.as_str(),
            object: self.to_object(&o.2),
        }
    }
    fn as_subject(&self, subject: &Subject) -> Option<Node1> {
        match *subject {
            Subject::IRI(iri) => {
                if let Some(iri) = self.iris.get(iri) {
                    Some(Node1::IRI(iri.iri.clone()))
                } else {
                    None
                }
            }
            Subject::BlankNode((n, graph)) if self.graph_id == graph => Some(Node1::BlankNode(n)),
            _ => None,
        }
    }
    fn as_predicate(&self, predicate: &str) -> Option<Rc<String>> {
        if let Some(iri) = self.iris.get(predicate) {
            Some(iri.iri.clone())
        } else {
            None
        }
    }
    fn as_object(&self, object: &Object) -> Option<Node2> {
        match *object {
            Object::IRI(iri) => {
                if let Some(iri) = self.iris.get(iri) {
                    Some(Node2::IRI(iri.iri.clone()))
                } else {
                    None
                }
            }
            Object::BlankNode((n, graph)) if self.graph_id == graph => Some(Node2::BlankNode(n)),
            Object::Literal(literal) => {
                if let Some(literal) = self.literals.get(literal) {
                    Some(Node2::IRI(literal.literal.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn find_triple(&self, triple: &Triple) -> Option<MemTriple> {
        if let Some(subject) = self.as_subject(&triple.subject) {
            if let Some(predicate) = self.as_predicate(triple.predicate) {
                if let Some(object) = self.as_object(&triple.object) {
                    let triple = (subject, predicate, object);
                    if self.triples.contains(&triple) {
                        return Some(triple);
                    }
                }
            }
        }
        None
    }
    fn remove_subject(&mut self, subject: &Node1) {
        match *subject {
            Node1::IRI(ref iri) => {
                if let Some(i) = self.iris.get_mut(iri) {
                    if use_count(i) > 1 {
                        i.subject_count -= 1;
                        return;
                    }
                }
                self.iris.remove(iri).unwrap();
            }
            Node1::BlankNode(_) => {}
        }
    }
    fn remove_predicate(&mut self, predicate: &Rc<String>) {
        if let Some(i) = self.iris.get_mut(predicate) {
            if use_count(i) > 1 {
                i.predicate_count -= 1;
                return;
            }
        }
        self.iris.remove(predicate).unwrap();
    }
    fn remove_object(&mut self, object: &Node2) {
        match *object {
            Node2::IRI(ref iri) => {
                if let Some(i) = self.iris.get_mut(iri) {
                    if use_count(i) > 1 {
                        i.object_count -= 1;
                        return;
                    }
                }
                self.iris.remove(iri).unwrap();
            }
            Node2::BlankNode(_) => {}
            Node2::Literal(ref literal) => {
                if let Some(l) = self.literals.get_mut(literal) {
                    if l.count > 1 {
                        l.count -= 1;
                        return;
                    }
                }
                self.literals.remove(literal).unwrap();
            }
        }
    }
}

#[derive(PartialEq,Eq,Hash)]
enum Node1 {
    IRI(Rc<String>),
    BlankNode(usize),
}
#[derive(PartialEq,Eq,Hash)]
enum Node2 {
    IRI(Rc<String>),
    BlankNode(usize),
    Literal(Rc<String>),
}

impl Graph for MemGraph {
    fn add_triple_si_oi(&mut self, s: &str, p: &str, o: &str) {
        let s = self.register_iri(s, Which::Subject);
        let p = self.register_iri(p, Which::Predicate);
        let o = self.register_iri(o, Which::Object);
        self.triples.insert((Node1::IRI(s), p, Node2::IRI(o)));
    }
    fn add_triple(&mut self, triple: &Triple) {
        let p = self.register_iri(triple.predicate, Which::Predicate);
        let t = (self.from_subject(&triple.subject), p, self.from_object(&triple.object));
        self.triples.insert(t);
    }
    fn remove_triple(&mut self, triple: &Triple) {
        if let Some(triple) = self.find_triple(triple) {
            self.triples.remove(&triple); // TODO: dimish use
            self.remove_subject(&triple.0);
            self.remove_predicate(&triple.1);
            self.remove_object(&triple.2);
        }
    }
    fn iter<'b>(&'b self) -> Box<Iterator<Item = Triple> + 'b> {
        Box::new(TripleIterator {
            graph: self,
            iter: self.triples.iter(),
        })
    }
    fn create_blank_node(&mut self) -> BlankNode {
        let bn = (self.blank_node_count, self.graph_id);
        self.blank_node_count += 1;
        bn
    }
}

pub struct TripleIterator<'a> {
    graph: &'a MemGraph,
    iter: hash_set::Iter<'a, MemTriple>,
}
impl<'a> Iterator for TripleIterator<'a> {
    type Item = Triple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|r| Some(self.graph.to_triple(r)))
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
