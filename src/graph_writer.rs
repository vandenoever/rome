use std::cmp;
use std::marker::PhantomData;
use std::mem;
use rand;
use compact_triple::*;
use grammar;
use graph;
use iter::sorted_iterator::SortedIterator;
use string_collector::*;
#[cfg(test)]
use triple64::*;

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

pub struct GraphData<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    graph_id: usize,
    strings: StringCollection,
    datatype_or_lang: StringCollection,
    spo: Vec<SPO>,
    ops: Vec<OPS>,
    highest_blank_node: u32,
}

pub struct Graph<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    d: GraphData<SPO, OPS>,
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
fn translate_object<T>(t: &mut T, translation: &Vec<u32>)
    where T: CompactTriple<u32>
{
    if !t.subject_is_iri() {
        let subject = t.subject() as usize;
        t.set_subject(translation[subject]);
    }
    if t.object_is_blank_node() {
        let object = t.object() as usize;
        t.set_object(translation[object]);
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

#[derive (Clone)]
pub enum SubjectPtr<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    IRI(&'g GraphData<SPO, OPS>, u32),
    BlankNode(&'g GraphData<SPO, OPS>, u32),
}
impl<'g, SPO, OPS> PartialEq for SubjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&SubjectPtr::IRI(_, a), &SubjectPtr::IRI(_, b)) => a == b,
            (&SubjectPtr::BlankNode(_, a), &SubjectPtr::BlankNode(_, b)) => a == b,
            _ => false,
        }
    }
}
impl<'g, SPO, OPS> Eq for SubjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> graph::SubjectPtr<'g> for SubjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn iri(&self) -> Option<&'g str> {
        match self {
            &SubjectPtr::IRI(ref graph, iri) => Some(graph.strings.get(StringId { id: iri })),
            _ => None,
        }
    }
}



#[derive (Clone)]
pub struct PredicatePtr<'g, SPO: 'g, OPS: 'g>(&'g GraphData<SPO, OPS>, u32)
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>;

impl<'g, SPO, OPS> PartialEq for PredicatePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}
impl<'g, SPO, OPS> Eq for PredicatePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> graph::PredicatePtr<'g> for PredicatePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn iri(&self) -> &str {
        self.0.strings.get(StringId { id: self.1 })
    }
}


#[derive (Clone)]
pub enum ObjectPtr<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    BlankNode(&'g GraphData<SPO, OPS>, u32),
    IRI(&'g GraphData<SPO, OPS>, u32),
    Literal(&'g GraphData<SPO, OPS>, u32),
}
impl<'g, SPO, OPS> PartialEq for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&ObjectPtr::IRI(_, a), &ObjectPtr::IRI(_, b)) => a == b,
            (&ObjectPtr::BlankNode(_, a), &ObjectPtr::BlankNode(_, b)) => a == b,
            (&ObjectPtr::Literal(_, a), &ObjectPtr::Literal(_, b)) => a == b,
            _ => false,
        }
    }
}
impl<'g, SPO, OPS> Eq for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> PartialOrd for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g, SPO, OPS> Ord for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (&ObjectPtr::BlankNode(_, a), &ObjectPtr::BlankNode(_, ref b)) => a.cmp(b),
            (&ObjectPtr::BlankNode(_, _), &ObjectPtr::IRI(_, _)) => cmp::Ordering::Less,
            (&ObjectPtr::BlankNode(_, _), &ObjectPtr::Literal(_, _)) => cmp::Ordering::Less,
            (&ObjectPtr::IRI(_, _), &ObjectPtr::BlankNode(_, _)) => cmp::Ordering::Greater,
            (&ObjectPtr::IRI(_, a), &ObjectPtr::IRI(_, ref b)) => a.cmp(b),
            (&ObjectPtr::IRI(_, _), &ObjectPtr::Literal(_, _)) => cmp::Ordering::Less,
            (&ObjectPtr::Literal(_, _), &ObjectPtr::BlankNode(_, _)) => cmp::Ordering::Greater,
            (&ObjectPtr::Literal(_, _), &ObjectPtr::IRI(_, _)) => cmp::Ordering::Greater,
            (&ObjectPtr::Literal(_, a), &ObjectPtr::Literal(_, ref b)) => a.cmp(b),
        }
    }
}
impl<'g, SPO, OPS> graph::SubjectPtr<'g> for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn iri(&self) -> Option<&'g str> {
        match self {
            &ObjectPtr::IRI(ref graph, iri) => Some(graph.strings.get(StringId { id: iri })),
            _ => None,
        }
    }
}
impl<'g, SPO: 'g, OPS: 'g> graph::ObjectPtr<'g> for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn literal(&self) -> Option<&str> {
        match self {
            &ObjectPtr::Literal(ref graph, l) => Some(graph.strings.get(StringId { id: l })),
            _ => None,
        }
    }
}
impl<'g, SPO: 'g, OPS: 'g> graph::IntoObject<'g> for ObjectPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn object(self) -> graph::Object<'g> {
        match self {
            ObjectPtr::IRI(_, _) => graph::Object::IRI(""),
            ObjectPtr::BlankNode(_, _) => graph::Object::IRI(""),
            ObjectPtr::Literal(_, _) => graph::Object::IRI(""),
        }
    }
}

