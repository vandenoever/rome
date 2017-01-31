use constants;
use graph::*;
use std::fmt::Display;
use std::io::{Result, Write};
use std::marker::PhantomData;

struct NTriplesWriter<'a, 'g, W: 'a, G: 'g>
    where W: Write,
          G: Graph<'g>
{
    buffer: Vec<u8>,
    writer: &'a mut W,
    xsd_string: Option<<<G as Graph<'g>>::LiteralPtr as LiteralPtr<'g>>::DatatypePtr>,
    phantom: PhantomData<&'g u8>,
}

/// write an RDF 1.1 N-Triples file in canonical form
pub fn write_ntriples<'g, G: 'g, T: 'g, I, W>(triples: I,
                                              graph: &'g G,
                                              writer: &mut W)
                                              -> Result<()>
    where T: Triple<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>,
          G: Graph<'g>,
          <G as Graph<'g>>::BlankNodePtr: Display,
          I: Iterator<Item = T>,
          W: Write
{
    let mut writer = NTriplesWriter::<_, G> {
        buffer: Vec::new(),
        writer: writer,
        xsd_string: graph.find_datatype(constants::XSD_STRING),
        phantom: PhantomData,
    };
    for triple in triples {
        writer.write_ntriple(&triple)?;
    }
    Ok(())
}

impl<'a, 'g, W: 'a, G: 'g> NTriplesWriter<'a, 'g, W, G>
    where W: Write,
          G: Graph<'g>,
          <G as Graph<'g>>::BlankNodePtr: Display
{
    fn write_iri(&mut self, iri: &str) -> Result<()> {
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
                self.buffer.push(b'\\');
            }
            self.buffer.push(*b);
        }
        self.writer.write_all(&self.buffer[..])
    }
    fn write_literal(&mut self, literal: G::LiteralPtr) -> Result<()> {
        self.writer.write_all(b"\"")?;
        self.write_literal_value(literal.as_str())?;
        self.writer.write_all(b"\"")?;
        if let Some(langtag) = literal.language() {
            self.writer.write_all(b"@")?;
            self.writer.write_all(langtag.as_bytes())?;
        } else if Some(literal.datatype()) != self.xsd_string {
            self.writer.write_all(b"^^")?;
            self.write_iri(literal.datatype_str())?;
        }
        Ok(())
    }
    fn write_subject(&mut self,
                     subject: BlankNodeOrIRI<'g, G::BlankNodePtr, G::IRIPtr>)
                     -> Result<()> {
        match subject {
            BlankNodeOrIRI::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            BlankNodeOrIRI::IRI(iri) => self.write_iri(iri.as_str()),
        }
    }
    fn write_predicate(&mut self, predicate: &str) -> Result<()> {
        self.write_iri(predicate)
    }
    fn write_object(&mut self,
                    object: Resource<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>)
                    -> Result<()> {
        match object {
            Resource::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            Resource::IRI(iri) => self.write_iri(iri.as_str()),
            Resource::Literal(literal) => self.write_literal(literal),
        }
    }
    fn write_ntriple<T: 'g>(&mut self, triple: &T) -> Result<()>
        where T: Triple<'g, G::BlankNodePtr, G::IRIPtr, G::LiteralPtr>
    {
        self.write_subject(triple.subject())?;
        self.writer.write_all(b" ")?;
        self.write_predicate(triple.predicate().as_str())?;
        self.writer.write_all(b" ")?;
        self.write_object(triple.object())?;
        self.writer.write_all(b" .\n")
    }
}
