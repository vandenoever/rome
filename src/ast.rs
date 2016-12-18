#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Subject {
    IRI(String),
    BlankNode(usize),
}
#[derive(Debug,PartialEq,Clone)]
pub enum Object {
    IRI(String),
    BlankNode(usize),
    LangString(String, String),
    XsdString(String),
    XsdInteger(i64),
    XsdDecimal(f64),
    XsdDouble(f64),
    XsdBoolean(bool),
    TypedLiteral(String, String),
}
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Triple {
    pub subject: Subject,
    pub predicate: String,
    pub object: Object,
}

// explicit implementation of Eq because f64 does not have that
impl Eq for Object {}