pub struct Triple<'g, SPO: 'g, OPS: 'g, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    graph: &'g GraphData<SPO, OPS>,
    triple: T,
}

impl<'g, SPO, OPS, T> PartialEq for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        // if the triples use the same Graph, it's ok to compare
        // the numeric value of the triple, when Rc::ptr_eq becomes stable,
        // we can use that.
        use graph::Triple;
        self.subject().eq(&other.subject()) && self.predicate().eq(other.predicate()) &&
        self.object().eq(&other.object())
    }
}
impl<'g, SPO, OPS, T> Eq for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
}
impl<'g, SPO, OPS, T> PartialOrd for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g, SPO, OPS, T> Ord for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.triple.cmp(&other.triple)
    }
}

impl<'g, SPO: 'g, OPS: 'g, T> graph::Triple<'g> for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    type SubjectPtr = SubjectPtr<'g, SPO, OPS>;
    type PredicatePtr = PredicatePtr<'g, SPO, OPS>;
    type ObjectPtr = ObjectPtr<'g, SPO, OPS>;
    fn subject(&self) -> graph::Subject {
        if self.triple.subject_is_iri() {
            graph::Subject::IRI(self.graph.strings.get(StringId { id: self.triple.subject() }))
        } else {
            graph::Subject::BlankNode((self.triple.subject() as usize, 0))
        }
    }
    fn predicate(&self) -> &str {
        self.graph.strings.get(StringId { id: self.triple.predicate() })
    }
    fn object(&self) -> graph::Object {
        if self.triple.object_is_iri() {
            graph::Object::IRI(self.graph.strings.get(StringId { id: self.triple.object() }))
        } else if self.triple.object_is_blank_node() {
            graph::Object::BlankNode((self.triple.object() as usize, 0))
        } else if self.triple.has_language() {
            graph::Object::Literal(graph::Literal {
                lexical: self.graph.strings.get(StringId { id: self.triple.object() }),
                datatype: grammar::RDF_LANG_STRING,
                language: Some(self.graph
                    .datatype_or_lang
                    .get(StringId { id: self.triple.datatype_or_lang() })),
            })
        } else {
            graph::Object::Literal(graph::Literal {
                lexical: self.graph.strings.get(StringId { id: self.triple.object() }),
                datatype: self.graph
                    .datatype_or_lang
                    .get(StringId { id: self.triple.datatype_or_lang() }),
                language: None,
            })
        }
    }
    fn subject_ptr(&self) -> Self::SubjectPtr {
        if self.triple.subject_is_iri() {
            SubjectPtr::IRI(self.graph, self.triple.subject())
        } else {
            SubjectPtr::BlankNode(self.graph, self.triple.subject())
        }
    }
    fn object_ptr(&self) -> Self::ObjectPtr {
        if self.triple.object_is_iri() {
            ObjectPtr::IRI(self.graph, self.triple.object())
        } else if !self.triple.object_is_blank_node() {
            ObjectPtr::Literal(self.graph, self.triple.object())
        } else {
            ObjectPtr::BlankNode(self.graph, self.triple.object())
        }
    }
}

pub trait Index<SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    fn index(graph: &GraphData<SPO, OPS>) -> &Vec<T>;
}

pub struct SPOIndex<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    phantom: PhantomData<Graph<SPO, OPS>>,
}

impl<SPO, OPS> Index<SPO, OPS, SPO> for SPOIndex<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn index(graph: &GraphData<SPO, OPS>) -> &Vec<SPO> {
        &graph.spo
    }
}

