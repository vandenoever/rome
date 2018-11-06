use super::grammar::{boolean, decimal, double, integer, pn_local};
use super::grammar_structs::Literal;
use error::{Error, Result};
use graph::*;
use namespaces::*;
use nom::types::CompleteStr;
use nom::IResult;
use ontology::iri::{rdf, xsd};
use std::fmt::Display;
use std::io::Write;
use std::iter::Peekable;

struct TurtleWriter<'a, 'g, W: 'a, G: 'g>
where
    W: Write,
    G: Graph<'g>,
{
    buffer: Vec<u8>,
    base: String,
    writer: &'a mut W,
    xsd_string: Option<<G::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    xsd_boolean: Option<<G::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    xsd_integer: Option<<G::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    xsd_decimal: Option<<G::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    xsd_double: Option<<G::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    rdf_first: Option<G::IRIPtr>,
    rdf_nil: Option<G::IRIPtr>,
    rdf_rest: Option<G::IRIPtr>,
    rdf_type: Option<G::IRIPtr>,
    graph: &'g G,
}

/// Write out triples as pretty turtle.
pub fn write_pretty_turtle<'g, G: 'g, W>(
    namespaces: &Namespaces,
    graph: &'g G,
    writer: &mut W,
) -> Result<()>
where
    G: Graph<'g>,
    <G as Graph<'g>>::BlankNodePtr: Display,
    W: Write,
{
    let mut writer = TurtleWriter::<_, G> {
        buffer: Vec::new(),
        base: String::new(),
        writer,
        xsd_string: graph.find_datatype(xsd::STRING),
        xsd_boolean: graph.find_datatype(xsd::BOOLEAN),
        xsd_integer: graph.find_datatype(xsd::INTEGER),
        xsd_decimal: graph.find_datatype(xsd::DECIMAL),
        xsd_double: graph.find_datatype(xsd::DOUBLE),
        rdf_first: graph.find_iri(rdf::FIRST),
        rdf_nil: graph.find_iri(rdf::NIL),
        rdf_rest: graph.find_iri(rdf::REST),
        rdf_type: graph.find_iri(rdf::TYPE),
        graph,
    };
    for ns in namespaces.iter() {
        writer.write_prefix(ns)?;
    }
    writer.writer.write_all(b"\n")?;
    writer.write_statements(namespaces)
}

