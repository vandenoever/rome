use std::mem;
use std::cmp::Ordering;

use string_collector::*;
use triple_to_uint::*;

pub type BlankNode = (u32, u32);

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

pub struct GraphWriter {
    string_collector: StringCollector,
    datatype_lang_collector: StringCollector,
    triples: Vec<Triple64>,
    prev_subject_iri: Option<(String, StringId)>,
    prev_predicate: Option<(String, StringId)>,
    prev_datatype: Option<(String, StringId)>,
    prev_lang: Option<(String, StringId)>,
}
pub struct Graph {
    strings: StringCollection,
    triples: Vec<Triple64>,
}

fn translate<T>(t: &mut T, translation: &Vec<StringId>)
    where T: CompactTriple<u32>
{
    if t.subject_is_iri() {
        let subject = t.subject() as usize;
        t.set_subject(translation[subject].id);
    }
    let predicate = t.predicate() as usize;
    t.set_predicate(translation[predicate].id);
    if !t.object_is_blank_node() {
        let object = t.object() as usize;
        t.set_object(translation[object].id);
        if !t.object_is_iri() {
            let datatype_or_lang = t.datatype_or_lang() as usize;
            t.set_datatype_or_lang(translation[datatype_or_lang].id);
        }
    }
}
/// check if the new string is the same as the string from the previous triple
/// if the string is the same, use the re-use the id
fn check_prev(string: &str,
              prev: &mut Option<(String, StringId)>,
              string_collector: &mut StringCollector)
              -> StringId {
    let id;
    if let Some((mut prev_string, prev_id)) = prev.take() {
        if string == prev_string {
            id = prev_id;
        } else {
            id = string_collector.add_string(string);
            prev_string.clear();
            prev_string.push_str(string);
        }
        *prev = Some((prev_string, id));
    } else {
        id = string_collector.add_string(string);
        *prev = Some((String::from(string), id));
    }
    id
}
impl GraphWriter {
    pub fn with_capacity(capacity: usize) -> GraphWriter {
        GraphWriter {
            string_collector: StringCollector::with_capacity(capacity),
            datatype_lang_collector: StringCollector::with_capacity(capacity),
            triples: Vec::new(),
            prev_subject_iri: None,
            prev_predicate: None,
            prev_datatype: None,
            prev_lang: None,
        }
    }
    fn add_s_iri(&mut self, s: &str, p: &str, ot: TripleObjectType, o: u32, d: u32) {
        let s = check_prev(s, &mut self.prev_subject_iri, &mut self.string_collector);
        let p = check_prev(p, &mut self.prev_predicate, &mut self.string_collector);
        let t = Triple64::triple(true, s.id, p.id, ot, o, d);
        self.triples.push(t);
    }
    pub fn add_iri_blank(&mut self, subject: &str, predicate: &str, object: u32) {
        self.add_s_iri(subject, predicate, TripleObjectType::BlankNode, object, 0);
    }
    pub fn add_iri_iri(&mut self, subject: &str, predicate: &str, object: &str) {
        let o = self.string_collector.add_string(object);
        self.add_s_iri(subject, predicate, TripleObjectType::IRI, o.id, 0);
    }
    pub fn add_iri_lit(&mut self, subject: &str, predicate: &str, object: &str, datatype: &str) {
        let o = self.string_collector.add_string(object);
        let d = check_prev(datatype,
                           &mut self.prev_datatype,
                           &mut self.datatype_lang_collector)
            .id;
        self.add_s_iri(subject, predicate, TripleObjectType::Literal, o.id, d);
    }
    pub fn add_iri_lit_lang(&mut self, subject: &str, predicate: &str, object: &str, lang: &str) {
        let o = self.string_collector.add_string(object);
        let l = check_prev(lang, &mut self.prev_lang, &mut self.datatype_lang_collector).id;
        self.add_s_iri(subject, predicate, TripleObjectType::LiteralLang, o.id, l);
    }
    fn add_s_blank(&mut self, s: u32, p: &str, ot: TripleObjectType, o: u32, d: u32) {
        let p = check_prev(p, &mut self.prev_predicate, &mut self.string_collector);
        let t = Triple64::triple(false, s, p.id, ot, o, d);
        self.triples.push(t);
    }
    pub fn add_blank_blank(&mut self, subject: u32, predicate: &str, object: u32) {
        self.add_s_blank(subject, predicate, TripleObjectType::BlankNode, object, 0);
    }
    pub fn add_blank_iri(&mut self, subject: u32, predicate: &str, object: &str) {
        let o = self.string_collector.add_string(object);
        self.add_s_blank(subject, predicate, TripleObjectType::IRI, o.id, 0);
    }
    pub fn add_blank_lit(&mut self, subject: u32, predicate: &str, object: &str, datatype: &str) {
        let o = self.string_collector.add_string(object);
        let d = check_prev(datatype,
                           &mut self.prev_datatype,
                           &mut self.datatype_lang_collector)
            .id;
        self.add_s_blank(subject, predicate, TripleObjectType::Literal, o.id, d);
    }
    pub fn add_blank_lit_lang(&mut self, subject: u32, predicate: &str, object: &str, lang: &str) {
        let o = self.string_collector.add_string(object);
        let l = check_prev(lang, &mut self.prev_lang, &mut self.datatype_lang_collector).id;
        self.add_s_blank(subject, predicate, TripleObjectType::LiteralLang, o.id, l);
    }
    pub fn collect(&mut self) -> Graph {
        let (translation, string_collection) = self.string_collector.collect();
        let mut triples = Vec::new();
        mem::swap(&mut triples, &mut self.triples);
        for t in triples.iter_mut() {
            translate(t, &translation);
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
