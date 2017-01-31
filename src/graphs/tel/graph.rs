use graph;
use std::cmp;
use std::marker::PhantomData;
use super::compact_triple::*;
use super::iter::*;
use super::string_collector::*;
use super::triple::*;

pub struct GraphData<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub graph_id: u32,
    pub strings: StringCollection,
    pub datatype_or_lang: StringCollection,
    pub spo: Vec<SPO>,
    pub ops: Vec<OPS>,
    pub lang_string_datatype_id: u32,
    pub highest_blank_node: u32,
}

pub struct Graph<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub d: GraphData<SPO, OPS>,
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

// get the first possible triple with the given blank node subject id
fn subject_blank_node<SPO>(subject: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(false, subject, 0, TripleObjectType::BlankNode, 0, 0)
}
// get the first possible triple with the given iri subject id
fn subject_iri<SPO>(subject: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(true, subject, 0, TripleObjectType::BlankNode, 0, 0)
}
// get the first possible triple with the given blank node subject id and predicate
fn subject_blank_node_predicate<SPO>(subject: u32, predicate: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(false, subject, predicate, TripleObjectType::BlankNode, 0, 0)
}
// get the first possible triple with the given iri subject id and predicate
fn subject_iri_predicate<SPO>(subject: u32, predicate: u32) -> SPO
    where SPO: CompactTriple<u32>
{
    SPO::triple(true, subject, predicate, TripleObjectType::BlankNode, 0, 0)
}
// get the first possible triple with the given blank node object id
fn object_blank_node<OPS>(object: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::BlankNode, object, 0)
}
// get the first possible triple with the given iri object id
fn object_iri<OPS>(object: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::IRI, object, 0)
}
// get the first possible triple with the given literal object id and datatype
fn object_literal<OPS>(object: u32, datatype: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::Literal, object, datatype)
}
// get the first possible triple with the given literal object id and language
fn object_literal_lang<OPS>(object: u32, lang: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, 0, TripleObjectType::LiteralLang, object, lang)
}
// get the first possible triple with the given blank node object id and predicate
fn object_blank_node_predicate<OPS>(object: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, predicate, TripleObjectType::BlankNode, object, 0)
}
// get the first possible triple with the given iri object id and predicate
fn object_iri_predicate<OPS>(object: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false, 0, predicate, TripleObjectType::IRI, object, 0)
}
// get the first possible triple with the given literal object id and datatype and predicate
fn object_literal_predicate<OPS>(object: u32, datatype: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false,
                0,
                predicate,
                TripleObjectType::Literal,
                object,
                datatype)
}
// get the first possible triple with the given literal object id and language and predicate
fn object_literal_lang_predicate<OPS>(object: u32, lang: u32, predicate: u32) -> OPS
    where OPS: CompactTriple<u32>
{
    OPS::triple(false,
                0,
                predicate,
                TripleObjectType::LiteralLang,
                object,
                lang)
}

