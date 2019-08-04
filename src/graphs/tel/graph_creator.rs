use super::compact_triple::*;
use super::graph::*;
use super::string_collector::*;
use super::triple::*;
#[cfg(test)]
use super::triple64::*;
use crate::graph;
use crate::ontology::iri::rdf;
use rand;
use std::cmp;
use std::marker::PhantomData;
use std::mem;

pub struct GraphCreator<SPO, OPS>
where
    SPO: CompactTriple<u32>,
    OPS: CompactTriple<u32>,
{
    graph_id: u32,
    string_collector: StringCollector,
    datatype_lang_collector: StringCollector,
    triples: Vec<SPO>,
    lang_string_datatype_id: StringId,
    highest_blank_node: u32,
    phantom: PhantomData<OPS>,
}

fn translate<T>(t: &mut T, translation: &[StringId], datatrans: &[StringId])
where
    T: CompactTriple<u32>,
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
impl<SPO, OPS> GraphCreator<SPO, OPS>
where
    SPO: CompactTriple<u32>,
    OPS: CompactTriple<u32>,
{
    pub fn with_capacity(capacity: usize) -> GraphCreator<SPO, OPS> {
        let mut dlc = StringCollector::with_capacity(capacity);
        let lang_string_datatype_id = dlc.add_string(rdf::LANG_STRING);
        GraphCreator {
            graph_id: rand::random::<u32>(),
            string_collector: StringCollector::with_capacity(capacity),
            datatype_lang_collector: dlc,
            triples: Vec::new(),
            lang_string_datatype_id,
            highest_blank_node: 0,
            phantom: PhantomData,
        }
    }
    fn add_s_iri(&mut self, s: StringId, p: StringId, ot: TripleObjectType, o: u32, d: u32) {
        let triple = SPO::triple(true, s.id, p.id, ot, o, d);
        self.triples.push(triple);
    }
    fn add_iri_blank(&mut self, s: StringId, p: StringId, o: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, o);
        self.add_s_iri(s, p, TripleObjectType::BlankNode, o, 0);
    }
    fn add_iri_iri(&mut self, s: StringId, p: StringId, o: StringId) {
        self.add_s_iri(s, p, TripleObjectType::IRI, o.id, 0);
    }
    fn add_iri_lit(&mut self, s: StringId, p: StringId, o: StringId, datatype: StringId) {
        self.add_s_iri(s, p, TripleObjectType::Literal, o.id, datatype.id);
    }
    fn add_iri_lit_lang(&mut self, s: StringId, p: StringId, o: StringId, lang: StringId) {
        self.add_s_iri(s, p, TripleObjectType::LiteralLang, o.id, lang.id);
    }
    fn add_s_blank(&mut self, s: u32, p: StringId, ot: TripleObjectType, o: u32, d: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, s);
        let triple = SPO::triple(false, s, p.id, ot, o, d);
        self.triples.push(triple);
    }
    fn add_blank_blank(&mut self, s: u32, p: StringId, o: u32) {
        self.highest_blank_node = cmp::max(self.highest_blank_node, o);
        self.add_s_blank(s, p, TripleObjectType::BlankNode, o, 0);
    }
    fn add_blank_iri(&mut self, s: u32, p: StringId, o: StringId) {
        self.add_s_blank(s, p, TripleObjectType::IRI, o.id, 0);
    }
    fn add_blank_lit(&mut self, s: u32, p: StringId, o: StringId, datatype: StringId) {
        self.add_s_blank(s, p, TripleObjectType::Literal, o.id, datatype.id);
    }
    fn add_blank_lit_lang(&mut self, s: u32, p: StringId, o: StringId, lang: StringId) {
        self.add_s_blank(s, p, TripleObjectType::LiteralLang, o.id, lang.id);
    }
    fn check_blank_node(&self, blank_node: BlankNodePtr<SPO, OPS>) {
        assert_eq!(
            self.graph_id, blank_node.graph_id,
            "Blank node is not associated with this graph creator."
        );
    }
}

