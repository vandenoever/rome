/// Generate rust code from a set of ontologies
///

extern crate rdfio;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use rdfio::graph_writer;
use rdfio::graph::{Object, Graph, GraphCreator, Triple, SubjectPtr, ObjectPtr};
use rdfio::triple_stream::*;
use rdfio::triple128::*;
use rdfio::namespaces::Namespaces;
use rdfio::resource::ResourceBase;
use rdfio::ontology::classes::rdf::Property;
use rdfio::ontology::classes::rdfs::Class;
use rdfio::ontology::properties::rdfs::{Comment, Domain, Range, SubClassOf};
use rdfio::ontology;
use rdfio::ontology_adapter;

type MyGraph = graph_writer::Graph<Triple128SPO, Triple128OPS>;
type OA = ontology_adapter::OntologyAdapter<MyGraph>;

type Writers = BTreeMap<Vec<u8>, Vec<u8>>;

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
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn camel_case(str: &str) -> String {
    let mut s = str[0..1].to_uppercase();
    s.push_str(&str[1..]);
    s.replace("-", "_")
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
    s.replace("-", "_")
}

fn comment_escape(str: &str) -> String {
    str.replace("\n", "")
}

fn write_impl_property<G, W>(class: &Class<G>,
                             property: &Property<G>,
                             mod_name: &str,
                             prefixes: &Namespaces,
                             writer: &mut W,
                             mod_uses: &mut BTreeSet<Vec<u8>>)
                             -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    if let Some(iri) = property.this().iri() {
        if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
            if let Some(domain) = class.this().iri() {
                if let Some((prefix, domain)) = prefixes.find_prefix(domain) {
                    writer.write_all(
                        format!("impl<G> {}::properties::{}::{}<G> for {}<G> where G: graph::Graph {{}}\n",
                            mod_name,
                            String::from_utf8_lossy(prop_prefix),
                            camel_case(prop),
                            camel_case(domain)).as_bytes())?;
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
                               mod_name: &str,
                               prefixes: &Namespaces,
                               writer: &mut W,
                               mod_uses: &mut BTreeSet<Vec<u8>>)
                               -> rdfio::Result<()>
    where W: Write,
          G: Graph
{
    for property in properties {
        for domain in property.domain() {
            if domain == *parent {
                write_impl_property(class, property, mod_name, prefixes, writer, mod_uses)?;
            }
        }
    }
    for parent in parent.sub_class_of() {
        write_impl_properties(class, &parent, properties, mod_name, prefixes, writer, mod_uses)?;
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

fn write_mod(output_dir: &Path, internal: bool, iris: &Vec<String>) -> rdfio::Result<()> {
    let path = output_dir.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    mod_rs.write_all(b"pub mod classes;\n")?;
    mod_rs.write_all(b"pub mod properties;\n")?;
    mod_rs.write_all(b"use std;\n")?;
    if internal {
        mod_rs.write_all(b"use graph;\n")?;
        mod_rs.write_all(b"use ontology_adapter;\n")?;
    } else {
        mod_rs.write_all(b"use rdfio::graph;\n")?;
        mod_rs.write_all(b"use rdfio::ontology_adapter;\n")?;
    }
    mod_rs.write_all(b"pub fn adapter<G>(graph: &std::rc::Rc<G>) -> ontology_adapter::OntologyAdapter<G>
    where G: graph::Graph
{
    let mut iris = Vec::with_capacity(")?;
    mod_rs.write_all(format!("{});\n", iris.len()).as_bytes())?;
    for iri in iris {
        mod_rs.write_all(format!("    iris.push(graph.predicate_ptr(\"{}\"));\n", iri).as_bytes())?;
    }
    mod_rs.write_all(b"    ontology_adapter::OntologyAdapter::new(graph, iris)\n}\n")?;
    Ok(())
}

fn load_files(inputs: &Vec<String>) -> rdfio::Result<(Namespaces, Rc<MyGraph>)> {
    let mut writer = graph_writer::GraphWriter::with_capacity(65000);
    let mut prefixes = Namespaces::new();
    for input in inputs {
        let data = read_file(input)?;
        let mut base = String::from("file:");
        base.push_str(input);
        let mut triples = TripleIterator::new(data.as_str(), &base)?;
        while let Some(triple) = triples.next() {
            writer.add_triple(&triple?);
        }
        for ns in triples.prefixes().iter() {
            prefixes.set(ns.prefix(), ns.namespace());
        }
    }
    let graph = writer.collect();
    let graph = infer(&graph)?;
    Ok((prefixes, Rc::new(graph)))
}

fn generate_classes<G>(classes: &Vec<Class<G>>,
                       properties: &Vec<Property<G>>,
                       output_dir: &Path,
                       mod_name: &str,
                       internal: bool,
                       prefixes: &Namespaces,
                       iris: &mut Vec<String>)
                       -> rdfio::Result<()>
    where G: Graph
{

    let mut outputs = BTreeMap::new();
    let mut mod_uses = BTreeMap::new();
    for ns in prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
        mod_uses.insert(Vec::from(ns.prefix()), BTreeSet::new());
    }

    for class in classes {
        if let Some(iri) = class.this().iri() {
            if let Some((prefix, name)) = prefixes.find_prefix(iri) {
                if let Some(mut writer) = outputs.get_mut(prefix) {
                    writer.write_all(b"\n/// ")?;
                    writer.write_all(prefix)?;
                    writer.write_all(b":")?;
                    writer.write_all(name.as_bytes())?;
                    for comment in class.comment() {
                        if let Some(l) = comment.this().literal() {
                            writer.write_all(b"\n/// ")?;
                            writer.write_all(comment_escape(l).as_bytes())?;
                        }
                    }
                    writer.write_all(format!("\nclass!(\"{}\", {}, {});\n", iri,
                            camel_case(name), iris.len())
                            .as_bytes())?;
                    write_impl_properties(&class,
                                          &class,
                                          properties,
                                          mod_name,
                                          prefixes,
                                          &mut writer,
                                          &mut mod_uses.get_mut(prefix).unwrap())?;
                    iris.push(String::from(iri));
                }
            }
        }
    }
    write_classes(output_dir, mod_name, internal, &outputs)?;
    Ok(())
}

fn generate_properties<G>(properties: &Vec<Property<G>>,
                          output_dir: &Path,
                          mod_name: &str,
                          internal: bool,
                          prefixes: &Namespaces,
                          iris: &mut Vec<String>)
                          -> rdfio::Result<()>
    where G: Graph
{

    let mut outputs = BTreeMap::new();
    let mut mod_uses = BTreeMap::new();
    for ns in prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
        mod_uses.insert(Vec::from(ns.prefix()), BTreeSet::new());
    }

    for property in properties {
        if let Some(iri) = property.this().iri() {
            if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
                for range in property.range() {
                    if let Some(range) = range.this().iri() {
                        if let Some((prefix, range)) = prefixes.find_prefix(range) {
                            if let Some(mut writer) = outputs.get_mut(prop_prefix) {
                                writer.write_all(b"\n/// ")?;
                                writer.write_all(prop_prefix)?;
                                writer.write_all(b":")?;
                                writer.write_all(prop.as_bytes())?;
                                for comment in property.comment() {
                                    if let Some(l) = comment.this().literal() {
                                        writer.write_all(b"\n/// ")?;
                                        writer.write_all(comment_escape(l).as_bytes())?;
                                    }
                                }
                                writer.write_all(
                                    format!("\nproperty!(\"{}\", {}, {}, {}::classes::{}::{}<G>, {});\n",
                                        iri, camel_case(prop),
                                        snake_case(prop), mod_name,
                                        String::from_utf8_lossy(prefix),
                                        camel_case(range), iris.len()).as_bytes())?;
                                mod_uses.get_mut(prop_prefix).unwrap().insert(Vec::from(prefix));
                            }
                        }
                    }
                }
                iris.push(String::from(iri));
            }
        }
    }
    write_properties(output_dir, mod_name, internal, &outputs)?;
    Ok(())
}

fn uses(mod_name: &str, internal: bool, classes: bool) -> String {
    let mut uses = String::new();
    uses.push_str("use std;\n");
    if internal {
        uses.push_str("use graph;\n");
        uses.push_str("use resource;\n");
        if classes {
            uses.push_str("use ontology_adapter;\n");
        }
    } else {
        uses.push_str("use rdfio::graph;\n");
        uses.push_str("use rdfio::resource;\n");
        if classes {
            uses.push_str("use rdfio::ontology_adapter;\n");
        }
    }
    uses.push_str(&format!("use {};\n", mod_name));
    uses
}

fn write_classes(output_dir: &Path,
                 mod_name: &str,
                 internal: bool,
                 writers: &Writers)
                 -> rdfio::Result<()> {
    write_files(output_dir,
                writers,
                "classes",
                &uses(mod_name, internal, true))
}

fn write_properties(output_dir: &Path,
                    mod_name: &str,
                    internal: bool,
                    writers: &Writers)
                    -> rdfio::Result<()> {
    write_files(output_dir,
                writers,
                "properties",
                &uses(mod_name, internal, false))
}

fn write_files(output_dir: &Path,
               writers: &Writers,
               folder: &str,
               uses: &str)
               -> rdfio::Result<()> {
    let dir_path = output_dir.join(folder);
    if !fs::metadata(&dir_path)?.is_dir() {
        fs::create_dir(output_dir.join(folder))?;
    }
    let path = dir_path.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    for (prefix, content) in writers.iter() {
        if content.len() > 0 {
            let mut filename = String::from_utf8_lossy(prefix).into_owned();
            filename.push_str(".rs");
            let path = dir_path.join(filename);
            let mut file = fs::File::create(path)?;
            file.write_all(uses.as_bytes())?;
            file.write_all(content)?;
            mod_rs.write_all(b"pub mod ")?;
            mod_rs.write_all(prefix)?;
            mod_rs.write_all(b";\n")?;
        }
    }
    Ok(())
}

fn generate(output_dir: &Path,
            mod_name: &str,
            internal: bool,
            inputs: &Vec<String>)
            -> rdfio::Result<()> {
    let (prefixes, graph) = load_files(inputs)?;
    let oa = Rc::new(ontology::adapter(&graph));
    let classes = Class::iter(&oa).collect();
    let properties = Property::iter(&oa).collect();
    let mut iris = Vec::new();
    iris.push(String::from(RDF_TYPE));
    generate_classes(&classes,
                     &properties,
                     output_dir,
                     mod_name,
                     internal,
                     &prefixes,
                     &mut iris)?;
    generate_properties(&properties,
                        output_dir,
                        mod_name,
                        internal,
                        &prefixes,
                        &mut iris)?;
    write_mod(output_dir, internal, &iris)?;
    Ok(())
}

fn main() {
    let mut args = args();
    let exe = args.next().unwrap();
    if args.len() < 2 {
        println_stderr!("Usage: {} [--mod MOD] OUTPUT_DIR INPUT_FILES", exe);
        std::process::exit(-1);
    }
    let arg = args.next().unwrap();
    let internal; // is the command run for rdfio itself?
    let mod_name;
    let output_dir;
    if args.len() > 2 && arg == "--mod" {
        mod_name = args.next().unwrap();
        internal = false;
        output_dir = args.next().unwrap();
    } else {
        mod_name = String::from("ontology");
        internal = true;
        output_dir = arg;
    }
    let inputs = args.collect();
    if let Err(e) = generate(Path::new(&output_dir), &mod_name, internal, &inputs) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
