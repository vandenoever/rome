use std::io::{Result, Write};
use std::fmt::Display;
use graph::*;
use namespaces::*;
use constants;

struct TurtleWriter<'a, 'g, W: 'a, G: 'g>
    where W: Write,
          G: Graph<'g>
{
    buffer: Vec<u8>,
    base: String,
    writer: &'a mut W,
    xsd_string: Option<<<G as Graph<'g>>::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    last_subject: Option<BlankNodeOrIRI<'g, <G as Graph<'g>>::BlankNodePtr, <G as Graph<'g>>::IRIPtr>>,
    open_statement: bool,
}

pub fn write_turtle<'g, G: 'g, T: 'g, I, W>(namespaces: &Namespaces,
                                            triples: I,
                                            graph: &'g G,
                                            writer: &mut W)
                                            -> Result<()>
    where T: Triple<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
          G: Graph<'g>,
          <G as Graph<'g>>::BlankNodePtr: Display,
          I: Iterator<Item = T>,
          W: Write
{
    let mut writer = TurtleWriter::<_, G> {
        buffer: Vec::new(),
        base: String::new(),
        writer: writer,
        xsd_string: graph.find_datatype(constants::XSD_STRING),
        last_subject: None,
        open_statement: false,
    };
    for ns in namespaces.iter() {
        writer.write_prefix(&ns)?;
    }
    writer.writer.write_all(b"\n")?;
    for triple in triples {
        writer.write_triple(&triple, namespaces)?;
    }
    writer.writer.write_all(b" .\n")
}

impl<'a, 'g, W: 'a, G: 'g> TurtleWriter<'a, 'g, W, G>
    where W: Write,
          G: Graph<'g>,
          <G as Graph<'g>>::BlankNodePtr: Display
{
    fn write_prefix(&mut self, ns: &Namespace) -> Result<()> {
        self.writer.write_all(b"@prefix ")?;
        self.writer.write_all(ns.prefix())?;
        self.writer.write_all(b": ")?;
        self.write_full_iri(ns.namespace())?;
        self.writer.write_all(b" .\n")
    }
    fn write_iri(&mut self, iri: &str, namespaces: &Namespaces) -> Result<()> {
        if iri == constants::RDF_TYPE {
            self.writer.write_all(b"a")
        } else {
            match namespaces.find_prefix(iri) {
                Some((prefix, iri)) => self.write_prefixed_iri(prefix, iri),
                None => self.write_full_iri(iri),
            }
        }
    }
    fn write_prefixed_iri(&mut self, prefix: &[u8], iri: &str) -> Result<()> {
        self.writer.write_all(prefix)?;
        self.writer.write_all(b":")?;
        self.writer.write_all(iri.as_bytes())
    }
    fn write_full_iri(&mut self, mut iri: &str) -> Result<()> {
        if iri.starts_with(self.base.as_str()) {
            iri = &iri[self.base.len()..];
        }
        self.writer.write_all(b"<")?;
        self.buffer.clear();
        for b in iri.as_bytes() {
            if *b < 20 || b"<>\"{}|^`\\".contains(b) {
                write!(&mut self.buffer, "\\u00{:X} ", *b).unwrap();
            } else {
                self.buffer.push(*b);
            }
        }
        self.writer.write_all(&self.buffer[..])?;
        self.writer.write_all(b">")
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
                self.buffer.push('\\' as u8);
            }
            self.buffer.push(*b);
        }
        self.writer.write_all(&self.buffer[..])
    }
    fn write_literal(&mut self, literal: G::LiteralPtr, namespaces: &Namespaces) -> Result<()> {
        self.writer.write_all(b"\"")?;
        self.write_literal_value(literal.as_str())?;
        self.writer.write_all(b"\"")?;
        if let Some(ref langtag) = literal.language() {
            self.writer.write_all(b"@")?;
            self.writer.write_all(langtag.as_bytes())?;
        } else if Some(literal.datatype()) != self.xsd_string {
            self.writer.write_all(b"^^")?;
            self.write_iri(literal.datatype_str(), namespaces)?;
        }
        Ok(())
    }
    fn write_subject(&mut self,
                     subject: BlankNodeOrIRI<'g, G::BlankNodePtr, G::IRIPtr>,
                     namespaces: &Namespaces)
                     -> Result<()> {
        match subject {
            BlankNodeOrIRI::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            BlankNodeOrIRI::IRI(ref iri) => self.write_iri(iri.as_str(), namespaces),
        }
    }
    fn write_predicate(&mut self, predicate: &str, namespaces: &Namespaces) -> Result<()> {
        self.write_iri(predicate, namespaces)
    }
    fn write_object(&mut self,
                    object: Resource<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
                    namespaces: &Namespaces)
                    -> Result<()> {
        match object {
            Resource::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            Resource::IRI(iri) => self.write_iri(iri.as_str(), namespaces),
            Resource::Literal(literal) => self.write_literal(literal, namespaces),
        }
    }
    fn write_triple<T>(&mut self, triple: &T, namespaces: &Namespaces) -> Result<()>
        where T: Triple<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>
    {
        let subject = triple.subject();
        if self.last_subject == Some(subject.clone()) {
            self.writer.write_all(b" ;\n\t")?;
        } else {
            if self.open_statement {
                self.writer.write_all(b" .\n")?;
            }
            self.write_subject(triple.subject(), namespaces)?;
            self.last_subject = Some(subject);
            self.writer.write_all(b" ")?;
        }
        self.open_statement = true;
        self.write_predicate(triple.predicate().as_str(), namespaces)?;
        self.writer.write_all(b" ")?;
        self.write_object(triple.object(), namespaces)
    }
}
