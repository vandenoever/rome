/// Generate rust code from a set of ontologies
///

extern crate rome;
use rome::graph::{Graph, GraphWriter, Triple, ResourceTranslator, IRIPtr, LiteralPtr,
                  WriterResource};
use rome::graphs::tel;
use rome::io::TurtleParser;
use rome::iter::TransitiveIterator;
use rome::namespaces::Namespaces;
use rome::ontology;
use rome::ontology::classes::rdf::Property;
use rome::ontology::classes::rdfs::Class;
use rome::ontology::properties::rdfs::{Comment, Domain, Range, SubClassOf};
use rome::ontology_adapter;
use rome::resource::{ResourceBase, IRI, ObjectIter};
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;

type MyGraph = tel::Graph128;
type Writers = BTreeMap<Vec<u8>, Vec<u8>>;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

struct Output {
    mod_name: String,
    output_dir: PathBuf,
    internal: bool,
}

struct OntoData<'g> {
    o: Output,
    classes: Vec<IRI<'g, Class<'g, MyGraph>>>,
    properties: Vec<IRI<'g, Property<'g, MyGraph>>>,
    prefixes: Namespaces,
}

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

fn write_impl_property<'g, G, W>(class: &IRI<'g, Class<'g, G>>,
                                 property: &IRI<'g, Property<'g, G>>,
                                 mod_name: &str,
                                 prefixes: &Namespaces,
                                 done: &mut HashSet<String>,
                                 writer: &mut W)
                                 -> rome::Result<()>
    where W: Write,
          G: Graph<'g>
{
    let iri = property.as_str();
    if done.contains(iri) {
        return Ok(());
    }
    if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
        if let Some((_, domain)) = prefixes.find_prefix(class.as_str()) {
            writer.write_all(format!("impl<'g, G: 'g> {}::properties::{}::{}<'g> for {}<'g, G> \
                                    where G: graph::Graph<'g> {{}}\n",
                                   mod_name,
                                   String::from_utf8_lossy(prop_prefix),
                                   camel_case(prop),
                                   camel_case(domain))
                    .as_bytes())?;
            writer.write_all(format!("impl<'g, G: 'g> {}::properties::{}::{}<'g> for \
                                    resource::IRI<'g, {}<'g, G>> where G: graph::Graph<'g> \
                                    {{}}\n",
                                   mod_name,
                                   String::from_utf8_lossy(prop_prefix),
                                   camel_case(prop),
                                   camel_case(domain))
                    .as_bytes())?;
            done.insert(String::from(iri));
        }
    }
    Ok(())
}

fn write_impl_properties<'g, W>(class: &IRI<'g, Class<'g, MyGraph>>,
                                parent: &Class<'g, MyGraph>,
                                d: &OntoData<'g>,
                                done: &mut HashSet<String>,
                                writer: &mut W)
                                -> rome::Result<()>
    where W: Write
{
    for property in &d.properties {
        for domain in property.domain() {
            if domain == *parent {
                write_impl_property(class, property, &d.o.mod_name, &d.prefixes, done, writer)?;
            }
        }
    }
    for parent in parent.sub_class_of() {
        write_impl_properties(class, &parent, d, done, writer)?;
    }
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn closure<'g>(class: &Class<'g, MyGraph>) -> ObjectIter<'g, Class<'g, MyGraph>> {
    class.sub_class_of()
}

struct Translator<'g> {
    blank_nodes: BTreeMap<<MyGraph as Graph<'g>>::BlankNodePtr,
                          <tel::GraphCreator<tel::Triple128SPO,
                           tel::Triple128OPS> as GraphWriter<'g>>::BlankNode>
}

impl<'g> Translator<'g> {
    fn new() -> Translator<'g> {
        Translator { blank_nodes: BTreeMap::new() }
    }
}

impl<'g> ResourceTranslator<'g> for Translator<'g> {
    type Graph = MyGraph;
    type GraphWriter = tel::GraphCreator<tel::Triple128SPO, tel::Triple128OPS>;
    fn translate_blank_node(&mut self,
                            w: &mut Self::GraphWriter,
                            blank_node: &<Self::Graph as Graph<'g>>::BlankNodePtr
        ) -> <Self::GraphWriter as GraphWriter<'g>>::BlankNode {
        if let Some(blank_node) = self.blank_nodes.get(blank_node) {
            return *blank_node;
        }
        let new_blank_node = w.create_blank_node();
        self.blank_nodes.insert(*blank_node, new_blank_node);
        new_blank_node
    }
}

