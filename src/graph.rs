use std::rc::Rc;

pub trait Graph {
    fn add_triple_si_oi(&mut self, s: &Rc<String>, p: &Rc<String>, o: &Rc<String>);
    /// Add a new triple
    /// This can fail if an incoming blank node is invalid
    fn add_triple(&mut self, triple: &Triple);
    fn remove_triple(&mut self, triple: &Triple);
    fn iter<'a>(&'a self) -> Box<Iterator<Item = Triple> + 'a>;
    fn create_blank_node(&mut self) -> BlankNode;
    /// Retains only the triples specified by the function.
    fn retain<F>(&mut self, f: F) where F: FnMut(&Triple) -> bool;
}

pub type BlankNode = (usize, usize);

pub struct Triple {
    pub subject: Subject,
    pub predicate: Rc<String>,
    pub object: Object,
}

pub enum Subject {
    IRI(Rc<String>),
    BlankNode(BlankNode),
}

pub enum Object {
    IRI(Rc<String>),
    BlankNode(BlankNode),
    Literal(Rc<String>),
}
