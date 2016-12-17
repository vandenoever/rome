#[derive(Clone)]
pub enum Subject {
    IRI(String),
    BlankNode(String),
}

pub enum Object {
    IRI(String),
    BlankNode(String),
    LangString(String, String),
    XsdString(String),
    XsdInteger(i64),
    XsdDecimal(f64),
    XsdDouble(f64),
    XsdBoolean(bool),
    TypedLiteral(String, String),
}

pub struct Triple {
    pub subject: Subject,
    pub predicate: String,
    pub object: Object,
}
