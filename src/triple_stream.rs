use grammar_structs::*;
use grammar::*;
use std::collections::HashMap;
use nom::IResult;
use graph;
use std::rc::Rc;

struct StatementIterator<'a> {
    src: &'a str,
    done: bool,
}

impl<'a> StatementIterator<'a> {
    pub fn new(src: &str) -> Result<StatementIterator, String> {
        match tws(src) {
            IResult::Done(left, _) => {
                Ok(StatementIterator {
                    src: left,
                    done: false,
                })
            }
            IResult::Error(e) => return Err(String::from("cannot start parsing")),
            IResult::Incomplete(_) => {
                return Ok(StatementIterator {
                    src: src,
                    done: false,
                })
            }
        }
    }
}

impl<'a> Iterator for StatementIterator<'a> {
    type Item = Result<Statement, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let mut r;
        match statement(self.src) {
            IResult::Done(left, s) => {
                r = Some(Ok(s));
                self.src = left;
            }
            IResult::Error(_) => {
                r = Some(Err(String::from("error parsing")));
                self.done = true;
            }
            IResult::Incomplete(_) => {
                self.done = true;
                r = None;
            }
        }
        match tws(self.src) {
            IResult::Done(left, _) => {
                self.src = left;
            }
            IResult::Error(_) => {
                r = Some(Err(String::from("error parsing")));
                self.done = true;
            }
            IResult::Incomplete(_) => {
                self.done = true;
            }
        }
        r
    }
}


pub struct TripleIterator<'a> {
    statement_iterator: StatementIterator<'a>,
    base: String,
    prefixes: HashMap<String, String>,
    blank_nodes: HashMap<String, graph::BlankNode>,
    triple_buffer: Vec<graph::Triple>,
    next_blank: usize,
    done: bool,
}

impl<'a> TripleIterator<'a> {
    pub fn new(src: &str) -> Result<TripleIterator, String> {
        Ok(TripleIterator {
            statement_iterator: try!(StatementIterator::new(src)),
            base: String::new(),
            prefixes: HashMap::new(),
            blank_nodes: HashMap::new(),
            triple_buffer: Vec::new(),
            next_blank: 0,
            done: false,
        })
    }
    pub fn prefixes(&self) -> &HashMap<String, String> {
        &self.prefixes
    }
    fn set_prefix(&mut self, prefix: String, value: String) {
        let value = self.resolve_iri_ref(&value);
        self.prefixes.insert(prefix, value);
    }
    fn fill_buffer(&mut self) -> Result<usize, String> {
        while let Some(statement) = self.statement_iterator.next() {
            match statement {
                Ok(Statement::Prefix(prefix, iri)) => {
                    self.set_prefix(prefix, iri);
                }
                Ok(Statement::Base(new_base)) => {
                    self.base = new_base;
                }
                Ok(Statement::Triples(new_triples)) => {
                    try!(add_triples(new_triples, self));
                }
                Err(e) => return Err(e),
            }
        }
        Ok(0)
    }
    fn resolve_iri_ref(&self, iri: &String) -> String {
        iri.clone()
    }
    fn add_triple(&mut self, triple: graph::Triple) {
        self.triple_buffer.push(triple)
    }
    fn new_blank(&mut self) -> (usize, usize) {
        let b = self.next_blank;
        self.next_blank += 1;
        (b, 0)
    }
    fn get_blank(&mut self, label: String) -> graph::BlankNode {
        if let Some(n) = self.blank_nodes.get(&label) {
            return *n;
        }
        let n = self.new_blank();
        self.blank_nodes.insert(label, n);
        n
    }
}

fn resolve_iri(iri: &IRI, state: &TripleIterator) -> Result<Rc<String>, String> {
    let i = match *iri {
        IRI::IRI(ref iri) => state.resolve_iri_ref(iri),
        IRI::PrefixedName(ref prefix, ref local) => {
            let base = match state.prefixes.get(prefix) {
                Some(base) => base.clone(),
                None => return Err(format!("Prefix {} was not defined.", prefix)),
            };
            base + local
        }
    };
    Ok(Rc::new(i))
}

