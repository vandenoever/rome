#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IRI<'a> {
    IRI(&'a str),
    PrefixedName(&'a str, &'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RDFLiteralType<'a> {
    LangTag(&'a str),
    DataType(IRI<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Literal<'a> {
    pub lexical: &'a str,
    pub datatype: Datatype<'a>,
    pub language: Option<&'a str>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Datatype<'a> {
    IRI(IRI<'a>),
    RDFLangString,
    XSDBoolean,
    XSDDecimal,
    XSDDouble,
    XSDInteger,
    XSDString,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BlankNode<'a> {
    Anon,
    BlankNode(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Subject<'a> {
    IRI(IRI<'a>),
    BlankNode(BlankNode<'a>),
    Collection(Vec<Object<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Object<'a> {
    IRI(IRI<'a>),
    BlankNode(BlankNode<'a>),
    Collection(Vec<Object<'a>>),
    BlankNodePropertyList(Vec<PredicatedObjects<'a>>),
    Literal(Literal<'a>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct PredicatedObjects<'a> {
    pub verb: IRI<'a>,
    pub objects: Vec<Object<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Triples<'a> {
    pub subject: Subject<'a>,
    pub predicated_objects_list: Vec<PredicatedObjects<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Prefix(&'a str, &'a str),
    Base(&'a str),
    Triples(Triples<'a>),
}
