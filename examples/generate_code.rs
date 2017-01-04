/// Generate rust code from an ontology
///

extern crate rdfio;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::collections::HashMap;
use rdfio::graph_writer;
use rdfio::graph::{Object, Graph, GraphCreator, Subject, Triple};
use rdfio::triple_stream::*;
use rdfio::triple64::*;
use rdfio::namespaces::Namespaces;
use rdfio::resource::{ResourceBase, Resource};
use rdfio::ontology::rdf::Property;
use rdfio::ontology::rdfs::{Class, Comment, Domain, Range, SubClassOf};

type MyGraph = graph_writer::Graph<Triple64SPO, Triple64OPS>;

type Writers = HashMap<Vec<u8>, fs::File>;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn read_file(path: &str) -> io::Result<String> {
    let mut f = match fs::File::open(path) {
        Err(e) => {
            println_stderr!("Cannot open file {}.", path);
            return Err(e);
        }
        Ok(f) => f,
    };
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn camel_case(str: &str) -> String {
    let mut s = str[0..1].to_uppercase();
    s.push_str(&str[1..]);
    s
}

fn snake_case(str: &str) -> String {
    if str == "type" {
        return String::from("a");
    }
    let mut s = str[0..1].to_lowercase();
    for c in str.chars().skip(1) {
        if c.is_uppercase() {
            s.push('_');
            for l in c.to_lowercase() {
                s.push(l);
            }
        } else {
            s.push(c);
        }
    }
    s
}

fn comment_escape(str: &str) -> String {
    str.replace("\n", "")
}


fn write_impl_property<G, W>(class: &Class<G>,
                             property: &Property<G>,
                             prefixes: &Namespaces,
                             writer: &mut W)
                             -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    if let &Resource::IRI(ref iri) = property.this() {
        if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri.as_str()) {
            if let &Resource::IRI(ref domain) = class.this() {
                if let Some((prefix, domain)) = prefixes.find_prefix(domain.as_str()) {
                    try!(writer.write_all(b"impl<G> "));
                    try!(writer.write_all(prop_prefix));
                    try!(writer.write_all(b"::"));
                    try!(writer.write_all(camel_case(prop).as_bytes()));
                    try!(writer.write_all(b"<G> for "));
                    try!(writer.write_all(prefix));
                    try!(writer.write_all(b"::"));
                    try!(writer.write_all(camel_case(domain).as_bytes()));
                    try!(writer.write_all(b"<G> where G: graph::Graph {}\n"));
                }
            }
        }
    }
    Ok(())
}

