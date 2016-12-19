use std::collections::HashSet;
use std::collections::hash_set;
use graph::*;

pub struct MemGraph {
    strings: Vec<String>,
    graph_id: usize,
    triples: HashSet<(Node1, usize, Node2)>,
}

impl MemGraph {
    pub fn new() -> MemGraph {
        MemGraph {
            strings: Vec::new(),
            graph_id: 0,
            triples: HashSet::new(),
        }
    }
    fn find_string(&self, iri: &str) -> Option<usize> {
        self.strings.iter().position(|i| i == iri)
    }
    fn insert_string(&mut self, iri: &str) -> usize {
        match self.find_string(iri) {
            Some(pos) => pos,
            None => {
                self.strings.push(String::from(iri));
                self.strings.len() - 1
            }
        }
    }
    fn from_subject(&mut self, subject: &Subject) -> Node1 {
        match *subject {
            Subject::IRI(iri) => Node1::IRI(self.insert_string(iri)),
            Subject::BlankNode(n, _) => Node1::BlankNode(n),
        }
    }
    fn from_object(&mut self, object: &Object) -> Node2 {
        match *object {
            Object::IRI(iri) => Node2::IRI(self.insert_string(iri)),
            Object::BlankNode(n, _) => Node2::BlankNode(n),
            Object::Literal(l) => Node2::Literal(self.insert_string(l)),
        }
    }
    fn to_subject(&self, s: &Node1) -> Subject {
        match *s {
            Node1::IRI(n) => Subject::IRI(self.strings[n].as_str()),
            Node1::BlankNode(n) => Subject::BlankNode(n, self.graph_id),
        }
    }
    fn to_object(&self, s: &Node2) -> Object {
        match *s {
            Node2::IRI(n) => Object::IRI(self.strings[n].as_str()),
            Node2::BlankNode(n) => Object::BlankNode(n, self.graph_id),
            Node2::Literal(n) => Object::Literal(self.strings[n].as_str()),
        }
    }
    fn to_triple(&self, o: &(Node1, usize, Node2)) -> Triple {
        Triple {
            subject: self.to_subject(&o.0),
            predicate: self.strings[o.1].as_str(),
            object: self.to_object(&o.2),
        }
    }
}

#[derive(PartialEq,Eq,Hash)]
enum Node1 {
    IRI(usize),
    BlankNode(usize),
}
#[derive(PartialEq,Eq,Hash)]
enum Node2 {
    IRI(usize),
    BlankNode(usize),
    Literal(usize),
}

impl<'a> Graph for MemGraph {
    fn add_triple_si_oi(&mut self, s: &str, p: &str, o: &str) {
        let s = self.insert_string(s);
        let p = self.insert_string(p);
        let o = self.insert_string(o);
        self.triples.insert((Node1::IRI(s), p, Node2::IRI(o)));
    }
    fn add_triple(&mut self, triple: &Triple) {
        let p = self.insert_string(triple.predicate);
        let t = (self.from_subject(&triple.subject), p, self.from_object(&triple.object));
        self.triples.insert(t);
    }
    fn iter<'b>(&'b self) -> Box<Iterator<Item = Triple> + 'b> {
        Box::new(TripleIterator {
            graph: self,
            iter: self.triples.iter(),
        })
    }
}

pub struct TripleIterator<'a> {
    graph: &'a MemGraph,
    iter: hash_set::Iter<'a, (Node1, usize, Node2)>,
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
    }
}
