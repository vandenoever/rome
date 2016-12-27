pub trait Graph {
    type Triple: Triple;
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Triple> + 'a>;
    /// return the number of triples in the graph
    fn len(&self) -> usize;
}

pub trait WritableGraph {
    fn add_triple_si_oi(&mut self, s: &String, p: &String, o: &String);
    /// Add a new triple
    /// This can fail if an incoming blank node is invalid
    fn add_triple<T>(&mut self, triple: &T) where T: Triple;
    fn remove_triple<T>(&mut self, triple: &T) where T: Triple;
    fn create_blank_node(&mut self) -> BlankNode;
    // Retains only the triples specified by the function.
    // fn retain<F>(&mut self, f: F) where F: FnMut(&Triple) -> bool;
}

pub type BlankNode = (usize, usize);

pub trait Triple {
    fn subject(&self) -> Subject;
    fn predicate(&self) -> &str;
    fn object(&self) -> Object;
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub enum Subject<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub struct Literal<'a> {
    pub lexical: &'a str,
    pub datatype: &'a str,
    pub language: Option<&'a str>,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub enum Object<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
    Literal(Literal<'a>),
}