pub struct OPSIndex<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    phantom: PhantomData<Graph<SPO, OPS>>,
}

impl<SPO, OPS> Index<SPO, OPS, OPS> for OPSIndex<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn index(graph: &GraphData<SPO, OPS>) -> &Vec<OPS> {
        &graph.ops
    }
}

pub struct GraphIterator<'g, SPO: 'g, OPS: 'g, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    graph: &'g GraphData<SPO, OPS>,
    pos: usize,
    phantom: PhantomData<(T, F)>,
}

impl<'g, SPO, OPS, T, F> Iterator for GraphIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
    type Item = Triple<'g, SPO, OPS, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < F::index(&self.graph).len() {
            let triple = F::index(&self.graph)[self.pos];
            self.pos += 1;
            Some(Triple {
                graph: self.graph.clone(),
                triple: triple,
            })
        } else {
            None
        }
    }
}
impl<'g, SPO, OPS, T, F> SortedIterator for GraphIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
}

pub struct TripleRangeIterator<'g, SPO: 'g, OPS: 'g, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    graph: &'g GraphData<SPO, OPS>,
    pos: usize,
    end: T,
    phantom: PhantomData<F>,
}

impl<'g, SPO, OPS, T, F> Iterator for TripleRangeIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
    type Item = Triple<'g, SPO, OPS, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < F::index(&self.graph).len() {
            let triple = F::index(&self.graph)[self.pos];
            self.pos += 1;
            if triple < self.end {
                Some(Triple {
                    graph: &self.graph,
                    triple: triple,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl<'g, SPO, OPS, T, F> SortedIterator for TripleRangeIterator<'g, SPO, OPS, T, F>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>,
          F: Index<SPO, OPS, T>
{
}

fn subject_blank_node<SPO>(subject: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(false, subject, 0, TripleObjectType::BlankNode, 0, 0)
}
fn subject_iri<SPO>(subject: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(true, subject, 0, TripleObjectType::BlankNode, 0, 0)
}
fn subject_blank_node_predicate<SPO>(subject: u32, predicate: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(false, subject, predicate, TripleObjectType::BlankNode, 0, 0)
}
fn subject_iri_predicate<SPO>(subject: u32, predicate: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(true, subject, predicate, TripleObjectType::BlankNode, 0, 0)
}
fn object_blank_node<OPS>(object: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::BlankNode, object, 0)
}
fn object_iri<OPS>(object: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::IRI, object, 0)
}
fn object_iri_predicate<OPS>(object: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(true, 0, predicate, TripleObjectType::IRI, object, 0)
}
fn object_blank_node_predicate<OPS>(object: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(true, 0, predicate, TripleObjectType::BlankNode, object, 0)
}
fn object_literal_predicate<OPS>(object: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(true, 0, predicate, TripleObjectType::Literal, object, 0)
}

type S<'g, SPO, OPS> = TripleRangeIterator<'g, SPO, OPS, SPO, SPOIndex<SPO, OPS>>;

impl<SPO, OPS> Graph<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn range_iter<T, F>(&self, start: T, end: T) -> TripleRangeIterator<SPO, OPS, T, F>
        where T: CompactTriple<u32>,
              F: Index<SPO, OPS, T>
    {
        let pos = match F::index(&self.d).binary_search(&start) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        TripleRangeIterator {
            graph: &self.d,
            pos: pos,
            end: end,
            phantom: PhantomData,
        }
    }
    fn empty_range_iter<T, F>(&self) -> TripleRangeIterator<SPO, OPS, T, F>
        where T: CompactTriple<u32>,
              F: Index<SPO, OPS, T>
    {
        let end = T::triple(true, 0, 0, TripleObjectType::BlankNode, 0, 0);
        TripleRangeIterator {
            graph: &self.d,
            pos: self.d.spo.len(),
            end: end,
            phantom: PhantomData,
        }
    }
    fn iter_subject_(&self, triple: SPO) -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_subject(triple.subject() + 1);
        self.range_iter(triple, end)
    }
    /// look up the iri or create a triple from the blank node
    fn create_subject_triple(&self, subject: &graph::Subject) -> Option<SPO> {
        match *subject {
            graph::Subject::IRI(iri) => {
                match self.d.strings.find(iri) {
                    None => None,
                    Some(id) => Some(subject_iri(id.id)),
                }
            }
            graph::Subject::BlankNode((n, graph)) if graph == self.d.graph_id => {
                Some(subject_blank_node(n as u32))
            }
            _ => None,
        }
    }
    /// iterator over all triples with the same subject
    pub fn iter_subject(&self,
                        subject: &graph::Subject)
                        -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        match self.create_subject_triple(subject) {
            Some(triple) => self.iter_subject_(triple),
            None => self.empty_range_iter(),
        }
    }
    /// iterator over all triples with the same subject
    pub fn iter_subject_iri(&self,
                            iri: &str)
                            -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        match self.d.strings.find(iri) {
            None => self.empty_range_iter(),
            Some(id) => {
                let triple = subject_iri(id.id);
                self.iter_subject_(triple)
            }
        }
    }
    /// iterator over all triples with the same object
    fn iter_object(&self, triple: OPS) -> TripleRangeIterator<SPO, OPS, OPS, OPSIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_object(triple.object() + 1);
        self.range_iter(triple, end)
    }
    /// iterator over all triples with the same object
    pub fn iter_object_iri(&self,
                           iri: &str)
                           -> TripleRangeIterator<SPO, OPS, OPS, OPSIndex<SPO, OPS>> {
        match self.d.strings.find(iri) {
            None => self.empty_range_iter(),
            Some(id) => {
                let triple = object_iri(id.id);
                self.iter_object(triple)
            }
        }
    }
    /// iterator over all triples with the same object and predicate
    fn iter_object_predicate(&self,
                             triple: OPS)
                             -> TripleRangeIterator<SPO, OPS, OPS, OPSIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_predicate(triple.predicate() + 1);
        self.range_iter(triple, end)
    }
    /// iterator over all triples with the same object and predicate
    pub fn iter_object_iri_predicate(&self,
                                     object_iri: &str,
                                     predicate: &str)
                                     -> TripleRangeIterator<SPO, OPS, OPS, OPSIndex<SPO, OPS>> {
        match self.d.strings.find(object_iri) {
            None => self.empty_range_iter(),
            Some(object) => {
                match self.d.strings.find(predicate) {
                    None => self.empty_range_iter(),
                    Some(predicate) => {
                        let triple = object_iri_predicate(object.id, predicate.id);
                        self.iter_object_predicate(triple)
                    }
                }
            }
        }
    }
    /// iterator over all triples with the same subject and predicate
    fn iter_subject_predicate__(&self,
                                triple: SPO)
                                -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_predicate(triple.predicate() + 1);
        self.range_iter(triple, end)
    }
    /// iterator over all triples with the same subject and predicate
    pub fn iter_subject_predicate_(&self,
                                   subject: &graph::Subject,
                                   predicate: &str)
                                   -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        match self.create_subject_triple(subject) {
            Some(mut triple) => {
                match self.d.strings.find(predicate) {
                    None => self.empty_range_iter(),
                    Some(StringId { id: predicate }) => {
                        triple.set_predicate(predicate);
                        self.iter_subject_predicate__(triple)
                    }
                }
            }
            None => self.empty_range_iter(),
        }
    }
    /// iterate over all triple with a blank node subject
    pub fn iter_subject_blank_nodes(&self)
                                    -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        let start = subject_blank_node(0);
        let end = subject_iri(0);
        self.range_iter(start, end)
    }
    /// iterate over all triple with a blank node object
    pub fn iter_object_blank_nodes(&self)
                                   -> TripleRangeIterator<SPO, OPS, OPS, OPSIndex<SPO, OPS>> {
        let start = object_blank_node(0);
        let end = object_iri(0);
        self.range_iter(start, end)
    }
    pub fn sort_blank_nodes(&self) -> Graph<SPO, OPS> {
        // sort nodes by usage (least used last)
        self.sort_blank_nodes_by(|b1, b2| {
            let mut cmp = b2.times_a_subject.cmp(&b1.times_a_subject);
            if cmp == cmp::Ordering::Equal {
                cmp = b2.times_a_subject_with_blank_object
                    .cmp(&b1.times_a_subject_with_blank_object);
            }
            if cmp == cmp::Ordering::Equal {
                cmp = b2.times_an_object.cmp(&b1.times_an_object);
            }
            if cmp == cmp::Ordering::Equal {
                cmp = b2.times_an_object_with_blank_subject
                    .cmp(&b1.times_an_object_with_blank_subject);
            }
            // if usage is equal compare the triples that the nodes are in
            if cmp == cmp::Ordering::Equal {
                let s1 = self.iter_subject_(subject_blank_node(b1.blank_node));
                let s2 = self.iter_subject_(subject_blank_node(b2.blank_node));
                cmp = s1.zip(s2)
                    .map(|(a, b)| compare_without_blank_nodes(a.triple, b.triple))
                    .find(|cmp| *cmp != cmp::Ordering::Equal)
                    .unwrap_or(cmp::Ordering::Equal);
            }
            if cmp == cmp::Ordering::Equal {
                let o1 = self.iter_object(object_blank_node(b1.blank_node));
                let o2 = self.iter_object(object_blank_node(b2.blank_node));
                cmp = o1.zip(o2)
                    .map(|(a, b)| compare_without_blank_nodes(a.triple, b.triple))
                    .find(|cmp| *cmp != cmp::Ordering::Equal)
                    .unwrap_or(cmp::Ordering::Equal);
            }
            cmp
        })
    }
    fn sort_blank_nodes_by<F>(&self, compare: F) -> Graph<SPO, OPS>
        where F: FnMut(&BlankNodeInfo, &BlankNodeInfo) -> cmp::Ordering
    {
        let len = self.d.highest_blank_node as usize + 1;
        let mut blank_info = Vec::with_capacity(len);
        for i in 0..len {
            blank_info.push(BlankNodeInfo {
                blank_node: i as u32,
                times_a_subject: 0,
                times_a_subject_with_blank_object: 0,
                times_an_object: 0,
                times_an_object_with_blank_subject: 0,
            })
        }
        // collection information on the blank nodes
        for t in self.iter_subject_blank_nodes() {
            let i = &mut blank_info[t.triple.subject() as usize];
            i.times_a_subject += 1;
            if t.triple.object_is_blank_node() {
                i.times_a_subject_with_blank_object += 1;
            }
        }
        for t in self.iter_object_blank_nodes() {
            let i = &mut blank_info[t.triple.object() as usize];
            i.times_an_object += 1;
            if !t.triple.subject_is_iri() {
                i.times_an_object_with_blank_subject += 1;
            }
        }
        // sort the vector
        blank_info.sort_by(compare);
        let mut translation = vec![0 as u32;len];
        for i in 0..len {
            translation[blank_info[i].blank_node as usize] = i as u32;
        }
        blank_info.clear();
        blank_info.shrink_to_fit();

        // translate the blank nodes in spo and ops
        let mut spo = self.d.spo.clone();
        for t in spo.iter_mut() {
            translate_object(t, &translation);
        }
        spo.sort();
        let mut ops = self.d.ops.clone();
        for t in ops.iter_mut() {
            translate_object(t, &translation);
        }
        ops.sort();

        Graph {
            d: GraphData {
                graph_id: self.d.graph_id,
                strings: self.d.strings.clone(),
                datatype_or_lang: self.d.datatype_or_lang.clone(),
                spo: spo,
                ops: ops,
                highest_blank_node: self.d.highest_blank_node,
            },
        }
    }
}