fn infer(graph: &MyGraph) -> rome::Result<MyGraph> {
    // for every triple with rdfs:subClassOf infer that the subject and the
    // object are rdfs:Class instances
    let oa: ontology_adapter::OntologyAdapter<MyGraph> = ontology::adapter(graph);
    let mut w = tel::GraphCreator::with_capacity(65000);
    let mut translator = Translator::new();
    let rdf_type = w.create_iri(&RDF_TYPE);
    let rdfs_class = WriterResource::IRI(w.create_iri(&RDFS_CLASS));
    let rdfs_sub_class_of = graph.find_iri(RDFS_SUB_CLASS_OF).unwrap();
    for triple in graph.iter()
        .filter(|triple| !triple.object().is_literal() && triple.predicate() == rdfs_sub_class_of) {
        let class = translator.translate_blank_node_or_iri(&mut w, &triple.subject());
        w.add(&class, &rdf_type, &rdfs_class);
        let class = translator.translate_blank_node_or_iri(&mut w,
                                                           &triple.object()
                                                               .to_blank_node_or_iri()
                                                               .unwrap());
        w.add(&class, &rdf_type, &rdfs_class);
    }
    // copy all triples
    for triple in graph.iter() {
        let subject = translator.translate_blank_node_or_iri(&mut w, &triple.subject());
        let predicate = w.create_iri(&triple.predicate());
        let object = translator.translate_resource(&mut w, &triple.object());
        w.add(&subject, &predicate, &object);
    }
    {
        // make subClassOf entailment concrete
        let rdfs_sub_class_of = w.create_iri(&RDFS_SUB_CLASS_OF);
        for class in Class::iter(&oa) {
            // let i = TransitiveIterator::new(class.sub_class_of(),
            //                                |class: &Class<MyGraph>| class.sub_class_of());
            let i = TransitiveIterator::new(class.sub_class_of(), closure);
            let c1 = translator.translate_blank_node_or_iri(&mut w,
                                                            &class.this()
                                                                .to_blank_node_or_iri()
                                                                .unwrap());
            for parent in i {
                let o = parent.this().as_iri().unwrap();
                let iri = w.create_iri(o);
                w.add(&c1, &rdfs_sub_class_of, &WriterResource::IRI(iri));
            }
        }
    }

    Ok(w.collect().sort_blank_nodes())
}

fn write_mod(o: &Output, iris: &[String]) -> rome::Result<()> {
    let path = o.output_dir.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    mod_rs.write_all(b"/// Ontology classes\n")?;
    mod_rs.write_all(b"pub mod classes;\n")?;
    mod_rs.write_all(b"/// Ontology properties\n")?;
    mod_rs.write_all(b"pub mod properties;\n")?;
    if o.internal {
        mod_rs.write_all(b"use graph;\n")?;
        mod_rs.write_all(b"use ontology_adapter;\n")?;
    } else {
        mod_rs.write_all(b"use rome::graph;\n")?;
        mod_rs.write_all(b"use rome::ontology_adapter;\n")?;
    }
    mod_rs.write_all(b"/// Adapter to access RDF data in graph via the ontology\n")?;
    mod_rs.write_all(b"pub fn adapter<'g, G>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
    where G: graph::Graph<'g>
{
    let mut iris = Vec::with_capacity(")?;
    mod_rs.write_all(format!("{});\n", iris.len()).as_bytes())?;
    for iri in iris {
        mod_rs.write_all(format!("    iris.push(graph.find_iri(\"{}\"));\n", iri).as_bytes())?;
    }
    mod_rs.write_all(b"    ontology_adapter::OntologyAdapter::new(graph, iris)\n}\n")?;
    Ok(())
}

fn load_files(inputs: &[String]) -> rome::Result<(Namespaces, MyGraph)> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    let mut prefixes = Namespaces::new();
    for input in inputs {
        let data = read_file(input)?;
        let mut base = String::from("file:");
        base.push_str(input);
        let mut triples = TurtleParser::new(data.as_str(), &base, &mut writer)?;
        while let Some(step) = triples.next() {
            step?;
        }
        for ns in triples.prefixes().iter() {
            prefixes.set(ns.prefix(), ns.namespace());
        }
    }
    let graph = writer.collect();
    let graph = infer(&graph)?;
    Ok((prefixes, graph))
}

fn write_comment<'g, W, C>(r: &C, writer: &mut W) -> rome::Result<()>
    where W: Write,
          C: 'g + Comment<'g>,
          <C as rome::resource::ResourceBase<'g>>::Graph: 'g
{
    for comment in r.comment() {
        if let Some(l) = comment.this().as_literal() {
            writer.write_all(b"\n/// ")?;
            writer.write_all(comment_escape(l.as_str()).as_bytes())?;
        }
    }
    Ok(())
}

fn generate_classes(d: &OntoData, iris: &mut Vec<String>) -> rome::Result<()> {
    let mut outputs = BTreeMap::new();
    for ns in d.prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
    }
    for class in &d.classes {
        let iri = class.as_str();
        if let Some((prefix, name)) = d.prefixes.find_prefix(iri) {
            if let Some(mut writer) = outputs.get_mut(prefix) {
                writer.write_all(b"\nclass!(\n/// **")?;
                writer.write_all(prefix)?;
                writer.write_all(b":")?;
                writer.write_all(name.as_bytes())?;
                writer.write_all(b"**")?;
                write_comment(class, writer)?;
                writer.write_all(format!("\n:\"{}\", {},\n{});\n",
                                       iri,
                                       camel_case(name),
                                       iris.len())
                        .as_bytes())?;
                let mut done = HashSet::new();
                write_impl_properties(class, class, d, &mut done, &mut writer)?;
                iris.push(String::from(iri));
            }
        }
    }
    write_files(&d.o, &outputs, "classes", true)
}
fn generate_properties(d: &OntoData, iris: &mut Vec<String>) -> rome::Result<()> {
    let mut outputs = BTreeMap::new();
    for ns in d.prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
    }
    for property in &d.properties {
        let iri = property.as_str();
        if let Some((prop_prefix, prop)) = d.prefixes.find_prefix(iri) {
            for range in property.range() {
                if let Some((prefix, range)) =
                    range.this().as_iri().and_then(|i| d.prefixes.find_prefix(i.as_str())) {
                    if let Some(mut writer) = outputs.get_mut(prop_prefix) {
                        writer.write_all(b"\nproperty!(\n/// **")?;
                        writer.write_all(prop_prefix)?;
                        writer.write_all(b":")?;
                        writer.write_all(prop.as_bytes())?;
                        writer.write_all(b"**")?;
                        write_comment(property, writer)?;
                        writer.write_all(format!("\n:\"{}\", {}, {},\n{}::classes::{}::{}<'g, \
                                                G>,\n{});\n",
                                               iri,
                                               camel_case(prop),
                                               snake_case(prop),
                                               d.o.mod_name,
                                               String::from_utf8_lossy(prefix),
                                               camel_case(range),
                                               iris.len())
                                .as_bytes())?;
                    }
                }
            }
            iris.push(String::from(iri));
        }
    }
    write_files(&d.o, &outputs, "properties", false)
}

