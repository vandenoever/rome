use std::cmp;
use std::marker::PhantomData;
use std::mem;
use rand;
use graph;
use constants;
use super::compact_triple::*;
use super::string_collector::*;
use super::graph::*;
use super::triple::*;
#[cfg(test)]
use super::triple64::*;

pub struct BlankNodeCreator<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    graph_id: u32,
    highest_blank_node: u32,
    phantom: PhantomData<(SPO, OPS)>,
}

impl<SPO, OPS> BlankNodeCreator<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub fn new() -> BlankNodeCreator<SPO, OPS> {
        BlankNodeCreator {
            graph_id: rand::random::<u32>(),
            highest_blank_node: 0,
            phantom: PhantomData,
        }
    }
}

impl<'g, SPO: 'g, OPS: 'g> graph::BlankNodeCreator<'g, BlankNodePtr<'g, SPO, OPS>>
    for BlankNodeCreator<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn create_blank_node(&mut self) -> BlankNodePtr<'g, SPO, OPS> {
        self.highest_blank_node += 1;
        BlankNodePtr {
            graph_id: self.graph_id,
            node_id: self.highest_blank_node,
            phantom: PhantomData,
        }
    }
}

pub struct GraphWriter<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    graph_id: u32,
    string_collector: StringCollector,
    datatype_lang_collector: StringCollector,
    triples: Vec<SPO>,
    prev_subject_iri: Option<(String, StringId)>,
    prev_predicate: Option<(String, StringId)>,
    prev_datatype: Option<(String, StringId)>,
    prev_lang: Option<(String, StringId)>,
    lang_string_datatype_id: StringId,
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
    pub fn with_capacity(capacity: usize,
                         blank_node_creator: &BlankNodeCreator<SPO, OPS>)
                         -> GraphWriter<SPO, OPS> {
        let mut dlc = StringCollector::with_capacity(capacity);
        let lang_string_datatype_id = dlc.add_string(constants::RDF_LANG_STRING);
        GraphWriter {
            graph_id: blank_node_creator.graph_id,
            string_collector: StringCollector::with_capacity(capacity),
            datatype_lang_collector: dlc,
            triples: Vec::new(),
            prev_subject_iri: None,
            prev_predicate: None,
            prev_datatype: None,
            prev_lang: None,
            lang_string_datatype_id: lang_string_datatype_id,
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
    fn add_iri_blank(&mut self, subject: &str, predicate: &str, object: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, object);
        self.add_s_iri(subject, predicate, TripleObjectType::BlankNode, object, 0);
    }
    fn add_iri_iri(&mut self, subject: &str, predicate: &str, object: &str) {
        let o = self.string_collector.add_string(object);
        self.add_s_iri(subject, predicate, TripleObjectType::IRI, o.id, 0);
    }
    fn add_iri_lit(&mut self, subject: &str, predicate: &str, object: &str, datatype: &str) {
        let o = self.string_collector.add_string(object);
        let d = check_prev(datatype,
                           &mut self.prev_datatype,
                           &mut self.datatype_lang_collector)
            .id;
        self.add_s_iri(subject, predicate, TripleObjectType::Literal, o.id, d);
    }
    fn add_iri_lit_lang(&mut self, subject: &str, predicate: &str, object: &str, lang: &str) {
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
    fn add_blank_blank<'a>(&mut self, subject: u32, predicate: &'a str, object: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, object);
        self.add_s_blank(subject, predicate, TripleObjectType::BlankNode, object, 0);
    }
    fn add_blank_iri(&mut self, subject: u32, predicate: &str, object: &str) {
        let o = self.string_collector.add_string(object);
        self.add_s_blank(subject, predicate, TripleObjectType::IRI, o.id, 0);
    }
    fn add_blank_lit(&mut self, subject: u32, predicate: &str, object: &str, datatype: &str) {
        let o = self.string_collector.add_string(object);
        let d = check_prev(datatype,
                           &mut self.prev_datatype,
                           &mut self.datatype_lang_collector)
            .id;
        self.add_s_blank(subject, predicate, TripleObjectType::Literal, o.id, d);
    }
    fn add_blank_lit_lang(&mut self, subject: u32, predicate: &str, object: &str, lang: &str) {
        let o = self.string_collector.add_string(object);
        let l = check_prev(lang, &mut self.prev_lang, &mut self.datatype_lang_collector).id;
        self.add_s_blank(subject, predicate, TripleObjectType::LiteralLang, o.id, l);
    }
    fn add<'t, I, L>(&mut self,
                     subject: graph::BlankNodeOrIRI<'t, BlankNodePtr<'t, SPO, OPS>, I>,
                     predicate: I,
                     object: graph::Resource<'t, BlankNodePtr<'t, SPO, OPS>, I, L>)
        where I: graph::IRIPtr<'t>,
              L: graph::LiteralPtr<'t>
    {
        match subject {
            graph::BlankNodeOrIRI::BlankNode(subject, _) => {
                match object {
                    graph::Resource::BlankNode(object, _) => {
                        self.add_blank_blank(subject.node_id, predicate.as_str(), object.node_id);
                    }
                    graph::Resource::IRI(object) => {
                        self.add_blank_iri(subject.node_id, predicate.as_str(), object.as_str());
                    }
                    graph::Resource::Literal(object) => {
                        match object.language() {
                            None => {
                                self.add_blank_lit(subject.node_id,
                                                   predicate.as_str(),
                                                   object.as_str(),
                                                   object.datatype());
                            }
                            Some(lang) => {
                                self.add_blank_lit_lang(subject.node_id,
                                                        predicate.as_str(),
                                                        object.as_str(),
                                                        lang);
                            }
                        }
                    }
                }
            }
            graph::BlankNodeOrIRI::IRI(subject) => {
                match object {
                    graph::Resource::BlankNode(object, _) => {
                        self.add_iri_blank(subject.as_str(), predicate.as_str(), object.node_id);
                    }
                    graph::Resource::IRI(object) => {
                        self.add_iri_iri(subject.as_str(), predicate.as_str(), object.as_str());
                    }
                    graph::Resource::Literal(object) => {
                        match object.language() {
                            None => {
                                self.add_iri_lit(subject.as_str(),
                                                 predicate.as_str(),
                                                 object.as_str(),
                                                 object.datatype());
                            }
                            Some(lang) => {
                                self.add_iri_lit_lang(subject.as_str(),
                                                      predicate.as_str(),
                                                      object.as_str(),
                                                      lang);
                            }
                        }
                    }
                }
            }
        }
    }
    fn check_blank_node(&self, blank_node: &BlankNodePtr<SPO, OPS>) {
        assert_eq!(self.graph_id,
                   blank_node.node_id,
                   "Blank node is not associated with this graph creator.");
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

impl<'g, SPO: 'g, OPS: 'g> graph::GraphCreator<'g, BlankNodePtr<'g, SPO, OPS>>
    for GraphWriter<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    type Graph = Graph<SPO, OPS>;
    fn add_triple<T, I: 'g, L: 'g>(&mut self, triple: &T)
        where T: graph::Triple<'g, BlankNodePtr<'g, SPO, OPS>, I, L>,
              I: graph::IRIPtr<'g>,
              L: graph::LiteralPtr<'g>
    {
        self.add(triple.subject(), triple.predicate(), triple.object());
    }

    fn collect(mut self) -> Graph<SPO, OPS> {
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
                graph_id: self.graph_id,
                strings: string_collection,
                datatype_or_lang: datatype_lang_collection,
                spo: spo,
                ops: ops,
                lang_string_datatype_id: datatrans[self.lang_string_datatype_id].id,
                highest_blank_node: self.highest_blank_node,
            },
        }
    }
    fn add_blank_blank<'p, P>(&mut self,
                              subject: BlankNodePtr<SPO, OPS>,
                              predicate: P,
                              object: BlankNodePtr<SPO, OPS>)
        where P: graph::IRIPtr<'p>
    {
        self.check_blank_node(&subject);
        self.check_blank_node(&object);
        GraphWriter::add_blank_blank(self, subject.node_id, predicate.as_str(), object.node_id);
    }
    fn add_blank_iri<'p, 'o, P, O>(&mut self,
                                   subject: BlankNodePtr<SPO, OPS>,
                                   predicate: P,
                                   object: O)
        where P: graph::IRIPtr<'p>,
              O: graph::IRIPtr<'o>
    {
        self.check_blank_node(&subject);
        GraphWriter::add_blank_iri(self, subject.node_id, predicate.as_str(), object.as_str());
    }
    fn add_blank_literal<'p, 'o, P, O>(&mut self,
                                       subject: BlankNodePtr<SPO, OPS>,
                                       predicate: P,
                                       object: O)
        where P: graph::IRIPtr<'p>,
              O: graph::LiteralPtr<'o>
    {
        self.check_blank_node(&subject);
        match object.language() {
            Some(lang) => {
                GraphWriter::add_blank_lit_lang(self,
                                                subject.node_id,
                                                predicate.as_str(),
                                                object.as_str(),
                                                lang)
            }
            None => {
                GraphWriter::add_blank_lit(self,
                                           subject.node_id,
                                           predicate.as_str(),
                                           object.as_str(),
                                           object.datatype())
            }
        }
    }
    fn add_iri_blank<'s, 'p, S, P>(&mut self,
                                   subject: S,
                                   predicate: P,
                                   object: BlankNodePtr<SPO, OPS>)
        where S: graph::IRIPtr<'s>,
              P: graph::IRIPtr<'p>
    {
        self.check_blank_node(&object);
        GraphWriter::add_iri_blank(self, subject.as_str(), predicate.as_str(), object.node_id);
    }
    fn add_iri_iri<'s, 'p, 'o, S, P, O>(&mut self, subject: S, predicate: P, object: O)
        where S: graph::IRIPtr<'s>,
              P: graph::IRIPtr<'p>,
              O: graph::IRIPtr<'o>
    {
        GraphWriter::add_iri_iri(self, subject.as_str(), predicate.as_str(), object.as_str());
    }
    fn add_iri_literal<'s, 'p, 'o, S, P, O>(&mut self, subject: S, predicate: P, object: O)
        where S: graph::IRIPtr<'s>,
              P: graph::IRIPtr<'p>,
              O: graph::LiteralPtr<'o>
    {
        match object.language() {
            Some(lang) => {
                GraphWriter::add_iri_lit_lang(self,
                                              subject.as_str(),
                                              predicate.as_str(),
                                              object.as_str(),
                                              lang)
            }
            None => {
                GraphWriter::add_iri_lit(self,
                                         subject.as_str(),
                                         predicate.as_str(),
                                         object.as_str(),
                                         object.datatype())
            }
        }
    }
}

#[test]
fn collect_empty() {
    let bnc = BlankNodeCreator::new();
    let writer: GraphWriter<Triple64SPO, Triple64OPS> = GraphWriter::with_capacity(0, &bnc);
    use graph::GraphCreator;
    writer.collect();
}

#[test]
fn keep_blank_subject() {
    use graph::{BlankNodeCreator, GraphCreator, Graph, IRIPtr, Triple};
    let mut bnc = super::BlankNodeCreator::new();
    let mut writer: GraphWriter<Triple64SPO, Triple64OPS> = GraphWriter::with_capacity(0, &bnc);
    let blank1 = bnc.create_blank_node();
    let blank2 = bnc.create_blank_node();
    writer.add_blank_blank(blank1.node_id, "", blank2.node_id);
    let graph = writer.collect();
    let triple = graph.iter().next().unwrap();
    assert_eq!(triple.subject(),
               graph::BlankNodeOrIRI::BlankNode(blank1, PhantomData));
    assert_eq!(triple.predicate().as_str(), "");
    assert_eq!(triple.object(),
               graph::Resource::BlankNode(blank2, PhantomData));
}