impl<'a, 'g, W: 'a, G: 'g> TurtleWriter<'a, 'g, W, G>
where
    W: Write,
    G: Graph<'g>,
    <G as Graph<'g>>::BlankNodePtr: Display,
{
    fn write_prefix(&mut self, ns: &Namespace) -> Result<()> {
        self.writer.write_all(b"@prefix ")?;
        self.writer.write_all(ns.prefix().as_bytes())?;
        self.writer.write_all(b":\t")?;
        self.write_full_iri(ns.namespace())?;
        self.writer.write_all(b" .\n")?;
        Ok(())
    }
    fn write_iri_str(&mut self, iri: &str, namespaces: &Namespaces) -> Result<()> {
        match namespaces.find_prefix(iri) {
            Some((prefix, local)) => {
                if let Ok((CompleteStr(""), _)) = pn_local(CompleteStr(local)) {
                    self.write_prefixed_iri(prefix, local)
                } else {
                    self.write_full_iri(iri)
                }
            }
            None => self.write_full_iri(iri),
        }
    }
    fn write_iri(&mut self, iri: &G::IRIPtr, namespaces: &Namespaces) -> Result<()> {
        if Some(iri) == self.rdf_nil.as_ref() {
            self.writer.write_all(b"()")?;
        } else {
            self.write_iri_str(iri.as_str(), namespaces)?;
        }
        Ok(())
    }
    fn write_prefixed_iri(&mut self, prefix: &str, iri: &str) -> Result<()> {
        self.writer.write_all(prefix.as_bytes())?;
        self.writer.write_all(b":")?;
        self.writer.write_all(iri.as_bytes())?;
        Ok(())
    }
    fn write_full_iri(&mut self, mut iri: &str) -> Result<()> {
        if iri.starts_with(self.base.as_str()) {
            iri = &iri[self.base.len()..];
        }
        self.writer.write_all(b"<")?;
        self.buffer.clear();
        for b in iri.as_bytes() {
            if *b < 20 || b"<>\"{}|^`\\".contains(b) {
                write!(&mut self.buffer, "\\u00{:X}", *b).unwrap();
            } else {
                self.buffer.push(*b);
            }
        }
        self.writer.write_all(&self.buffer[..])?;
        self.writer.write_all(b">")?;
        Ok(())
    }
    fn write_blank_node(&mut self, blank_node: G::BlankNodePtr) -> Result<()> {
        self.writer.write_all(b"_:")?;
        write!(self.writer, "{}", blank_node)?;
        Ok(())
    }
    fn write_literal_value(&mut self, value: &str) -> Result<()> {
        self.buffer.clear();
        for b in value.as_bytes() {
            if *b == 0x22 || *b == 0x5C || *b == 0x0A || *b == 0x0D {
                self.buffer.push(b'\\');
            }
            self.buffer.push(*b);
        }
        self.writer.write_all(&self.buffer[..])?;
        Ok(())
    }
    fn write_literal(&mut self, literal: &G::LiteralPtr, namespaces: &Namespaces) -> Result<()> {
        let d = Some(literal.datatype());
        let v = literal.as_str();
        // if the literal matches the unquoted production for its datatype,
        // print without quotes
        let mut unquoted = false;
        for i in &[
            (
                &self.xsd_boolean,
                boolean as fn(CompleteStr) -> IResult<CompleteStr, Literal>,
            ),
            (&self.xsd_integer, integer),
            (&self.xsd_decimal, decimal),
            (&self.xsd_double, double),
        ] {
            if &d == i.0 {
                if let Ok((CompleteStr(""), _)) = (i.1)(CompleteStr(v)) {
                    unquoted = true;
                    break;
                }
            }
        }
        if unquoted {
            self.write_literal_value(literal.as_str())?;
        } else {
            self.writer.write_all(b"\"")?;
            self.write_literal_value(literal.as_str())?;
            self.writer.write_all(b"\"")?;
            if let Some(langtag) = literal.language() {
                self.writer.write_all(b"@")?;
                self.writer.write_all(langtag.as_bytes())?;
            } else if d != self.xsd_string {
                self.writer.write_all(b"^^")?;
                self.write_iri_str(literal.datatype_str(), namespaces)?;
            }
        }
        Ok(())
    }
    fn write_predicate(&mut self, predicate: &G::IRIPtr, namespaces: &Namespaces) -> Result<()> {
        if Some(predicate) == self.rdf_type.as_ref() {
            self.writer.write_all(b"a")?;
        } else {
            self.write_iri_str(predicate.as_str(), namespaces)?;
        }
        Ok(())
    }
    fn write_collection(
        &mut self,
        mut triple: G::SPOTriple,
        mut iter: G::SPORangeIter,
        namespaces: &Namespaces,
    ) -> Result<()> {
        loop {
            // write rdf:first value
            self.write_object(triple.object(), namespaces)?;
            let rest = iter
                .next()
                .ok_or_else(|| Error::Custom("An rdf:rest triple was expected."))?;
            if Some(rest.predicate()) != self.rdf_rest {
                return Err(Error::Custom("An rdf:rest triple was expected."));
            }
            if rest.object().as_iri() == self.rdf_nil.as_ref() {
                if iter.next().is_some() {
                    return Err(Error::Custom(
                        "No more triples were expected for the list node.",
                    ));
                }
                self.writer.write_all(b")")?;
                return Ok(());
            }
            let rest = rest
                .object()
                .as_blank_node()
                .ok_or_else(|| Error::Custom("A blank node was expected."))?
                .clone();
            if iter.next().is_some() {
                return Err(Error::Custom(
                    "No more triples were expected for the list node.",
                ));
            }
            self.writer.write_all(b"\n\t\t\t")?;
            iter = self.graph.iter_s(&rest.to_blank_node_or_iri());
            triple = iter.next().expect("The list node should have properties.");
            if Some(triple.predicate()) != self.rdf_first {
                return Err(Error::Custom("An rdf:first triple was expected."));
            }
        }
    }
    fn write_object(
        &mut self,
        object: Resource<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
        namespaces: &Namespaces,
    ) -> Result<()> {
        match object {
            Resource::BlankNode(blank_node, _) => {
                // check how often the node is used as an object
                {
                    let mut object_iter = self.graph.iter_o(&blank_node.to_resource());
                    object_iter
                        .next()
                        .expect("Implementation error. There should be at least one triple.");
                    if object_iter.next().is_some() {
                        // blank node is used more than once, cannot be anonymous
                        return self.write_blank_node(blank_node);
                    }
                }
                // blank node is used as object only once
                let mut subject_iter = self.graph.iter_s(&blank_node.to_blank_node_or_iri());
                match subject_iter.next() {
                    None => {
                        // blank node has no properties, just write []
                        self.writer.write_all(b"[]")?;
                    }
                    Some(triple) => {
                        let predicate_iri = triple.predicate();
                        if Some(predicate_iri) == self.rdf_first {
                            self.writer.write_all(b"(")?;
                            self.write_collection(triple, subject_iter, namespaces)?;
                        } else {
                            self.writer.write_all(b"[")?;
                            let mut peekable = subject_iter.peekable();
                            self.write_predicate_object_list(&triple, &mut peekable, namespaces)?;
                            self.writer.write_all(b"]")?;
                        }
                    }
                }
            }
            Resource::IRI(iri) => self.write_iri(&iri, namespaces)?,
            Resource::Literal(literal) => self.write_literal(&literal, namespaces)?,
        }
        Ok(())
    }
    fn write_predicate_object_list<I>(
        &mut self,
        first_triple: &G::SPOTriple,
        iter: &mut Peekable<I>,
        namespaces: &Namespaces,
    ) -> Result<()>
    where
        I: Iterator<Item = G::SPOTriple>,
    {
        let subject = first_triple.subject();
        let mut triple = first_triple.clone();
        // loop until the subject changes
        loop {
            let predicate = triple.predicate();
            self.write_predicate(&predicate, namespaces)?;
            self.writer.write_all(b"\t")?;
            let mut same_subject = false;
            loop {
                self.write_object(triple.object(), namespaces)?;
                match iter.peek() {
                    None => {
                        break;
                    }
                    Some(next) => {
                        same_subject = next.subject() == subject;
                        if !same_subject || next.predicate() != predicate {
                            break;
                        }
                    }
                }
                self.writer.write_all(b" ,\n\t\t")?;
                triple = iter.next().unwrap();
            }
            if !same_subject {
                break;
            }
            self.writer.write_all(b" ;\n\t")?;
            triple = iter.next().unwrap();
        }
        Ok(())
    }
    fn write_statements(&mut self, namespaces: &Namespaces) -> Result<()> {
        let mut iter = self.graph.iter().peekable();
        while let Some(triple) = iter.next() {
            match triple.subject() {
                BlankNodeOrIRI::BlankNode(blank_node, _) => {
                    let mut object_iter = self.graph.iter_o(&blank_node.to_resource());
                    if object_iter.next().is_none() {
                        // node is never used as object, write anonymous node []
                        self.writer.write_all(b"[]")?;
                    } else if object_iter.next().is_none() {
                        // node is only used once, skip it now
                        // it will be written when it is an object
                        continue;
                    } else {
                        self.write_blank_node(blank_node)?;
                    }
                }
                BlankNodeOrIRI::IRI(ref iri) => self.write_iri(iri, namespaces)?,
            }
            self.writer.write_all(b"\t")?;
            self.write_predicate_object_list(&triple, &mut iter, namespaces)?;
            self.writer.write_all(b" .\n")?;
        }
        Ok(())
    }
}
