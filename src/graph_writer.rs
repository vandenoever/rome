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
    triples: Vec<Triple64SPO>,
    prev_subject_iri: Option<(String, StringId)>,
    prev_predicate: Option<(String, StringId)>,
    prev_datatype: Option<(String, StringId)>,
    prev_lang: Option<(String, StringId)>,
}
pub struct Graph {
    strings: Rc<StringCollection>,
    spo: Vec<Triple64SPO>,
    ops: Vec<Triple64OPS>,
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
        let t = Triple64SPO::triple(true, s.id, p.id, ot, o, d);
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
        let t = Triple64SPO::triple(false, s, p.id, ot, o, d);
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
        let mut spo = Vec::new();
        mem::swap(&mut spo, &mut self.triples);
        for t in spo.iter_mut() {
            translate(t, &translation);
        }
        // sort according to StringId, which is sorted alphabetically
        spo.sort();
        spo.dedup();
        spo.shrink_to_fit();
        let ops = create_ops(&spo);
        Graph {
            strings: Rc::new(string_collection),
            spo: spo,
            ops: ops,
        }
    }
}

fn create_ops(spo: &[Triple64SPO]) -> Vec<Triple64OPS> {
    let mut ops = Vec::with_capacity(spo.len());
    for t in spo {
        ops.push(Triple64OPS::triple(t.subject_is_iri(),
                                     t.subject(),
                                     t.predicate(),
                                     t.object_type(),
                                     t.object(),
                                     t.datatype_or_lang()));
    }
    ops.sort();
    ops
}

pub struct GraphTriple<T> {
    strings: Rc<StringCollection>,
    triple: T,
}

struct GraphIterator<'a, T: 'a> {
    strings: Rc<StringCollection>,
    iter: slice::Iter<'a, T>,
}

impl<'a, T> Iterator for GraphIterator<'a, T>
    where T: Copy
{
    type Item = GraphTriple<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|t| {
            GraphTriple {
                strings: self.strings.clone(),
                triple: *t,
            }
        })
    }
}

impl<T> graph::Triple for GraphTriple<T>
    where T: CompactTriple<u32>
{
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

pub struct TripleRangeIterator<'a, T: 'a>
    where T: CompactTriple<u32>
{
    strings: Rc<StringCollection>,
    iter: slice::Iter<'a, T>,
    end: T,
}

impl<'a, T> Iterator for TripleRangeIterator<'a, T>
    where T: Ord + CompactTriple<u32> + Copy
{
    type Item = GraphTriple<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(t) if *t < self.end => {
                Some(GraphTriple {
                    strings: self.strings.clone(),
                    triple: *t,
                })
            }
            _ => None,
        }
    }
}

impl Graph {
    fn range_iter<'a, T>(&self, index: &'a [T], start: &T, end: &T) -> TripleRangeIterator<'a, T>
        where T: CompactTriple<u32> + Ord + Copy
    {
        let slice = match index.binary_search(&start) {
            Ok(pos) => &index[pos..pos],
            Err(pos) => &index[pos..pos],
        };
        TripleRangeIterator {
            strings: self.strings.clone(),
            iter: slice.iter(),
            end: *end,
        }
    }
    fn empty_range_iter<T>(&self) -> TripleRangeIterator<T>
        where T: CompactTriple<u32> + Ord
    {
        let end = T::triple(true, 0, 0, TripleObjectType::BlankNode, 0, 0);
        TripleRangeIterator {
            strings: self.strings.clone(),
            iter: [].iter(),
            end: end,
        }
    }
    /// iterator over all triples with the same subject
    fn iter_subject(&self, triple: Triple64SPO) -> TripleRangeIterator<Triple64SPO> {
        let mut end = triple;
        end.set_subject(triple.subject() + 1);
        self.range_iter(&self.spo, &triple, &end)
    }
    /// iterator over all triples with the same subject
    pub fn iter_subject_iri(&self, iri: &str) -> TripleRangeIterator<Triple64SPO> {
        match self.strings.find(iri) {
            None => self.empty_range_iter(),
            Some(id) => {
                let triple = Triple64SPO::triple(true, id.id, 0, TripleObjectType::BlankNode, 0, 0);
                self.iter_subject(triple)
            }
        }
    }
    /// iterator over all triples with the same object
    fn iter_object(&self, triple: Triple64OPS) -> TripleRangeIterator<Triple64OPS> {
        let mut end = triple;
        end.set_object(triple.object() + 1);
        self.range_iter(&self.ops, &triple, &end)
    }
    /// iterator over all triples with the same object
    pub fn iter_object_iri(&self, iri: &str) -> TripleRangeIterator<Triple64OPS> {
        match self.strings.find(iri) {
            None => self.empty_range_iter(),
            Some(id) => {
                let triple = Triple64OPS::triple(true, 0, 0, TripleObjectType::IRI, id.id, 0);
                self.iter_object(triple)
            }
        }
    }
}

impl<'a> graph::Graph<'a> for Graph {
    type Triple = GraphTriple<Triple64SPO>;
    fn iter(&'a self) -> Box<Iterator<Item = Self::Triple> + 'a> {
        Box::new(GraphIterator {
            strings: self.strings.clone(),
            iter: self.spo.iter(),
        })
    }
    fn len(&self) -> usize {
        self.spo.len()
    }
}