fn write_impl_properties<G, W>(class: &Class<G>,
                               parent: &Class<G>,
                               properties: &Vec<Property<G>>,
                               prefixes: &Namespaces,
                               writer: &mut W)
                               -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    for property in properties {
        for domain in property.domain() {
            if domain == *parent {
                try!(write_impl_property(class, property, prefixes, writer));
            }
        }
    }
    for parent in parent.sub_class_of() {
        try!(write_impl_properties(class, &parent, properties, prefixes, writer));
    }
    Ok(())
}
fn write_code<G>(classes: &Vec<Class<G>>,
                 properties: &Vec<Property<G>>,
                 prefixes: &Namespaces,
                 writers: &Writers)
                 -> rdfio::Result<()>
    where G: Graph
{
    for class in classes {
        if let &Resource::IRI(ref iri) = class.this() {
            if let Some((prefix, name)) = prefixes.find_prefix(iri.as_str()) {
                if let Some(mut writer) = writers.get(prefix) {
                    try!(writer.write_all(b"\n/// "));
                    try!(writer.write_all(prefix));
                    try!(writer.write_all(b":"));
                    try!(writer.write_all(name.as_bytes()));
                    for comment in class.comment() {
                        if let &Resource::Literal(ref l) = comment.this() {
                            try!(writer.write_all(b"\n/// "));
                            try!(writer.write_all(comment_escape(l).as_bytes()));
                        }
                    }
                    try!(writer.write_all(b"\nclass!(\""));
                    try!(writer.write_all(iri.as_bytes()));
                    try!(writer.write_all(b"\", "));
                    try!(writer.write_all(camel_case(name).as_bytes()));
                    try!(writer.write_all(b");\n"));
                    try!(write_impl_properties(class, class, properties, prefixes, &mut writer));
                }
            }
        }
    }
    for property in properties {
        if let &Resource::IRI(ref iri) = property.this() {
            if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri.as_str()) {
                for range in property.range() {
                    if let &Resource::IRI(ref range) = range.this() {
                        if let Some((prefix, range)) = prefixes.find_prefix(range.as_str()) {

                            if let Some(mut writer) = writers.get(prop_prefix) {
                                try!(writer.write_all(b"\n/// "));
                                try!(writer.write_all(prop_prefix));
                                try!(writer.write_all(b":"));
                                try!(writer.write_all(prop.as_bytes()));
                                for comment in property.comment() {
                                    if let &Resource::Literal(ref l) = comment.this() {
                                        try!(writer.write_all(b"\n/// "));
                                        try!(writer.write_all(comment_escape(l).as_bytes()));
                                    }
                                }
                                try!(writer.write_all(b"\nproperty!(\""));
                                try!(writer.write_all(iri.as_bytes()));
                                try!(writer.write_all(b"\", "));
                                try!(writer.write_all(camel_case(prop).as_bytes()));
                                try!(writer.write_all(b", "));
                                try!(writer.write_all(snake_case(prop).as_bytes()));
                                try!(writer.write_all(b", "));
                                try!(writer.write_all(prefix));
                                try!(writer.write_all(b"::"));
                                try!(writer.write_all(camel_case(range).as_bytes()));
                                try!(writer.write_all(b"<G>);\n"));
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDF_PROPERTY: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn infer(graph: &MyGraph) -> rdfio::Result<MyGraph> {
    // for every triple with rdfs:subClassOf infer that the subject and the
    // object are rdfs:Class instances
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    for triple in graph.iter().filter(|triple| triple.predicate() == RDFS_SUB_CLASS_OF) {
        writer.add(triple.subject(), RDF_TYPE, RDFS_CLASS);
        match triple.object() {
            Object::IRI(iri) => {
                writer.add(iri, RDF_TYPE, RDFS_CLASS);
            }
            Object::BlankNode(b) => {
                writer.add(b, RDF_TYPE, RDFS_CLASS);
            }
            _ => {}
        }
    }
    for triple in graph.iter() {
        writer.add_triple(&triple);
    }
    Ok(writer.collect().sort_blank_nodes())
}

fn get_classes(graph: &Rc<MyGraph>) -> rdfio::Result<Vec<Class<MyGraph>>> {
    let mut classes = Vec::new();
    for t in graph.iter_object_iri_predicate(RDFS_CLASS, RDF_TYPE) {
        if let Subject::IRI(iri) = t.subject() {
            classes.push(Class::new(&Resource::IRI(Rc::new(String::from(iri))), &graph));
        }
    }
    Ok(classes)
}

fn get_properties(graph: &Rc<MyGraph>) -> rdfio::Result<Vec<Property<MyGraph>>> {
    let mut properties = Vec::new();
    for t in graph.iter_object_iri_predicate(RDF_PROPERTY, RDF_TYPE) {
        if let Subject::IRI(iri) = t.subject() {
            properties.push(Property::new(&Resource::IRI(Rc::new(String::from(iri))), &graph));
        }
    }
    Ok(properties)
}

fn open_writers(output_dir: &Path, prefixes: &Namespaces) -> rdfio::Result<Writers> {
    let mut writers = HashMap::new();
    for ns in prefixes.iter() {
        let mut filename = String::from_utf8_lossy(ns.prefix()).into_owned();
        filename.push_str(".rs");
        let path = output_dir.join(filename);
        let mut file = try!(fs::File::create(path));
        try!(file.write_all(b"use std;\nuse graph;\nuse resource;\nuse ontology::rdf;\nuse ontology::rdfs;\n"));
        writers.insert(Vec::from(ns.prefix()), file);
    }
    Ok(writers)
}

fn generate(output_dir: &Path, inputs: &Vec<String>) -> rdfio::Result<()> {
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    let mut prefixes = Namespaces::new();
    for input in inputs {
        let data = try!(read_file(input));
        let mut base = String::from("file:");
        base.push_str(input);
        let mut triples = try!(TripleIterator::new(data.as_str(), &base));
        while let Some(triple) = triples.next() {
            writer.add_triple(&try!(triple));
        }
        for ns in triples.prefixes().iter() {
            prefixes.set(ns.prefix(), ns.namespace());
        }
    }
    let graph: MyGraph = writer.collect().sort_blank_nodes();
    let graph = try!(infer(&graph));
    let graph = Rc::new(graph);
    let classes = try!(get_classes(&graph));
    let properties = try!(get_properties(&graph));
    let writers = try!(open_writers(output_dir, &prefixes));
    try!(write_code(&classes, &properties, &prefixes, &writers));
    Ok(())
}

fn main() {
    let mut args = args();
    let exe = args.next().unwrap();
    if args.len() < 2 {
        println_stderr!("Usage: {} OUTPUT_DIR INPUT_FILES", exe);
        std::process::exit(-1);
    }
    let output_dir = args.next().unwrap();
    println!("output_dir {}", output_dir);
    let inputs = args.collect();
    if let Err(e) = generate(Path::new(&output_dir), &inputs) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
