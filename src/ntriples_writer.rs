use std::io::{Result, Write};
use graph::*;

struct NTripleWriter<'a, W>
    where W: Write + 'a
{
    buffer: Vec<u8>,
    writer: &'a mut W,
}

/// write an RDF 1.1 N-Triples file in canonical form
pub fn write_ntriples<T, I, W>(triples: I, writer: &mut W) -> Result<()>
    where T: Triple,
          I: Iterator<Item = T>,
          W: Write
{
    let mut writer = NTripleWriter {
        buffer: Vec::new(),
        writer: writer,
    };
    for triple in triples {
        try!(writer.write_ntriple(&triple));
    }
    Ok(())
}

impl<'a, W> NTripleWriter<'a, W>
    where W: Write + 'a
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
    fn write_blank_node(&mut self, blank_node: &BlankNode) -> Result<()> {
        try!(self.writer.write_all(b"_:"));
        try!(write!(self.writer, "{}_{}", blank_node.1, blank_node.0));
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
    fn write_literal(&mut self, literal: &Literal) -> Result<()> {
        try!(self.writer.write_all(b"\""));
        try!(self.write_literal_value(&literal.lexical));
        try!(self.writer.write_all(b"\""));
        if let Some(ref langtag) = literal.language {
            try!(self.writer.write_all(b"@"));
            try!(self.writer.write_all(langtag.as_bytes()));
        } else if literal.datatype != "http://www.w3.org/2001/XMLSchema#string" {
            try!(self.writer.write_all(b"^^"));
            try!(self.write_iri(&literal.datatype));
        }
        Ok(())
    }

    fn write_subject(&mut self, subject: &Subject) -> Result<()> {
        match subject {
            &Subject::IRI(ref iri) => self.write_iri(&iri),
            &Subject::BlankNode(blank_node) => self.write_blank_node(&blank_node),
        }
    }
    fn write_predicate(&mut self, predicate: &str) -> Result<()> {
        self.write_iri(predicate)
    }
    fn write_object(&mut self, object: &Object) -> Result<()> {
        match object {
            &Object::IRI(ref iri) => self.write_iri(&iri),
            &Object::BlankNode(blank_node) => self.write_blank_node(&blank_node),
            &Object::Literal(ref literal) => self.write_literal(&literal),
        }
    }
    fn write_ntriple<T>(&mut self, triple: &T) -> Result<()>
        where T: Triple
    {
        try!(self.write_subject(&triple.subject()));
        try!(self.writer.write_all(b" "));
        try!(self.write_predicate(&triple.predicate()));
        try!(self.writer.write_all(b" "));
        try!(self.write_object(&triple.object()));
        self.writer.write_all(b" .\n")
    }
}
