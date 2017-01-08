use std::io::{Result, Write};
use graph::*;
use namespaces::*;
use constants;

struct TurtleWriter<'a, 'g, W: 'a, B: 'g, I: 'g>
    where W: Write,
          B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    buffer: Vec<u8>,
    base: String,
    writer: &'a mut W,
    last_subject: Option<BlankNodeOrIRI<'g, B, I>>,
    open_statement: bool,
}

pub fn write_turtle<'g, T, I, W, T1: 'g, T2: 'g, T3: 'g>(namespaces: &Namespaces,
                                                         triples: I,
                                                         writer: &mut W)
                                                         -> Result<()>
    where T: Triple<'g, T1, T2, T3>,
          I: Iterator<Item = T>,
          W: Write,
          T1: BlankNodePtr<'g>,
          T2: IRIPtr<'g>,
          T3: LiteralPtr<'g>
{
    let mut writer = TurtleWriter {
        buffer: Vec::new(),
        base: String::new(),
        writer: writer,
        last_subject: None,
        open_statement: false,
    };
    for ns in namespaces.iter() {
        try!(writer.write_prefix(&ns));
    }
    try!(writer.writer.write_all(b"\n"));
    for triple in triples {
        try!(writer.write_triple(&triple, namespaces));
    }
    writer.writer.write_all(b" .\n")
}

impl<'a, 'g, W: 'a, B, I> TurtleWriter<'a, 'g, W, B, I>
    where W: Write,
          B: BlankNodePtr<'g>,
          I: IRIPtr<'g>
{
    fn write_prefix(&mut self, ns: &Namespace) -> Result<()> {
        try!(self.writer.write_all(b"@prefix "));
        try!(self.writer.write_all(ns.prefix()));
        try!(self.writer.write_all(b": "));
        try!(self.write_full_iri(ns.namespace()));
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
        try!(self.writer.write_all(prefix));
        try!(self.writer.write_all(b":"));
        self.writer.write_all(iri.as_bytes())
    }
    fn write_full_iri(&mut self, mut iri: &str) -> Result<()> {
        if iri.starts_with(self.base.as_str()) {
            iri = &iri[self.base.len()..];
        }
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
    fn write_blank_node(&mut self, blank_node: B) -> Result<()> {
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
    fn write_literal<L>(&mut self, literal: L, namespaces: &Namespaces) -> Result<()>
        where L: LiteralPtr<'g>
    {
        try!(self.writer.write_all(b"\""));
        try!(self.write_literal_value(literal.as_str()));
        try!(self.writer.write_all(b"\""));
        if let Some(ref langtag) = literal.language() {
            try!(self.writer.write_all(b"@"));
            try!(self.writer.write_all(langtag.as_bytes()));
        } else if literal.datatype() != "http://www.w3.org/2001/XMLSchema#string" {
            try!(self.writer.write_all(b"^^"));
            try!(self.write_iri(literal.datatype(), namespaces));
        }
        Ok(())
    }
    fn write_subject(&mut self,
                     subject: BlankNodeOrIRI<'g, B, I>,
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
    fn write_object<L>(&mut self,
                       object: Resource<'g, B, I, L>,
                       namespaces: &Namespaces)
                       -> Result<()>
        where L: LiteralPtr<'g>
    {
        match object {
            Resource::BlankNode(blank_node, _) => self.write_blank_node(blank_node),
            Resource::IRI(iri) => self.write_iri(iri.as_str(), namespaces),
            Resource::Literal(literal) => self.write_literal(literal, namespaces),
        }
    }
    fn write_triple<T, L: 'g>(&mut self, triple: &T, namespaces: &Namespaces) -> Result<()>
        where T: Triple<'g, B, I, L>,
              L: LiteralPtr<'g>
    {
        let subject = triple.subject();
        if self.last_subject == Some(subject.clone()) {
            try!(self.writer.write_all(b" ;\n\t"));
        } else {
            if self.open_statement {
                try!(self.writer.write_all(b" .\n"));
            }
            try!(self.write_subject(triple.subject(), namespaces));
            self.last_subject = Some(subject);
            try!(self.writer.write_all(b" "));
        }
        self.open_statement = true;
        try!(self.write_predicate(triple.predicate().as_str(), namespaces));
        try!(self.writer.write_all(b" "));
        self.write_object(triple.object(), namespaces)
    }
}
