use std::cmp;
use std::marker::PhantomData;
use std::fmt;
use graph;
use super::compact_triple::*;
use super::graph::*;
use super::string_collector::*;

#[derive (Clone,Copy,Eq,PartialOrd,Ord)]
pub struct BlankNodePtr<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub graph_id: u32,
    pub node_id: u32,
    pub phantom: PhantomData<&'g (SPO, OPS)>,
}
impl<'g, SPO, OPS> PartialEq for BlankNodePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        // TODO: figure if we want PartialEq and Eq for nodes at all
        // (probably only PartialEq)
        self.node_id == other.node_id && self.graph_id == other.graph_id
    }
}
impl<'g, SPO, OPS> fmt::Display for BlankNodePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.node_id)
    }
}
impl<'g, SPO, OPS> graph::BlankNodePtr<'g> for BlankNodePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> fmt::Debug for BlankNodePtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "_:{}", self.node_id)
    }
}
#[derive (Clone)]
pub struct IRIPtr<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub graph: &'g GraphData<SPO, OPS>,
    pub iri: u32,
}
impl<'g, SPO, OPS> PartialEq for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        if self.graph.graph_id == other.graph.graph_id {
            self.iri == other.iri
        } else {
            use graph::IRIPtr;
            self.as_str() == other.as_str()
        }
    }
}
impl<'g, SPO, OPS> Eq for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> PartialOrd for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g, SPO, OPS> Ord for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.iri.cmp(&other.iri)
    }
}
impl<'g, SPO, OPS> graph::IRIPtr<'g> for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn as_str(&self) -> &str {
        self.graph.strings.get(StringId { id: self.iri })
    }
}
impl<'g, SPO, OPS> fmt::Debug for IRIPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use graph::IRIPtr;
        write!(f, "<{}>", self.as_str())
    }
}
#[derive (Clone)]
pub struct LiteralPtr<'g, SPO: 'g, OPS: 'g>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    pub graph: &'g GraphData<SPO, OPS>,
    pub lexical: u32,
    pub datatype: u32,
    pub language: Option<u32>,
}
impl<'g, SPO, OPS> PartialEq for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn eq(&self, other: &Self) -> bool {
        if self.graph.graph_id == other.graph.graph_id {
            self.lexical == other.lexical && self.datatype == other.datatype &&
            self.language == other.language
        } else {
            use graph::LiteralPtr;
            self.as_str() == other.as_str() && self.datatype() == other.datatype() &&
            self.language() == other.language()
        }
    }
}
impl<'g, SPO, OPS> Eq for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
}
impl<'g, SPO, OPS> PartialOrd for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g, SPO, OPS> Ord for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.lexical.cmp(&other.lexical)
    }
}
impl<'g, SPO, OPS> graph::LiteralPtr<'g> for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn as_str(&self) -> &str {
        self.graph.strings.get(StringId { id: self.lexical })
    }
    fn datatype(&self) -> &str {
        self.graph.datatype_or_lang.get(StringId { id: self.datatype })
    }
    fn language(&self) -> Option<&str> {
        self.language.map(|l| self.graph.datatype_or_lang.get(StringId { id: l }))
    }
}
impl<'g, SPO, OPS> fmt::Debug for LiteralPtr<'g, SPO, OPS>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use graph::LiteralPtr;
        match self.language() {
            None => write!(f, "\"{}\"^^<{}>", self.as_str(), self.datatype()),
            Some(lang) => write!(f, "\"{}\"@{}", self.as_str(), lang),
        }
    }
}

pub type BlankNodeOrIRI<'t, SPO, OPS> = graph::BlankNodeOrIRI<'t,
                                                              BlankNodePtr<'t, SPO, OPS>,
                                                              IRIPtr<'t, SPO, OPS>>;
pub type Resource<'t, SPO, OPS> = graph::Resource<'t,
                                                  BlankNodePtr<'t, SPO, OPS>,
                                                  IRIPtr<'t, SPO, OPS>,
                                                  LiteralPtr<'t, SPO, OPS>>;

#[derive(Clone)]
pub struct Triple<'g, SPO: 'g, OPS: 'g, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    pub graph: &'g GraphData<SPO, OPS>,
    pub triple: T,
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
        self.subject().eq(&other.subject()) && self.predicate().eq(&other.predicate()) &&
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

impl<'g, SPO: 'g, OPS: 'g, T> graph::Triple<'g,
                                              BlankNodePtr<'g, SPO, OPS>,
                                              IRIPtr<'g, SPO, OPS>,
                                              LiteralPtr<'g, SPO, OPS>> for Triple<'g, SPO, OPS, T>
    where SPO: CompactTriple<u32>,
          OPS: CompactTriple<u32>,
          T: CompactTriple<u32>
{
    fn subject(&self) -> BlankNodeOrIRI<'g,SPO,OPS> {
        if self.triple.subject_is_iri() {
            graph::BlankNodeOrIRI::IRI(IRIPtr{graph:self.graph, iri: self.triple.subject() })
        } else {
            graph::BlankNodeOrIRI::BlankNode(BlankNodePtr{
                    graph_id:self.graph.graph_id,
                    node_id: self.triple.subject(),phantom:PhantomData },
                PhantomData)
        }
    }
    fn predicate(&self) -> IRIPtr<'g,SPO,OPS> {
        IRIPtr{graph:self.graph, iri: self.triple.predicate() }
    }
    fn object(&self) -> Resource<'g,SPO,OPS> {
        if self.triple.object_is_iri() {
            graph::Resource::IRI(IRIPtr{graph:self.graph, iri: self.triple.object() })
        } else if self.triple.object_is_blank_node() {
            graph::Resource::BlankNode(BlankNodePtr{
                    graph_id: self.graph.graph_id,
                    node_id: self.triple.object(),phantom:PhantomData
                },PhantomData)
        } else if self.triple.has_language() {
            graph::Resource::Literal(LiteralPtr {
                graph: self.graph,
                lexical: self.triple.object(),
                datatype: self.graph.lang_string_datatype_id,
                language: Some(self.triple.datatype_or_lang()),
            })
        } else {
            graph::Resource::Literal(LiteralPtr {
                graph: self.graph,
                lexical: self.triple.object(),
                datatype: self.triple.datatype_or_lang(),
                language: None,
            })
        }
    }
}
