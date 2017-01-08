use std::cmp;
use std::marker::PhantomData;
use std::mem;
use rand;
use graph;
use super::compact_triple::*;
use super::string_collector::*;
use super::graph::*;
#[cfg(test)]
use super::triple64::*;

pub struct GraphWriter<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    string_collector: StringCollector,
    datatype_lang_collector: StringCollector,
    triples: Vec<SPO>,
    prev_subject_iri: Option<(String, StringId)>,
    prev_predicate: Option<(String, StringId)>,
    prev_datatype: Option<(String, StringId)>,
    prev_lang: Option<(String, StringId)>,
    highest_blank_node: u32,
    phantom: PhantomData<OPS>,
}

fn translate<T>(t: &mut T, translation: &Vec<StringId>, datatrans: &Vec<StringId>)
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
            t.set_datatype_or_lang(datatrans[datatype_or_lang].id);
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
impl<SPO, OPS> GraphWriter<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub fn with_capacity(capacity: usize) -> GraphWriter<SPO, OPS> {
        GraphWriter {
            string_collector: StringCollector::with_capacity(capacity),
            datatype_lang_collector: StringCollector::with_capacity(capacity),
            triples: Vec::new(),
            prev_subject_iri: None,
            prev_predicate: None,
            prev_datatype: None,
            prev_lang: None,
            highest_blank_node: 0,
            phantom: PhantomData,
        }
    }
    pub fn highest_blank_node(&self) -> u32 {
        self.highest_blank_node
    }
    fn add_s_iri(&mut self, s: &str, p: &str, ot: TripleObjectType, o: u32, d: u32) {
        let s = check_prev(s, &mut self.prev_subject_iri, &mut self.string_collector);
        let p = check_prev(p, &mut self.prev_predicate, &mut self.string_collector);
        let t = SPO::triple(true, s.id, p.id, ot, o, d);
        self.triples.push(t);
    }
    pub fn add_iri_blank(&mut self, subject: &str, predicate: &str, object: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, object);
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
        self.highest_blank_node = cmp::max(self.highest_blank_node, s);
        let p = check_prev(p, &mut self.prev_predicate, &mut self.string_collector);
        let t = SPO::triple(false, s, p.id, ot, o, d);
        self.triples.push(t);
    }
    pub fn add_blank_blank(&mut self, subject: u32, predicate: &str, object: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, object);
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
    fn add_(&mut self, subject: graph::Subject, predicate: &str, object: graph::Object) {
        match subject {
            graph::Subject::IRI(subject) => {
                match object {
                    graph::Object::IRI(object) => {
                        self.add_iri_iri(subject, predicate, object);
                    }
                    graph::Object::BlankNode(object) => {
                        self.add_iri_blank(subject, predicate, object.0 as u32);
                    }
                    graph::Object::Literal(object) => {
                        match object.language {
                            None => {
                                self.add_iri_lit(subject,
                                                 predicate,
                                                 object.lexical,
                                                 object.datatype);
                            }
                            Some(lang) => {
                                self.add_iri_lit_lang(subject, predicate, object.lexical, lang);
                            }
                        }
                    }
                }
            }
            graph::Subject::BlankNode(subject) => {
                match object {
                    graph::Object::IRI(object) => {
                        self.add_blank_iri(subject.0 as u32, predicate, object);
                    }
                    graph::Object::BlankNode(object) => {
                        self.add_blank_blank(subject.0 as u32, predicate, object.0 as u32);
                    }
                    graph::Object::Literal(object) => {
                        match object.language {
                            None => {
                                self.add_blank_lit(subject.0 as u32,
                                                   predicate,
                                                   object.lexical,
                                                   object.datatype);
                            }
                            Some(lang) => {
                                self.add_blank_lit_lang(subject.0 as u32,
                                                        predicate,
                                                        object.lexical,
                                                        lang);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn create_ops<SPO, OPS>(spo: &[SPO]) -> Vec<OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    let mut ops = Vec::with_capacity(spo.len());
    for t in spo {
        ops.push(OPS::triple(t.subject_is_iri(),
                             t.subject(),
                             t.predicate(),
                             t.object_type(),
                             t.object(),
                             t.datatype_or_lang()));
    }
    ops.sort();
    ops
}

impl<'g, SPO: 'g, OPS: 'g> graph::GraphCreator<'g> for GraphWriter<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    type Graph = Graph<SPO, OPS>;
    fn add_triple<'t, T>(&mut self, triple: &T)
        where T: graph::Triple<'t>
    {
        self.add_(triple.subject(), triple.predicate(), triple.object());
    }

    fn collect(&mut self) -> Graph<SPO, OPS> {
        let (translation, string_collection) = self.string_collector.collect();
        let (datatrans, datatype_lang_collection) = self.datatype_lang_collector.collect();
        let mut spo = Vec::new();
        mem::swap(&mut spo, &mut self.triples);
        for t in spo.iter_mut() {
            translate(t, &translation, &datatrans);
        }
        // sort according to StringId, which is sorted alphabetically
        spo.sort();
        spo.dedup();
        spo.shrink_to_fit();
        let ops = create_ops(&spo);
        Graph {
            d: GraphData {
                graph_id: rand::random::<usize>(),
                strings: string_collection,
                datatype_or_lang: datatype_lang_collection,
                spo: spo,
                ops: ops,
                highest_blank_node: self.highest_blank_node,
            },
        }
    }
    fn create_blank_node(&mut self) -> graph::BlankNode {
        self.highest_blank_node += 1;
        (self.highest_blank_node as usize, 0)
    }
    fn add<'a: 'b, 'b, S, O>(&'a mut self, subject: S, predicate: &str, object: O)
        where S: graph::IntoSubject<'b>,
              O: graph::IntoObject<'b>
    {
        self.add_(subject.subject(), predicate, object.object());
    }
}

#[test]
fn collect_empty() {
    let mut writer: GraphWriter<Triple64SPO, Triple64OPS> = GraphWriter::with_capacity(0);
    use graph::GraphCreator;
    writer.collect();
}

#[test]
fn keep_blank_subject() {
    let mut writer: GraphWriter<Triple64SPO, Triple64OPS> = GraphWriter::with_capacity(0);
    writer.add_blank_blank(1, "", 2);
    use graph::{GraphCreator, Graph, Triple};
    let graph = writer.collect();
    let triple = graph.iter().next().unwrap();
    assert_eq!(triple.subject(), graph::Subject::BlankNode((1, 0)));
    assert_eq!(triple.predicate(), "");
    assert_eq!(triple.object(), graph::Object::BlankNode((2, 0)));
}
