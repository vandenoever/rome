use std::mem;
use std::cmp::Ordering;

use string_collector::*;

#[derive (PartialEq,Eq,PartialOrd,Ord)]
struct InnerTriple {
    subject: InnerSubject,
    predicate: StringId,
    object: InnerObject,
}

pub type BlankNode = (u32, u32);

#[derive (PartialEq,Eq,PartialOrd,Ord)]
enum InnerSubject {
    IRI(StringId),
    BlankNode(BlankNode),
}
#[derive (PartialEq,Eq,PartialOrd,Ord)]
pub struct InnerLiteral {
    lexical: StringId,
    datatype: StringId,
    extra: InnerLiteralExtra,
}
#[derive(Clone,Copy)]
pub enum InnerLiteralExtra {
    None,
    LanguageTag(StringId),
    XsdInteger(i64),
    XsdDecimal(f64),
    XsdDouble(f64),
    XsdBoolean(bool),
}
#[derive (PartialEq,Eq,PartialOrd,Ord)]
enum InnerObject {
    IRI(StringId),
    BlankNode(BlankNode),
    Literal(InnerLiteral),
}

#[derive (PartialEq,Eq,PartialOrd,Ord)]
pub struct Triple<'a> {
    subject: Subject<'a>,
    predicate: &'a str,
    object: Object<'a>,
}

#[derive (PartialEq,Eq,PartialOrd,Ord)]
pub enum Subject<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
pub struct Literal<'a> {
    lexical: &'a str,
    datatype: &'a str,
    extra: LiteralExtra<'a>,
}

pub enum LiteralExtra<'a> {
    None,
    LanguageTag(&'a str),
    XsdInteger(i64),
    XsdDecimal(f64),
    XsdDouble(f64),
    XsdBoolean(bool),
}

