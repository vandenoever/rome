pub trait Graph {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Triple> + 'a>;
    /// return the number of triples in the graph
    fn len(&self) -> usize;
}

pub trait WritableGraph: Graph {
    fn add_triple_si_oi(&mut self, s: &String, p: &String, o: &String);
    /// Add a new triple
    /// This can fail if an incoming blank node is invalid
    fn add_triple(&mut self, triple: &Triple);
    fn remove_triple(&mut self, triple: &Triple);
    fn create_blank_node(&mut self) -> BlankNode;
    // Retains only the triples specified by the function.
    // fn retain<F>(&mut self, f: F) where F: FnMut(&Triple) -> bool;
}

pub type BlankNode = (usize, usize);

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: String,
    pub object: Object,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub enum Subject {
    IRI(String),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub struct Literal {
    pub lexical: String,
    pub datatype: String,
    pub language: Option<String>,
}

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
pub enum Object {
    IRI(String),
    BlankNode(BlankNode),
    Literal(Literal),
}
