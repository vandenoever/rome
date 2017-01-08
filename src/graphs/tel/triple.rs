use std::cmp;
use graph;
use constants;
use super::compact_triple::*;
use super::graph::*;
use super::string_collector::*;

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
                datatype: constants::RDF_LANG_STRING,
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
    fn predicate_ptr(&self) -> Self::PredicatePtr {
        PredicatePtr(self.graph, self.triple.predicate())
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
pub struct PredicatePtr<'g, SPO: 'g, OPS: 'g>(pub &'g GraphData<SPO, OPS>, pub u32)
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
