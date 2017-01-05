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
use std::collections::HashSet;
use rdfio::graph_writer;
use rdfio::graph::{Object, Graph, GraphCreator, Triple, SubjectPtr, ObjectPtr};
use rdfio::triple_stream::*;
use rdfio::triple64::*;
use rdfio::namespaces::Namespaces;
use rdfio::resource::ResourceBase;
use rdfio::ontology::rdf::Property;
use rdfio::ontology::rdfs::{Class, Comment, Domain, Range, SubClassOf};
use rdfio::ontology;
use rdfio::ontology_adapter;

type MyGraph = graph_writer::Graph<Triple64SPO, Triple64OPS>;
type OA = ontology_adapter::OntologyAdapter<MyGraph>;

type Writers = HashMap<Vec<u8>, Vec<u8>>;

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
                             writer: &mut W,
                             mod_uses: &mut HashSet<Vec<u8>>)
                             -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    if let Some(iri) = property.this().iri() {
        if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
            if let Some(domain) = class.this().iri() {
                if let Some((prefix, domain)) = prefixes.find_prefix(domain) {
                    try!(writer.write_all(b"impl<G> "));
                    try!(writer.write_all(prop_prefix));
                    try!(writer.write_all(b"::"));
                    try!(writer.write_all(camel_case(prop).as_bytes()));
                    try!(writer.write_all(b"<G> for "));
                    try!(writer.write_all(prefix));
                    try!(writer.write_all(b"::"));
                    try!(writer.write_all(camel_case(domain).as_bytes()));
                    try!(writer.write_all(b"<G> where G: graph::Graph {}\n"));
                    mod_uses.insert(Vec::from(prop_prefix));
                    mod_uses.insert(Vec::from(prefix));
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
                               writer: &mut W,
                               mod_uses: &mut HashSet<Vec<u8>>)
                               -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    for property in properties {
        for domain in property.domain() {
            if domain == *parent {
                try!(write_impl_property(class, property, prefixes, writer, mod_uses));
            }
        }
    }
    for parent in parent.sub_class_of() {
        try!(write_impl_properties(class, &parent, properties, prefixes, writer, mod_uses));
    }
    Ok(())
}
fn generate_code<G>(classes: &Vec<Class<G>>,
                    properties: &Vec<Property<G>>,
                    prefixes: &Namespaces,
                    writers: &mut Writers,
                    iris: &mut Vec<String>,
                    mod_uses: &mut HashMap<Vec<u8>, HashSet<Vec<u8>>>)
                    -> rdfio::Result<()>
    where G: Graph
{
    for class in classes {
        if let Some(iri) = class.this().iri() {
            if let Some((prefix, name)) = prefixes.find_prefix(iri) {
                if let Some(mut writer) = writers.get_mut(prefix) {
                    try!(writer.write_all(b"\n/// "));
                    try!(writer.write_all(prefix));
                    try!(writer.write_all(b":"));
                    try!(writer.write_all(name.as_bytes()));
                    for comment in class.comment() {
                        if let Some(l) = comment.this().literal() {
                            try!(writer.write_all(b"\n/// "));
                            try!(writer.write_all(comment_escape(l).as_bytes()));
                        }
                    }
                    try!(writer.write_all(b"\nclass!(\""));
                    try!(writer.write_all(iri.as_bytes()));
                    try!(writer.write_all(b"\", "));
                    try!(writer.write_all(camel_case(name).as_bytes()));
                    try!(writer.write_all(format!(", {});\n", iris.len()).as_bytes()));
                    try!(write_impl_properties(class, class, properties, prefixes,
                            &mut writer, &mut mod_uses.get_mut(prefix).unwrap()));
                    iris.push(String::from(iri));
                }
            }
        }
    }
    for property in properties {
        if let Some(iri) = property.this().iri() {
            if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
                for range in property.range() {
                    if let Some(range) = range.this().iri() {
                        if let Some((prefix, range)) = prefixes.find_prefix(range) {

                            if let Some(mut writer) = writers.get_mut(prop_prefix) {
                                try!(writer.write_all(b"\n/// "));
                                try!(writer.write_all(prop_prefix));
                                try!(writer.write_all(b":"));
                                try!(writer.write_all(prop.as_bytes()));
                                for comment in property.comment() {
                                    if let Some(l) = comment.this().literal() {
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
                                try!(writer.write_all(format!("<G>, {});\n", iris.len()).as_bytes()));
                                mod_uses.get_mut(prop_prefix).unwrap().insert(Vec::from(prefix));
                            }
                        }
                    }
                }
                iris.push(String::from(iri));
            }
        }
    }
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
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

fn get_classes(oa: &Rc<OA>) -> rdfio::Result<Vec<Class<MyGraph>>> {
    let mut classes = Vec::new();
    for c in Class::iter(oa) {
        classes.push(c);
    }
    Ok(classes)
}

fn get_properties(oa: &Rc<OA>) -> rdfio::Result<Vec<Property<MyGraph>>> {
    let mut properties = Vec::new();
    for p in Property::iter(oa) {
        properties.push(p);
    }
    Ok(properties)
}

fn write_code(output_dir: &Path,
              writers: &Writers,
              iris: &Vec<String>,
              mod_uses: &HashMap<Vec<u8>, HashSet<Vec<u8>>>)
              -> rdfio::Result<()> {
    let path = output_dir.join("mod.rs");
    let mut mod_rs = try!(fs::File::create(path));
    for (prefix, content) in writers.iter() {
        if content.len() > 0 {
            let mut filename = String::from_utf8_lossy(prefix).into_owned();
            filename.push_str(".rs");
            let path = output_dir.join(filename);
            let mut file = try!(fs::File::create(path));
            try!(file.write_all(
                b"use std;\nuse graph;\nuse resource;\nuse ontology_adapter;\n"));
            for u in mod_uses[prefix].iter() {
                try!(file.write_all(b"use ontology::"));
                try!(file.write_all(&u));
                try!(file.write_all(b";\n"));
            }
            try!(file.write_all(content));
            try!(mod_rs.write_all(b"pub mod "));
            try!(mod_rs.write_all(prefix));
            try!(mod_rs.write_all(b";\n"));
        }
    }
    try!(mod_rs.write_all(b"use graph;
use std;
use ontology_adapter;

pub fn adapter<G>(graph: &std::rc::Rc<G>) -> ontology_adapter::OntologyAdapter<G>
    where G: graph::Graph
{
    let mut iris = Vec::with_capacity("));
    try!(mod_rs.write_all(format!("{});\n", iris.len()).as_bytes()));
    for iri in iris {
        try!(mod_rs.write_all(format!("    iris.push(graph.predicate_ptr(\"{}\"));\n", iri).as_bytes()));
    }
    try!(mod_rs.write_all(b"    ontology_adapter::OntologyAdapter::new(graph, iris)\n}\n"));
    Ok(())
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
    let oa = Rc::new(ontology::adapter(&graph));
    let classes = try!(get_classes(&oa));
    let properties = try!(get_properties(&oa));

    let mut outputs = HashMap::new();
    let mut mod_uses = HashMap::new();
    for ns in prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
        mod_uses.insert(Vec::from(ns.prefix()), HashSet::new());
    }
    let mut iris = Vec::new();
    iris.push(String::from(RDF_TYPE));
    try!(generate_code(&classes, &properties, &prefixes, &mut outputs, &mut iris, &mut mod_uses));
    try!(write_code(output_dir, &outputs, &iris, &mod_uses));
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
