use std::mem;
use std::slice;
use std::rc::Rc;
use grammar;
use graph;
use string_collector::*;
use triple_to_uint::*;

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
    strings: Rc<StringCollection>,
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
    pub fn add_triple<T>(&mut self, triple: &T)
        where T: graph::Triple
    {
        match triple.subject() {
            graph::Subject::IRI(subject) => {
                match triple.object() {
                    graph::Object::IRI(object) => {
                        self.add_iri_iri(subject, triple.predicate(), object);
                    }
                    graph::Object::BlankNode(object) => {
                        self.add_iri_blank(subject, triple.predicate(), object.0 as u32);
                    }
                    graph::Object::Literal(object) => {
                        match object.language {
                            None => {
                                self.add_iri_lit(subject,
                                                 triple.predicate(),
                                                 object.lexical,
                                                 object.datatype);
                            }
                            Some(lang) => {
                                self.add_iri_lit_lang(subject,
                                                      triple.predicate(),
                                                      object.lexical,
                                                      lang);
                            }
                        }
                    }
                }
            }
            graph::Subject::BlankNode(subject) => {
                match triple.object() {
                    graph::Object::IRI(object) => {
                        self.add_blank_iri(subject.0 as u32, triple.predicate(), object);
                    }
                    graph::Object::BlankNode(object) => {
                        self.add_blank_blank(subject.0 as u32, triple.predicate(), object.0 as u32);
                    }
                    graph::Object::Literal(object) => {
                        match object.language {
                            None => {
                                self.add_blank_lit(subject.0 as u32,
                                                   triple.predicate(),
                                                   object.lexical,
                                                   object.datatype);
                            }
                            Some(lang) => {
                                self.add_blank_lit_lang(subject.0 as u32,
                                                        triple.predicate(),
                                                        object.lexical,
                                                        lang);
                            }
                        }
                    }
                }
            }
        }
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
            strings: Rc::new(string_collection),
            triples: triples,
        }
    }
}

pub struct GraphTriple {
    strings: Rc<StringCollection>,
    triple: Triple64,
}

struct GraphIterator<'a> {
    strings: Rc<StringCollection>,
    iter: slice::Iter<'a, Triple64>,
}

impl<'a> Iterator for GraphIterator<'a> {
    type Item = GraphTriple;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| {
            GraphTriple {
                strings: self.strings.clone(),
                triple: *t,
            }
        })
    }
}

impl graph::Triple for GraphTriple {
    fn subject(&self) -> graph::Subject {
        if self.triple.subject_is_iri() {
            graph::Subject::IRI(self.strings.get(StringId { id: self.triple.subject() }))
        } else {
            graph::Subject::BlankNode((0, 0))
        }
    }
    fn predicate(&self) -> &str {
        self.strings.get(StringId { id: self.triple.predicate() })
    }
    fn object(&self) -> graph::Object {
        if self.triple.object_is_iri() {
            graph::Object::IRI(self.strings.get(StringId { id: self.triple.object() }))
        } else if self.triple.object_is_blank_node() {
            graph::Object::BlankNode((0, 0))
        } else if self.triple.has_language() {
            graph::Object::Literal(graph::Literal {
                lexical: self.strings.get(StringId { id: self.triple.object() }),
                datatype: grammar::RDF_LANG_STRING,
                language: Some(self.strings.get(StringId { id: self.triple.datatype_or_lang() })),
            })
        } else {
            graph::Object::Literal(graph::Literal {
                lexical: self.strings.get(StringId { id: self.triple.object() }),
                datatype: self.strings.get(StringId { id: self.triple.datatype_or_lang() }),
                language: None,
            })
        }
    }
}

impl Graph {
    // pub fn iter<'a>(&'a self) -> Box<Iterator<Item = &GraphTriple> + 'a> {
    // Box::new(GraphIterator {
    // strings: self.strings.clone(),
    // iter: self.triples.iter(),
    // })
    // }
    // pub fn len(&self) -> usize {
    // self.triples.len()
    // }
    //
}

impl<'a> graph::Graph<'a> for Graph {
    type Triple = GraphTriple;
    fn iter(&'a self) -> Box<Iterator<Item = Self::Triple> + 'a> {
        Box::new(GraphIterator {
            strings: self.strings.clone(),
            iter: self.triples.iter(),
        })
    }
    fn len(&self) -> usize {
        self.triples.len()
    }
}