fn create_ops<SPO, OPS>(spo: &[SPO]) -> Vec<OPS>
where
    SPO: CompactTriple<u32>,
    OPS: CompactTriple<u32>,
{
    let mut ops = Vec::with_capacity(spo.len());
    for t in spo {
        ops.push(OPS::triple(
            t.subject_is_iri(),
            t.subject(),
            t.predicate(),
            t.object_type(),
            t.object(),
            t.datatype_or_lang(),
        ));
    }
    ops.sort_unstable();
    ops
}

#[derive(Clone)]
pub struct CreateIRI {
    iri: StringId,
}
pub struct CreateLiteral {
    lexical: StringId,
    datatype: StringId,
    language: Option<StringId>,
}
#[derive(Clone)]
pub struct CreateDatatype {
    datatype: StringId,
}
pub struct CreateLanguage {
    language: StringId,
}

impl<'g, SPO: 'g, OPS: 'g> graph::GraphWriter<'g> for GraphCreator<SPO, OPS>
where
    SPO: CompactTriple<u32>,
    OPS: CompactTriple<u32>,
{
    type BlankNode = BlankNodePtr<'g, SPO, OPS>;
    type IRI = CreateIRI;
    type Literal = CreateLiteral;
    type Datatype = CreateDatatype;
    type Language = CreateLanguage;
    type Graph = Graph<SPO, OPS>;
    fn create_blank_node(&mut self) -> BlankNodePtr<'g, SPO, OPS> {
        self.highest_blank_node += 1;
        BlankNodePtr {
            graph_id: self.graph_id,
            node_id: self.highest_blank_node,
            phantom: PhantomData,
        }
    }
    fn create_iri<'a, I: 'a>(&mut self, i: &I) -> CreateIRI
    where
        I: graph::IRIPtr<'a>,
    {
        CreateIRI {
            iri: self.string_collector.add_string(i.as_str()),
        }
    }
    fn create_literal<'a, L: 'a>(&mut self, l: &L) -> CreateLiteral
    where
        L: graph::LiteralPtr<'a>,
    {
        match l.language() {
            Some(language) => CreateLiteral {
                lexical: self.string_collector.add_string(l.as_str()),
                datatype: self.lang_string_datatype_id,
                language: Some(self.datatype_lang_collector.add_string(language)),
            },
            None => CreateLiteral {
                lexical: self.string_collector.add_string(l.as_str()),
                datatype: self.datatype_lang_collector.add_string(l.datatype_str()),
                language: None,
            },
        }
    }
    fn create_datatype(&mut self, datatype: &str) -> Self::Datatype {
        CreateDatatype {
            datatype: self.datatype_lang_collector.add_string(datatype),
        }
    }
    fn create_language(&mut self, language: &str) -> Self::Language {
        CreateLanguage {
            language: self.datatype_lang_collector.add_string(language),
        }
    }
    fn create_literal_datatype(&mut self, value: &str, datatype: &Self::Datatype) -> Self::Literal {
        CreateLiteral {
            lexical: self.string_collector.add_string(value),
            datatype: datatype.datatype,
            language: None,
        }
    }
    fn create_literal_language(&mut self, value: &str, language: &Self::Language) -> Self::Literal {
        CreateLiteral {
            lexical: self.string_collector.add_string(value),
            datatype: self.lang_string_datatype_id,
            language: Some(language.language),
        }
    }

    fn collect(mut self) -> Graph<SPO, OPS> {
        let (translation, string_collection) = self.string_collector.collect();
        let (datatrans, datatype_lang_collection) = self.datatype_lang_collector.collect();
        let mut spo = Vec::new();
        mem::swap(&mut spo, &mut self.triples);
        for t in &mut spo {
            translate(t, &translation, &datatrans);
        }
        // sort according to StringId, which is sorted alphabetically
        spo.sort_unstable();
        spo.dedup();
        spo.shrink_to_fit();
        let ops = create_ops(&spo);
        Graph {
            d: GraphData {
                graph_id: self.graph_id,
                strings: string_collection,
                datatype_or_lang: datatype_lang_collection,
                spo,
                ops,
                lang_string_datatype_id: datatrans[self.lang_string_datatype_id].id,
                highest_blank_node: self.highest_blank_node,
            },
        }
    }
    fn add_blank_blank(
        &mut self,
        subject: &BlankNodePtr<SPO, OPS>,
        predicate: &CreateIRI,
        object: &BlankNodePtr<SPO, OPS>,
    ) {
        self.check_blank_node(*subject);
        self.check_blank_node(*object);
        GraphCreator::add_blank_blank(self, subject.node_id, predicate.iri, object.node_id);
    }
    fn add_blank_iri(
        &mut self,
        subject: &BlankNodePtr<SPO, OPS>,
        predicate: &CreateIRI,
        object: &CreateIRI,
    ) {
        self.check_blank_node(*subject);
        GraphCreator::add_blank_iri(self, subject.node_id, predicate.iri, object.iri);
    }
    fn add_blank_literal(
        &mut self,
        subject: &BlankNodePtr<SPO, OPS>,
        predicate: &CreateIRI,
        object: &CreateLiteral,
    ) {
        self.check_blank_node(*subject);
        match object.language {
            Some(lang) => GraphCreator::add_blank_lit_lang(
                self,
                subject.node_id,
                predicate.iri,
                object.lexical,
                lang,
            ),
            None => GraphCreator::add_blank_lit(
                self,
                subject.node_id,
                predicate.iri,
                object.lexical,
                object.datatype,
            ),
        }
    }
    fn add_iri_blank(
        &mut self,
        subject: &CreateIRI,
        predicate: &CreateIRI,
        object: &BlankNodePtr<SPO, OPS>,
    ) {
        self.check_blank_node(*object);
        GraphCreator::add_iri_blank(self, subject.iri, predicate.iri, object.node_id);
    }
    fn add_iri_iri(&mut self, subject: &CreateIRI, predicate: &CreateIRI, object: &CreateIRI) {
        GraphCreator::add_iri_iri(self, subject.iri, predicate.iri, object.iri);
    }
    fn add_iri_literal(
        &mut self,
        subject: &CreateIRI,
        predicate: &CreateIRI,
        object: &CreateLiteral,
    ) {
        match object.language {
            Some(lang) => GraphCreator::add_iri_lit_lang(
                self,
                subject.iri,
                predicate.iri,
                object.lexical,
                lang,
            ),
            None => GraphCreator::add_iri_lit(
                self,
                subject.iri,
                predicate.iri,
                object.lexical,
                object.datatype,
            ),
        }
    }
}

#[test]
fn collect_empty() {
    let creator: GraphCreator<Triple64SPO, Triple64OPS> = GraphCreator::with_capacity(0);
    use crate::graph::GraphWriter;
    creator.collect();
}

#[test]
fn keep_blank_subject() {
    let mut creator: GraphCreator<Triple64SPO, Triple64OPS> = GraphCreator::with_capacity(0);
    use crate::graph::{Graph, GraphWriter, IRIPtr, Triple};
    let blank1 = creator.create_blank_node();
    let blank2 = creator.create_blank_node();
    let iri = creator.create_iri(&"");
    creator.add_blank_blank(blank1.node_id, iri.iri, blank2.node_id);
    let graph = creator.collect();
    let triple = graph.iter().next().unwrap();
    assert_eq!(
        triple.subject(),
        graph::BlankNodeOrIRI::BlankNode(blank1, PhantomData)
    );
    assert_eq!(triple.predicate().as_str(), "");
    assert_eq!(
        triple.object(),
        graph::Resource::BlankNode(blank2, PhantomData)
    );
}
