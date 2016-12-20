
pub trait Graph {
    fn add_triple_si_oi(&mut self, s: &str, p: &str, o: &str);
    /// Add a new triple
    /// This can fail if an incoming blank node is invalid
    fn add_triple(&mut self, triple: &Triple);
    fn remove_triple(&mut self, triple: &Triple);
    fn iter<'a>(&'a self) -> Box<Iterator<Item = Triple> + 'a>;
    fn create_blank_node(&mut self) -> BlankNode;
}

pub type BlankNode = (usize, usize);

pub struct Triple<'a> {
    pub subject: Subject<'a>,
    pub predicate: &'a str,
    pub object: Object<'a>,
}

pub enum Subject<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
}

pub enum Object<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
    Literal(&'a str),
}