fn zero_blank_nodes<T>(a: &mut T)
    where T: CompactTriple<u32>
{
    if !a.subject_is_iri() {
        a.set_subject(0);
    }
    if a.object_is_blank_node() {
        a.set_subject(0);
    }
}

fn compare_without_blank_nodes<T>(mut a: T, mut b: T) -> cmp::Ordering
    where T: CompactTriple<u32>
{
    zero_blank_nodes(&mut a);
    zero_blank_nodes(&mut b);
    a.cmp(&b)
}

struct BlankNodeInfo {
    blank_node: u32,
    times_a_subject: u32,
    times_a_subject_with_blank_object: u32,
    times_an_object: u32,
    times_an_object_with_blank_subject: u32,
}

impl<'g, SPO: 'g, OPS: 'g> graph::Graph<'g> for Graph<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    type SubjectPtr = SubjectPtr<'g, SPO, OPS>;
    type PredicatePtr = PredicatePtr<'g, SPO, OPS>;
    type ObjectPtr = ObjectPtr<'g, SPO, OPS>;
    type SPOTriple = Triple<'g, SPO, OPS, SPO>;
    type SPOIter = GraphIterator<'g, SPO, OPS, SPO, SPOIndex<SPO, OPS>>;
    type SPORangeIter = TripleRangeIterator<'g, SPO, OPS, SPO, SPOIndex<SPO, OPS>>;
    type OPSTriple = Triple<'g, SPO, OPS, OPS>;
    type OPSRangeIter = TripleRangeIterator<'g, SPO, OPS, OPS, OPSIndex<SPO, OPS>>;
    fn iter(&'g self) -> Self::SPOIter {
        GraphIterator {
            graph: &self.d,
            pos: 0,
            phantom: PhantomData,
        }
    }
    fn subject_ptr<'a, S>(&'g self, subject: S) -> Option<Self::SubjectPtr>
        where S: graph::IntoSubject<'a>
    {
        match subject.subject() {
            graph::Subject::IRI(iri) => {
                self.d.strings.find(iri).map(|s| SubjectPtr::IRI(&self.d, s.id))
            }
            graph::Subject::BlankNode(b) if b.1 == self.d.graph_id => {
                Some(SubjectPtr::BlankNode(&self.d, b.0 as u32))
            }
            _ => None,
        }
    }
    fn predicate_ptr<'a>(&'g self, predicate: &str) -> Option<Self::PredicatePtr> {
        self.d.strings.find(predicate).map(|s| PredicatePtr(&self.d, s.id))
    }
    fn object_ptr<'a, O>(&'g self, object: O) -> Option<Self::ObjectPtr>
        where O: graph::IntoObject<'a>
    {
        match object.object() {
            graph::Object::IRI(iri) => {
                self.d.strings.find(iri).map(|s| ObjectPtr::IRI(&self.d, s.id))
            }
            graph::Object::BlankNode(b) if b.1 == self.d.graph_id => {
                Some(ObjectPtr::BlankNode(&self.d, b.0 as u32))
            }
            _ => None,
        }
    }
    fn object_to_subject(&self, object: Self::ObjectPtr) -> Option<Self::SubjectPtr> {
        match object {
            ObjectPtr::IRI(graph, iri) => Some(SubjectPtr::IRI(graph, iri)),
            ObjectPtr::BlankNode(graph, bn) => Some(SubjectPtr::BlankNode(graph, bn)),
            _ => None,
        }
    }
    fn object_to_predicate(&self, object: Self::ObjectPtr) -> Option<Self::PredicatePtr> {
        match object {
            ObjectPtr::IRI(graph, iri) => Some(PredicatePtr(graph, iri)),
            _ => None,
        }
    }
    fn subject_to_object(&self, subject: Self::SubjectPtr) -> Self::ObjectPtr {
        match subject {
            SubjectPtr::IRI(graph, iri) => ObjectPtr::IRI(graph, iri),
            SubjectPtr::BlankNode(graph, bn) => ObjectPtr::BlankNode(graph, bn),
        }
    }
    fn predicate_to_object(&self, predicate: Self::PredicatePtr) -> Self::ObjectPtr {
        ObjectPtr::IRI(predicate.0, predicate.1)
    }
    fn iter_s_p(&'g self,
                subject: Self::SubjectPtr,
                predicate: Self::PredicatePtr)
                -> Self::SPORangeIter {
        let spo = match subject {
            SubjectPtr::IRI(_, iri) => subject_iri_predicate(iri, predicate.1),
            SubjectPtr::BlankNode(_, bn) => subject_blank_node_predicate(bn, predicate.1),
        };
        self.iter_subject_predicate__(spo)
    }
    fn iter_o_p(&'g self,
                object: Self::ObjectPtr,
                predicate: Self::PredicatePtr)
                -> Self::OPSRangeIter {
        let ops = match object {
            ObjectPtr::IRI(_, iri) => object_iri_predicate(iri, predicate.1),
            ObjectPtr::BlankNode(_, bn) => object_blank_node_predicate(bn, predicate.1),
            ObjectPtr::Literal(_, l) => object_literal_predicate(l, predicate.1),
        };
        self.iter_object_predicate(ops)
    }
    fn iter_subject_predicate(&'g self,
                              subject: &graph::Subject,
                              predicate: &str)
                              -> Self::SPORangeIter {
        self.iter_subject_predicate_(subject, predicate)
    }
    fn empty_spo_range(&'g self) -> Self::SPORangeIter {
        self.empty_range_iter()
    }
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter {
        self.empty_range_iter()
    }
    fn len(&self) -> usize {
        self.d.spo.len()
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
