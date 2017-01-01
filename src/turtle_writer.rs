use std::io::{Result, Write};
use graph::*;
use grammar;

pub struct Namespaces {
    namespaces: Vec<Namespace>,
}

impl Namespaces {
    pub fn new() -> Namespaces {
        Namespaces { namespaces: Vec::new() }
    }
    pub fn add(&mut self, namespace: &str, prefix: &[u8]) {
        self.namespaces.push(Namespace {
            namespace: String::from(namespace),
            prefix: Vec::from(prefix),
        });
    }
}

struct Namespace {
    namespace: String,
    prefix: Vec<u8>,
}

struct TurtleWriter<'a, W>
    where W: Write + 'a
{
    buffer: Vec<u8>,
    base: String,
    namespaces: &'a Vec<Namespace>,
    writer: &'a mut W,
    last_subject: SubjectClone,
    open_statement: bool,
}

pub fn write_turtle<T, I, W>(namespaces: &Namespaces, triples: I, writer: &mut W) -> Result<()>
    where T: Triple,
          I: Iterator<Item = T>,
          W: Write
{
    let mut writer = TurtleWriter {
        buffer: Vec::new(),
        base: String::new(),
        namespaces: &namespaces.namespaces,
        writer: writer,
        last_subject: SubjectClone::new(),
        open_statement: false,
    };
    for ns in writer.namespaces {
        try!(writer.write_prefix(ns));
    }
    try!(writer.writer.write_all(b"\n"));
    for triple in triples {
        try!(writer.write_triple(&triple));
    }
    writer.writer.write_all(b" .\n")
}

fn find_prefix<'a>(iri: &'a str, namespaces: &Vec<Namespace>) -> Option<(&'a str, usize)> {
    for i in 0..namespaces.len() {
        let ns = namespaces[i].namespace.as_str();
        if iri.starts_with(ns) {
            return Some((&iri[ns.len()..], i));
        }
    }
    None
}

impl<'a, W> TurtleWriter<'a, W>
    where W: Write + 'a
{
    fn write_prefix(&mut self, ns: &Namespace) -> Result<()> {
        try!(self.writer.write_all(b"@prefix "));
        try!(self.writer.write_all(ns.prefix.as_slice()));
        try!(self.writer.write_all(b": "));
        try!(self.write_full_iri(ns.namespace.as_str()));
        self.writer.write_all(b" .\n")
    }

    fn write_iri(&mut self, iri: &str) -> Result<()> {
        if iri == grammar::RDF_TYPE {
            self.writer.write_all(b"a")
        } else {
            match find_prefix(iri, &self.namespaces) {
                Some((iri, prefix)) => self.write_prefixed_iri(iri, prefix),
                None => self.write_full_iri(iri),
            }
        }
    }
    fn write_prefixed_iri(&mut self, iri: &str, prefix: usize) -> Result<()> {
        try!(self.writer.write_all(self.namespaces[prefix].prefix.as_slice()));
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
    fn write_triple<T>(&mut self, triple: &T) -> Result<()>
        where T: Triple
    {
        let subject = triple.subject();
        if self.last_subject.eq(&subject) {
            try!(self.writer.write_all(b" ;\n\t"));
        } else {
            if self.open_statement {
                try!(self.writer.write_all(b" .\n"));
            }
            try!(self.write_subject(&triple.subject()));
            self.last_subject.assign(&subject);
            try!(self.writer.write_all(b" "));
        }
        self.open_statement = true;
        try!(self.write_predicate(&triple.predicate()));
        try!(self.writer.write_all(b" "));
        self.write_object(&triple.object())
    }
}
