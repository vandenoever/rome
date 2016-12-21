use std::rc::Rc;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

pub trait Graph {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Triple> + 'a>;
    /// return the number of triples in the graph
    fn len(&self) -> usize;
}

pub trait WritableGraph: Graph {
    fn add_triple_si_oi(&mut self, s: &Rc<String>, p: &Rc<String>, o: &Rc<String>);
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
    pub predicate: Rc<String>,
    pub object: Object,
}

#[derive(PartialEq,Eq,Hash,Clone,PartialOrd,Ord)]
pub enum Subject {
    IRI(Rc<String>),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
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

#[derive(PartialEq,Eq,Hash,PartialOrd,Ord)]
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
        match (self, other) {
            (&LiteralExtra::LanguageTag(ref langtag1),
             &LiteralExtra::LanguageTag(ref langtag2)) => langtag1 == langtag2,
            (&LiteralExtra::LanguageTag(_), _) => false,
            (_, &LiteralExtra::LanguageTag(_)) => false,
            _ => true,
        }
    }
}
impl PartialOrd for LiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn partial_cmp(&self, other: &LiteralExtra) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for LiteralExtra {}
impl Ord for LiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&LiteralExtra::LanguageTag(ref langtag1),
             &LiteralExtra::LanguageTag(ref langtag2)) => langtag1.cmp(&langtag2),
            (&LiteralExtra::LanguageTag(_), _) => Ordering::Greater,
            (_, &LiteralExtra::LanguageTag(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}