#[derive(PartialEq,Eq,PartialOrd,Ord)]
pub enum Object<'a> {
    IRI(&'a str),
    BlankNode(BlankNode),
    Literal(Literal<'a>),
}
impl<'a> PartialEq for LiteralExtra<'a> {
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
impl<'a> PartialOrd for LiteralExtra<'a> {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn partial_cmp(&self, other: &LiteralExtra) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Eq for LiteralExtra<'a> {}
impl<'a> Ord for LiteralExtra<'a> {
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
impl PartialEq for InnerLiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn eq(&self, other: &InnerLiteralExtra) -> bool {
        match (self, other) {
            (&InnerLiteralExtra::LanguageTag(ref langtag1),
             &InnerLiteralExtra::LanguageTag(ref langtag2)) => langtag1 == langtag2,
            (&InnerLiteralExtra::LanguageTag(_), _) => false,
            (_, &InnerLiteralExtra::LanguageTag(_)) => false,
            _ => true,
        }
    }
}
impl PartialOrd for InnerLiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn partial_cmp(&self, other: &InnerLiteralExtra) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for InnerLiteralExtra {}
impl Ord for InnerLiteralExtra {
    // the language tag is the only significant content in LiteralExtra
    // the other differences should be triggered by lexical and datatype
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&InnerLiteralExtra::LanguageTag(ref langtag1),
             &InnerLiteralExtra::LanguageTag(ref langtag2)) => langtag1.cmp(&langtag2),
            (&InnerLiteralExtra::LanguageTag(_), _) => Ordering::Greater,
            (_, &InnerLiteralExtra::LanguageTag(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

struct GraphWriter {
    string_collector: StringCollector,
    triples: Vec<InnerTriple>,
    prev_subject_iri: Option<(String, StringId)>,
    prev_predicate: Option<(String, StringId)>,
}
struct Graph {
    strings: StringCollection,
    triples: Vec<InnerTriple>,
}

fn subject<'a>(s: &InnerSubject, strings: &'a StringCollection) -> Subject<'a> {
    match *s {
        InnerSubject::IRI(iri) => Subject::IRI(strings.get(iri)),
        InnerSubject::BlankNode(n) => Subject::BlankNode(n),
    }
}
fn object<'a>(o: &InnerObject, strings: &'a StringCollection) -> Object<'a> {
    match *o {
        InnerObject::IRI(iri) => Object::IRI(strings.get(iri)),
        InnerObject::BlankNode(n) => Object::BlankNode(n),
        InnerObject::Literal(ref l) => {
            Object::Literal(Literal {
                lexical: strings.get(l.lexical),
                datatype: strings.get(l.datatype),
                extra: match l.extra {
                    InnerLiteralExtra::None => LiteralExtra::None,
                    InnerLiteralExtra::LanguageTag(lang) => {
                        LiteralExtra::LanguageTag(strings.get(lang))
                    }
                    InnerLiteralExtra::XsdInteger(v) => LiteralExtra::XsdInteger(v),
                    InnerLiteralExtra::XsdDecimal(v) => LiteralExtra::XsdDecimal(v),
                    InnerLiteralExtra::XsdDouble(v) => LiteralExtra::XsdDouble(v),
                    InnerLiteralExtra::XsdBoolean(v) => LiteralExtra::XsdBoolean(v),
                },
            })
        }
    }
}
fn triple<'a>(t: &InnerTriple, strings: &'a StringCollection) -> Triple<'a> {
    Triple {
        subject: subject(&t.subject, strings),
        predicate: strings.get(t.predicate),
        object: object(&t.object, strings),
    }
}
fn translate_subject(s: &InnerSubject, translation: &Vec<StringId>) -> InnerSubject {
    match *s {
        InnerSubject::IRI(iri) => InnerSubject::IRI(translation[iri]),
        InnerSubject::BlankNode(b) => InnerSubject::BlankNode(b),
    }
}
fn translate_object(o: &InnerObject, translation: &Vec<StringId>) -> InnerObject {
    match *o {
        InnerObject::IRI(iri) => InnerObject::IRI(translation[iri]),
        InnerObject::BlankNode(b) => InnerObject::BlankNode(b),
        InnerObject::Literal(ref l) => {
            InnerObject::Literal(InnerLiteral {
                lexical: translation[l.lexical],
                datatype: translation[l.datatype],
                extra: match l.extra {
                    InnerLiteralExtra::LanguageTag(lang) => {
                        InnerLiteralExtra::LanguageTag(translation[lang])
                    }
                    o => o,
                },
            })
        }
    }
}
impl GraphWriter {
    pub fn with_capacity(capacity: usize) -> GraphWriter {
        GraphWriter {
            string_collector: StringCollector::with_capacity(capacity),
            triples: Vec::new(),
            prev_subject_iri: None,
            prev_predicate: None,
        }
    }
    pub fn add(&mut self, subject: &str, predicate: &str, object: &str) {
        if let Some((str, id)) = self.prev_subject_iri.take() {

        }
        let required = subject.len() + predicate.len() + object.len();
        if self.string_collector.space() < required {
            // handling a full buffer is not implemented yet
            unimplemented!();
        }
        let s = self.string_collector.add_string(subject);
        let p = self.string_collector.add_string(predicate);
        let o = self.string_collector.add_string(object);
        self.triples.push(InnerTriple {
            subject: InnerSubject::IRI(s),
            predicate: p,
            object: InnerObject::IRI(o),
        });
    }
    pub fn collect(&mut self) -> Graph {
        let (translation, string_collection) = self.string_collector.collect();
        let mut triples = Vec::new();
        mem::swap(&mut triples, &mut self.triples);
        for t in triples.iter_mut() {
            t.subject = translate_subject(&t.subject, &translation);
            t.predicate = translation[t.predicate];
            t.object = translate_object(&t.object, &translation);
        }
        // sort according to StringId, which is sorted alphabetically
        triples.sort();
        triples.dedup();
        Graph {
            strings: string_collection,
            triples: triples,
        }
    }
}
