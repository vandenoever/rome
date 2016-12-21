use std::rc::Rc;
use std::hash::{Hash, Hasher};

pub trait Graph {
    fn add_triple_si_oi(&mut self, s: &Rc<String>, p: &Rc<String>, o: &Rc<String>);
    /// Add a new triple
    /// This can fail if an incoming blank node is invalid
    fn add_triple(&mut self, triple: &Triple);
    fn remove_triple(&mut self, triple: &Triple);
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Triple> + 'a>;
    fn create_blank_node(&mut self) -> BlankNode;
    // Retains only the triples specified by the function.
    // fn retain<F>(&mut self, f: F) where F: FnMut(&Triple) -> bool;
}

pub type BlankNode = (usize, usize);

#[derive(PartialEq,Eq,Hash)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: Rc<String>,
    pub object: Object,
}

#[derive(PartialEq,Eq,Hash,Clone)]
pub enum Subject {
    IRI(Rc<String>),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash)]
pub struct Literal {
    pub lexical: Rc<String>,
    pub datatype: Rc<String>,
    pub extra: LiteralExtra,
}

#[derive(Clone)]
pub enum LiteralExtra {
    None,
    LanguageTag(Rc<String>),
    XsdInteger(i64),
    XsdDecimal(f64),
    XsdDouble(f64),
    XsdBoolean(bool),
}

#[derive(PartialEq,Eq,Hash)]
pub enum Object {
    IRI(Rc<String>),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl Hash for LiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            LiteralExtra::LanguageTag(ref langtag) => {
                langtag.hash(state);
            }
            _ => {}
        }
    }
}
impl PartialEq for LiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn eq(&self, other: &LiteralExtra) -> bool {
        match *self {
            LiteralExtra::LanguageTag(ref langtag1) => {
                match *other {
                    LiteralExtra::LanguageTag(ref langtag2) => langtag1 == langtag2,
                    _ => true,
                }
            }
            _ => true,
        }
    }
}
impl Eq for LiteralExtra {}
