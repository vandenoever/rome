use std::collections::HashSet;
use std::collections::hash_set;
use graph::*;

pub struct MemGraph {
    graph_id: usize,
    triples: HashSet<(Node1, String, Node2)>,
}

impl MemGraph {
    pub fn new() -> MemGraph {
        MemGraph {
            graph_id: 0,
            triples: HashSet::new(),
        }
    }
}

#[derive(PartialEq,Eq,Hash)]
enum Node1 {
    IRI(String),
    BlankNode(usize),
}
#[derive(PartialEq,Eq,Hash)]
enum Node2 {
    IRI(String),
    BlankNode(usize),
    Literal(String),
}

fn from_subject(subject: &Subject) -> Node1 {
    match *subject {
        Subject::IRI(str) => Node1::IRI(String::from(str)),
        Subject::BlankNode(n, _) => Node1::BlankNode(n),
    }
}
fn from_object(object: &Object) -> Node2 {
    match *object {
        Object::IRI(str) => Node2::IRI(String::from(str)),
        Object::BlankNode(n, _) => Node2::BlankNode(n),
        Object::Literal(str) => Node2::Literal(String::from(str)),
    }
}

fn to_subject(graph: usize, s: &Node1) -> Subject {
    match *s {
        Node1::IRI(ref str) => Subject::IRI(str),
        Node1::BlankNode(n) => Subject::BlankNode(n, graph),
    }
}
fn to_object(graph: usize, s: &Node2) -> Object {
    match *s {
        Node2::IRI(ref str) => Object::IRI(str),
        Node2::BlankNode(n) => Object::BlankNode(n, graph),
        Node2::Literal(ref str) => Object::Literal(str),
    }
}

fn to_triple(graph_id: usize, o: &(Node1, String, Node2)) -> Triple {
    Triple {
        subject: to_subject(graph_id, &o.0),
        predicate: &o.1,
        object: to_object(graph_id, &o.2),
    }
}

impl Graph for MemGraph {
    fn add_triple_si_oi(&mut self, s: &str, p: &str, o: &str) {
        self.triples
            .insert((Node1::IRI(String::from(s)), String::from(p), Node2::IRI(String::from(o))));
    }
    fn add_triple(&mut self, triple: &Triple) {
        self.triples
            .insert((from_subject(&triple.subject),
                     String::from(triple.predicate),
                     from_object(&triple.object)));
    }
    fn iter<'a>(&'a self) -> Box<Iterator<Item = Triple> + 'a> {
        Box::new(TripleIterator {
            graph_id: self.graph_id,
            iter: self.triples.iter(),
        })
    }
}

pub struct TripleIterator<'a> {
    graph_id: usize,
    iter: hash_set::Iter<'a, (Node1, String, Node2)>,
}
impl<'a> Iterator for TripleIterator<'a> {
    type Item = Triple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|r| Some(to_triple(self.graph_id, r)))
    }
}

#[test]
fn iter() {
    let a = MemGraph::new();
    let mut b = MemGraph::new();
    let it = a.iter();
    for i in it {
        b.add_triple(&i);
    }
}
