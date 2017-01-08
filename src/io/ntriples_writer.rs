use std::io::{Result, Write};
use graph::*;

struct NTriplesWriter<'a, W: 'a>
    where W: Write
{
    buffer: Vec<u8>,
    writer: &'a mut W,
}

/// write an RDF 1.1 N-Triples file in canonical form
pub fn write_ntriples<'g, T: 'g, I, W, T1: 'g, T2: 'g, T3: 'g>(triples: I,
                                                               writer: &mut W)
                                                               -> Result<()>
    where T: Triple<'g, T1, T2, T3>,
          I: Iterator<Item = T>,
          W: Write,
          T1: BlankNodePtr<'g>,
          T2: IRIPtr<'g>,
          T3: LiteralPtr<'g>
{
    let mut writer = NTriplesWriter {
        buffer: Vec::new(),
        writer: writer,
    };
    for triple in triples {
        try!(writer.write_ntriple(&triple));
    }
    Ok(())
}

impl<'a, W: 'a> NTriplesWriter<'a, W>
    where W: Write
{
    fn write_iri(&mut self, iri: &str) -> Result<()> {
        try!(self.writer.write_all(b"<"));
        self.buffer.clear();
        for b in iri.as_bytes() {
            if *b < 20 || b"<>\"{}|^`\\".contains(b) {
                write!(&mut self.buffer, "\\u00{:X} ", *b).unwrap();
            } else {
                self.buffer.push(*b);
            }
        }
        try!(self.writer.write_all(&self.buffer[..]));
        self.writer.write_all(b">")
    }
    fn write_blank_node<'g, B>(&mut self, blank_node: B) -> Result<()>
        where B: BlankNodePtr<'g>
    {
        try!(self.writer.write_all(b"_:"));
        try!(write!(self.writer,
                    "{}_{}",
                    blank_node.graph_id(),
                    blank_node.node_id()));
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
    fn write_literal<'g, L>(&mut self, literal: L) -> Result<()>
        where L: LiteralPtr<'g>
    {
        try!(self.writer.write_all(b"\""));
        try!(self.write_literal_value(literal.as_str()));
        try!(self.writer.write_all(b"\""));
        if let Some(langtag) = literal.language() {
            try!(self.writer.write_all(b"@"));
            try!(self.writer.write_all(langtag.as_bytes()));
        } else if literal.datatype() != "http://www.w3.org/2001/XMLSchema#string" {
            try!(self.writer.write_all(b"^^"));
            try!(self.write_iri(literal.datatype()));
        }
        Ok(())
    }
    fn write_subject<'g, B, I>(&mut self, subject: BlankNodeOrIRI<'g, B, I>) -> Result<()>
        where B: BlankNodePtr<'g>,
              I: IRIPtr<'g>
    {
        match subject {
            BlankNodeOrIRI::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            BlankNodeOrIRI::IRI(iri) => self.write_iri(iri.as_str()),
        }
    }
    fn write_predicate<'b>(&mut self, predicate: &'b str) -> Result<()> {
        self.write_iri(predicate)
    }
    fn write_object<'g, B, I, L>(&mut self, object: Resource<'g, B, I, L>) -> Result<()>
        where B: BlankNodePtr<'g>,
              I: IRIPtr<'g>,
              L: LiteralPtr<'g>
    {
        match object {
            Resource::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            Resource::IRI(iri) => self.write_iri(iri.as_str()),
            Resource::Literal(literal) => self.write_literal(literal),
        }
    }
    fn write_ntriple<'g, T: 'g, B: 'g, I: 'g, L: 'g>(&mut self, triple: &T) -> Result<()>
        where T: Triple<'g, B, I, L>,
              B: BlankNodePtr<'g>,
              I: IRIPtr<'g>,
              L: LiteralPtr<'g>
    {
        try!(self.write_subject(triple.subject()));
        try!(self.writer.write_all(b" "));
        try!(self.write_predicate(triple.predicate().as_str()));
        try!(self.writer.write_all(b" "));
        try!(self.write_object(triple.object()));
        self.writer.write_all(b" .\n")
    }
}