fn uses(o: &Output, classes: bool) -> String {
    let mut uses = String::new();
    uses.push_str("use std;\n");
    if o.internal {
        uses.push_str("use graph;\n");
        uses.push_str("use resource;\n");
        if classes {
            uses.push_str("use ontology_adapter;\n");
        }
    } else {
        uses.push_str("use rome::graph;\n");
        uses.push_str("use rome::resource;\n");
        if classes {
            uses.push_str("use rome::ontology_adapter;\n");
        }
    }
    uses.push_str(&format!("use {};\n", o.mod_name));
    uses
}

fn write_files(o: &Output, writers: &Writers, folder: &str, classes: bool) -> rome::Result<()> {
    let uses = uses(o, classes);
    let dir_path = o.output_dir.join(folder);
    let metadata = fs::metadata(&dir_path);
    if !fs::metadata(&dir_path).is_ok() || !metadata?.is_dir() {
        fs::create_dir(&dir_path)?;
    }
    let path = dir_path.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    for (prefix, content) in writers.iter() {
        if !content.is_empty() {
            let mut filename = String::from_utf8_lossy(prefix).into_owned();
            filename.push_str(".rs");
            let path = dir_path.join(filename);
            let mut file = fs::File::create(path)?;
            file.write_all(uses.as_bytes())?;
            file.write_all(content)?;
            mod_rs.write_all(b"/// ontology namespace ")?;
            mod_rs.write_all(prefix)?;
            mod_rs.write_all(b"\npub mod ")?;
            mod_rs.write_all(prefix)?;
            mod_rs.write_all(b";\n")?;
        }
    }
    Ok(())
}

fn generate(output_dir: PathBuf,
            mod_name: String,
            internal: bool,
            inputs: &[String])
            -> rome::Result<()> {
    let (prefixes, graph) = load_files(inputs)?;
    let oa = ontology::adapter(&graph);
    let mut iris = Vec::new();
    let data = OntoData {
        o: Output {
            mod_name: mod_name,
            output_dir: output_dir,
            internal: internal,
        },
        classes: IRI::iter(&oa).collect(),
        properties: IRI::iter(&oa).collect(),
        prefixes: prefixes,
    };

    // rdf:type is always needed
    iris.push(String::from(RDF_TYPE));
    generate_classes(&data, &mut iris)?;
    generate_properties(&data, &mut iris)?;
    write_mod(&data.o, &iris)?;
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
    let output_dir = if args.len() > 2 && arg == "--mod" {
        mod_name = args.next().unwrap();
        internal = false;
        args.next().unwrap()
    } else {
        mod_name = String::from("ontology");
        internal = true;
        arg
    };
    let inputs = args.collect::<Vec<_>>();
    if let Err(e) = generate(PathBuf::from(output_dir), mod_name, internal, &inputs) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