fn make_blank(blank_node: BlankNode, state: &mut TripleIterator) -> graph::BlankNode {
    match blank_node {
        BlankNode::Anon => state.new_blank(),
        BlankNode::BlankNode(label) => state.get_blank(label),
    }
}

fn r(str: &str) -> Rc<String> {
    Rc::new(String::from(str))
}
fn rdf_first() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#first")
}
fn rdf_rest() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest")
}
fn rdf_nil() -> Rc<String> {
    r("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil")
}

fn make_collection(collection: Vec<Object>,
                   state: &mut TripleIterator)
                   -> Result<graph::BlankNode, String> {
    let head = state.new_blank();

    let mut node = head;
    for object in collection {
        let o = try!(make_object(object, state));
        state.add_triple(graph::Triple {
            subject: graph::Subject::BlankNode(node),
            predicate: rdf_first(),
            object: o,
        });
        let next = state.new_blank();
        state.add_triple(graph::Triple {
            subject: graph::Subject::BlankNode(node),
            predicate: rdf_rest(),
            object: graph::Object::BlankNode(next),
        });
        node = next;
    }
    state.add_triple(graph::Triple {
        subject: graph::Subject::BlankNode(node),
        predicate: rdf_rest(),
        object: graph::Object::IRI(rdf_nil()),
    });
    Ok(head)
}

fn make_subject(subject: Subject, state: &mut TripleIterator) -> Result<graph::Subject, String> {
    Ok(match subject {
        Subject::IRI(iri) => {
            let iri = try!(resolve_iri(&iri, state));
            graph::Subject::IRI(iri)
        }
        Subject::BlankNode(blank) => graph::Subject::BlankNode(make_blank(blank, state)),
        Subject::Collection(collection) => {
            graph::Subject::BlankNode(try!(make_collection(collection, state)))
        }
    })
}

fn make_object(object: Object, state: &mut TripleIterator) -> Result<graph::Object, String> {
    Ok(match object {
        Object::IRI(ref iri) => graph::Object::IRI(try!(resolve_iri(&iri, state))),
        Object::BlankNode(blank) => graph::Object::BlankNode(make_blank(blank, state)),
        Object::Collection(collection) => {
            graph::Object::BlankNode(try!(make_collection(collection, state)))
        }
        Object::Literal(l) => {
            graph::Object::Literal(graph::Literal {
                lexical: Rc::new(l.lexical),
                datatype: try!(resolve_iri(&l.datatype, state)),
                language: l.language.map(Rc::new),
            })
        }
        Object::BlankNodePropertyList(predicated_objects_list) => {
            let blank = state.new_blank();
            let subject = graph::Subject::BlankNode(blank);
            try!(add_predicated_objects(subject, predicated_objects_list, state));
            graph::Object::BlankNode(blank)
        }
    })
}

fn add_predicated_objects(subject: graph::Subject,
                          predicated_objects_list: Vec<PredicatedObjects>,
                          state: &mut TripleIterator)
                          -> Result<(), String> {
    for po in predicated_objects_list {
        for o in po.objects.into_iter() {
            let triple = graph::Triple {
                subject: subject.clone(),
                predicate: try!(resolve_iri(&po.verb, state)),
                object: try!(make_object(o, state)),
            };
            state.add_triple(triple);
        }
    }
    Ok(())
}

fn add_triples(new_triples: Triples, state: &mut TripleIterator) -> Result<(), String> {
    let subject = try!(make_subject(new_triples.subject, state));
    add_predicated_objects(subject, new_triples.predicated_objects_list, state)
}


impl<'a> Iterator for TripleIterator<'a> {
    type Item = Result<graph::Triple, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.triple_buffer.len() == 0 {
            match self.fill_buffer() {
                Ok(0) => {
                    self.done = true;
                    return None;
                }
                Ok(_) => {}
                Err(e) => return Some(Err(e)),
            }
        }
        self.triple_buffer.pop().map(Ok)
    }
}
