use std::fmt::Debug;
#[derive(Debug, Clone)]
pub enum TripleObjectType {
    BlankNode = 0,
    IRI = 1,
    Literal = 2,
    LiteralLang = 3,
}

pub trait CompactTriple<T>: Ord + Copy + Eq + Debug
where
    T: Ord + Copy,
{
    fn triple(
        subject_is_iri: bool,
        subject: T,
        predicate: T,
        object_type: TripleObjectType,
        object: T,
        datatype_or_lang: T,
    ) -> Self;
    fn max_subject_value() -> T;
    fn max_datatype_value() -> T;
    fn subject_is_iri(&self) -> bool;
    fn object_is_blank_node(&self) -> bool;
    fn object_is_iri(&self) -> bool;
    fn object_type(&self) -> TripleObjectType;
    fn has_language(&self) -> bool;
    fn subject(&self) -> T;
    fn predicate(&self) -> T;
    fn object(&self) -> T;
    fn datatype_or_lang(&self) -> T;
    fn set_subject(&mut self, subject: T);
    fn set_predicate(&mut self, predicate: T);
    fn set_object(&mut self, object: T);
    fn set_datatype_or_lang(&mut self, datatype_or_lang: T);
}
