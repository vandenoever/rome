
pub trait Graph {
    fn add_triple_si_oi(&mut self, s: &str, p: &str, o: &str);
    fn add_triple(&mut self, triple: &Triple);
    fn get_triple(&self) -> Option<Triple>;
}

pub struct Triple<'a> {
    pub subject: Subject<'a>,
    pub predicate: &'a str,
    pub object: Object<'a>,
}

pub enum Subject<'a> {
    IRI(&'a str),
    BlankNode(usize, usize),
}

pub enum Object<'a> {
    IRI(&'a str),
    BlankNode(usize, usize),
    Literal(&'a str),
}