impl<SPO, OPS> Graph<SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn range_iter<T, F>(&self, start: T, end: T) -> TripleRangeIterator<SPO, OPS, T, F>
        where T: CompactTriple<u32>,
              F: Index<SPO, OPS, T>
    {
        let pos = match F::index(&self.d).binary_search(&start) {
            Ok(pos) | Err(pos) => pos,
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
    fn iter_subject(&self, triple: SPO) -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_subject(triple.subject() + 1);
        self.range_iter(triple, end)
    }
    /// iterator over all triples with the same subject
    pub fn iter_subject_iri(&self,
                            iri: &str)
                            -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        match self.d.strings.find(iri) {
            None => self.empty_range_iter(),
            Some(id) => {
                let triple = subject_iri(id.id);
                self.iter_subject(triple)
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
    fn iter_subject_predicate(&self,
                              triple: SPO)
                              -> TripleRangeIterator<SPO, OPS, SPO, SPOIndex<SPO, OPS>> {
        let mut end = triple;
        end.set_predicate(triple.predicate() + 1);
        self.range_iter(triple, end)
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
                let s1 = self.iter_subject(subject_blank_node(b1.blank_node));
                let s2 = self.iter_subject(subject_blank_node(b2.blank_node));
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
        for (i, b) in blank_info.iter().enumerate().take(len) {
            translation[b.blank_node as usize] = i as u32;
        }
        blank_info.clear();
        blank_info.shrink_to_fit();

        // translate the blank nodes in spo and ops
        let mut spo = self.d.spo.clone();
        for t in &mut spo {
            translate_object(t, &translation);
        }
        spo.sort();
        let mut ops = self.d.ops.clone();
        for t in &mut ops {
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
                lang_string_datatype_id: self.d.lang_string_datatype_id,
            },
        }
    }
    pub fn len(&self) -> usize {
        self.d.spo.len()
    }
}

fn translate_object<T>(t: &mut T, translation: &[u32])
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
    type BlankNodePtr = BlankNodePtr<'g, SPO, OPS>;
    type IRIPtr = IRIPtr<'g, SPO, OPS>;
    type LiteralPtr = LiteralPtr<'g, SPO, OPS>;
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
    fn find_iri(&'g self, iri: &str) -> Option<Self::IRIPtr> {
        self.d.strings.find(iri).map(|s| {
            IRIPtr {
                graph: &self.d,
                iri: s.id,
            }
        })
    }
    fn find_literal<'a>(&'g self,
                        literal: &'a str,
                        datatype: &'a str,
                        language: Option<&'a str>)
                        -> Option<Self::LiteralPtr> {
        if let Some(l) = self.d.strings.find(literal) {
            match language.and_then(|l| self.d.datatype_or_lang.find(l)) {
                Some(lang) => {
                    Some(LiteralPtr {
                        graph: &self.d,
                        lexical: l.id,
                        datatype: self.d.lang_string_datatype_id,
                        language: Some(lang.id),
                    })
                }
                None => {
                    self.d.datatype_or_lang.find(datatype).map(|d| {
                        LiteralPtr {
                            graph: &self.d,
                            lexical: l.id,
                            datatype: d.id,
                            language: None,
                        }
                    })
                }
            }
        } else {
            None
        }
    }
    fn find_datatype<'a>(&'g self, datatype: &'a str) -> Option<DatatypePtr<'g, SPO, OPS>> {
        self.d.datatype_or_lang.find(datatype).map(|d| {
            DatatypePtr {
                graph: &self.d,
                datatype: d.id,
            }
        })
    }
    fn iter_s(&'g self, subject: &BlankNodeOrIRI<'g, SPO, OPS>) -> Self::SPORangeIter {
        let spo = match *subject {
            graph::BlankNodeOrIRI::BlankNode(bn, _) => subject_blank_node(bn.node_id),
            graph::BlankNodeOrIRI::IRI(ref iri) => subject_iri(iri.iri),
        };
        self.iter_subject(spo)
    }
    fn iter_s_p(&'g self,
                subject: &BlankNodeOrIRI<'g, SPO, OPS>,
                predicate: &IRIPtr<'g, SPO, OPS>)
                -> Self::SPORangeIter {
        let spo = match *subject {
            graph::BlankNodeOrIRI::BlankNode(bn, _) => {
                subject_blank_node_predicate(bn.node_id, predicate.iri)
            }
            graph::BlankNodeOrIRI::IRI(ref iri) => subject_iri_predicate(iri.iri, predicate.iri),
        };
        self.iter_subject_predicate(spo)
    }
    fn iter_o(&'g self, object: &Resource<'g, SPO, OPS>) -> Self::OPSRangeIter {
        let ops = match *object {
            graph::Resource::BlankNode(bn, _) => object_blank_node(bn.node_id),
            graph::Resource::IRI(ref iri) => object_iri(iri.iri),
            graph::Resource::Literal(ref l) => {
                match l.language {
                    Some(lang) => object_literal_lang(l.lexical, lang),
                    None => object_literal(l.lexical, l.datatype),
                }
            }
        };
        self.iter_object(ops)
    }
    fn iter_o_p(&'g self,
                object: &Resource<'g, SPO, OPS>,
                predicate: &IRIPtr<'g, SPO, OPS>)
                -> Self::OPSRangeIter {
        let ops = match *object {
            graph::Resource::BlankNode(bn, _) => {
                object_blank_node_predicate(bn.node_id, predicate.iri)
            }
            graph::Resource::IRI(ref iri) => object_iri_predicate(iri.iri, predicate.iri),
            graph::Resource::Literal(ref l) => {
                match l.language {
                    Some(lang) => object_literal_lang_predicate(l.lexical, lang, predicate.iri),
                    None => object_literal_predicate(l.lexical, l.datatype, predicate.iri),
                }
            }
        };
        self.iter_object_predicate(ops)
    }
    fn empty_spo_range(&'g self) -> Self::SPORangeIter {
        self.empty_range_iter()
    }
    fn empty_ops_range(&'g self) -> Self::OPSRangeIter {
        self.empty_range_iter()
    }
}
