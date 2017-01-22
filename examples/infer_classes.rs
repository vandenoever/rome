/// A simple example program that infers classes
///
/// The program reads a Turtle or N-Triples file and infers Class instances by
/// using rdf:subClassOf.

extern crate rome;
use rome::graph::{Graph, GraphWriter, Triple, WriterResource, ResourceTranslator};
use rome::graphs::tel;
use rome::io::{TurtleParser, write_turtle};
use rome::namespaces::Namespaces;
use std::collections::BTreeMap;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

type MyGraph = tel::Graph64;

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

fn load_graph(data: &str, base: &str) -> rome::Result<MyGraph> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    {
        let mut triples = TurtleParser::new(data, base, &mut writer)?;
        while let Some(step) = triples.next() {
            step?;
        }
    }
    Ok(writer.collect().sort_blank_nodes())
}

fn output_as_turtle(graph: &MyGraph) -> rome::Result<()> {
    let mut ns = Namespaces::new();
    ns.set(b"rdfs", "http://www.w3.org/2000/01/rdf-schema#");
    write_turtle(&ns, graph.iter(), graph, &mut ::std::io::stdout())?;
    Ok(())
}

const RDFS_SUB_CLASS_OF: &'static str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
const RDFS_CLASS: &'static str = "http://www.w3.org/2000/01/rdf-schema#Class";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

struct Translator<'g> {
    blank_nodes: BTreeMap<<MyGraph as Graph<'g>>::BlankNodePtr,
                          <tel::GraphCreator<tel::Triple64SPO,
                           tel::Triple64OPS> as GraphWriter<'g>>::BlankNode>
}

impl<'g> Translator<'g> {
    fn new() -> Translator<'g> {
        Translator { blank_nodes: BTreeMap::new() }
    }
}

impl<'g> ResourceTranslator<'g> for Translator<'g> {
    type Graph = MyGraph;
    type GraphWriter = tel::GraphCreator<tel::Triple64SPO, tel::Triple64OPS>;
    fn translate_blank_node(&mut self,
                            w: &mut Self::GraphWriter,
                            blank_node: &<Self::Graph as Graph<'g>>::BlankNodePtr
        ) -> <Self::GraphWriter as GraphWriter<'g>>::BlankNode {
        if let Some(blank_node) = self.blank_nodes.get(blank_node) {
            return blank_node.clone();
        }
        let new_blank_node = w.create_blank_node();
        self.blank_nodes.insert(blank_node.clone(), new_blank_node.clone());
        new_blank_node
    }
}

fn infer(graph: &MyGraph) -> rome::Result<MyGraph> {
    // for every triple with rdfs:subClassOf infer that the subject and the
    // object are rdfs:Class instances
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
    Ok(w.collect().sort_blank_nodes())
}

fn run(path: &str, base: &str) -> rome::Result<()> {
    let data = read_file(path)?;
    let graph = load_graph(data.as_str(), base)?;
    let result = infer(&graph)?;
    output_as_turtle(&result)?;
    Ok(())
}

fn check_file<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(meta) => meta.is_file(),
        _ => false,
    }
}

fn main() {
    let mut args = args();
    args.next();
    let mut args_ok = false;
    let mut base = String::new();
    let mut path = String::new();
    if args.len() == 1 {
        path = args.next().unwrap();
        args_ok = check_file(&path);
        base.push_str("file:");
        base.push_str(&path);
    } else if args.len() == 3 {
        args_ok = args.next().unwrap() == "--base";
        base.push_str(&args.next().unwrap());
        path = args.next().unwrap();
        args_ok &= check_file(&path);
    }
    if !args_ok {
        println_stderr!("Usage: [--base BASE] infer_classes INPUT_FILE");
        std::process::exit(-1);
    }
    if let Err(e) = run(path.as_str(), base.as_str()) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
