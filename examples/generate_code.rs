/// Generate rust code from a set of ontologies
///
extern crate rome;
use rome::graph::{
    BlankNodeOrIRI, Graph, GraphWriter, IRIPtr, LiteralPtr, Resource, ResourceTranslator, Triple,
    WriterBlankNodeOrIRI, WriterResource,
};
use rome::graphs::tel;
use rome::io::TurtleParser;
use rome::iter::TransitiveIterator;
use rome::namespaces::Namespaces;
use rome::ontology;
use rome::ontology::classes::rdf::Property;
use rome::ontology::classes::rdfs::Class;
use rome::ontology::iri;
use rome::ontology::properties::rdfs::{Comment, Domain, Range, SubClassOf};
use rome::ontology_adapter;
use rome::resource::{ResourceBase, IRI};
use std::collections::HashSet;
use std::collections::{btree_map, BTreeMap, BTreeSet};
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
    if str.is_empty() {
        return String::new();
    }
    if str == "type" {
        return String::from("a");
    }
    let mut s = str[0..1].to_lowercase();
    let chars: Vec<_> = str.chars().collect();
    for (pos, c) in chars.iter().enumerate().skip(1) {
        if c.is_uppercase() {
            if !chars[pos - 1].is_uppercase() {
                s.push('_');
            }
            for l in c.to_lowercase() {
                s.push(l);
            }
        } else {
            s.push(*c);
        }
    }
    s.replace("-", "_")
}

fn comment_escape(str: &str) -> String {
    str.replace("\n", "")
}

fn write_impl_property<'g, G, W>(
    class: &IRI<'g, Class<'g, G>>,
    property: &IRI<'g, Property<'g, G>>,
    mod_name: &str,
    prefixes: &Namespaces,
    done: &mut HashSet<String>,
    writer: &mut W,
) -> rome::Result<()>
where
    W: Write,
    G: Graph<'g>,
{
    let iri = property.as_str();
    if done.contains(iri) {
        return Ok(());
    }
    if let Some((prop_prefix, prop)) = prefixes.find_prefix(iri) {
        if let Some((_, domain)) = prefixes.find_prefix(class.as_str()) {
            writer.write_all(
                format!(
                    "impl<'g, G: 'g> {}::properties::{}::{}<'g> for {}<'g, G> \
                     where G: graph::Graph<'g> {{}}\n",
                    mod_name,
                    prop_prefix,
                    camel_case(prop),
                    camel_case(domain)
                ).as_bytes(),
            )?;
            writer.write_all(
                format!(
                    "impl<'g, G: 'g> {}::properties::{}::{}<'g> for \
                     resource::IRI<'g, {}<'g, G>> where G: graph::Graph<'g> \
                     {{}}\n",
                    mod_name,
                    prop_prefix,
                    camel_case(prop),
                    camel_case(domain)
                ).as_bytes(),
            )?;
            done.insert(String::from(iri));
        }
    }
    Ok(())
}

fn write_impl_properties<'g, W>(
    class: &IRI<'g, Class<'g, MyGraph>>,
    parent: &Class<'g, MyGraph>,
    d: &OntoData<'g>,
    done: &mut HashSet<String>,
    writer: &mut W,
) -> rome::Result<()>
where
    W: Write,
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

struct Translator<'g> {
    blank_nodes: BTreeMap<
        <MyGraph as Graph<'g>>::BlankNodePtr,
        <tel::GraphCreator<tel::Triple128SPO, tel::Triple128OPS> as GraphWriter<'g>>::BlankNode,
    >,
}

impl<'g> Translator<'g> {
    fn new() -> Translator<'g> {
        Translator {
            blank_nodes: BTreeMap::new(),
        }
    }
}

impl<'g> ResourceTranslator<'g> for Translator<'g> {
    type Graph = MyGraph;
    type GraphWriter = tel::GraphCreator<tel::Triple128SPO, tel::Triple128OPS>;
    fn translate_blank_node(
        &mut self,
        w: &mut Self::GraphWriter,
        blank_node: &<Self::Graph as Graph<'g>>::BlankNodePtr,
    ) -> <Self::GraphWriter as GraphWriter<'g>>::BlankNode {
        match self.blank_nodes.entry(blank_node.clone()) {
            btree_map::Entry::Occupied(entry) => *entry.get(),
            btree_map::Entry::Vacant(entry) => {
                let new_blank_node = w.create_blank_node();
                entry.insert(new_blank_node);
                new_blank_node
            }
        }
    }
}

/// CONSTRUCT {
///   ?s a ?class
/// } WHERE {
///   ?s ?p ?o .
///   ?p rdfs:domain ?class
/// }
fn infer_class_from_domain<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g, Graph = G>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let Some(rdfs_domain) = graph.find_iri(iri::rdfs::DOMAIN) {
        let rdf_type = w.create_iri(&iri::rdf::TYPE);
        for triple in graph.iter().filter(|t| t.predicate() == rdfs_domain) {
            let class = translator.translate_resource(w, &triple.object());
            if let BlankNodeOrIRI::IRI(p) = triple.subject() {
                for triple2 in graph.iter().filter(|t| t.predicate() == p) {
                    let s = translator.translate_blank_node_or_iri(w, &triple2.subject());
                    w.add(&s, &rdf_type, &class);
                }
            }
        }
    }
}

/// CONSTRUCT {
///   ?o a ?class
/// } WHERE {
///   ?s ?p ?o .
///   ?p rdfs:range ?class
///   FILTER(isIRI(?o))
/// }
fn infer_class_from_range<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g, Graph = G>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let Some(rdfs_range) = graph.find_iri(iri::rdfs::RANGE) {
        let rdf_type = w.create_iri(&iri::rdf::TYPE);
        for triple in graph.iter().filter(|t| t.predicate() == rdfs_range) {
            let class = translator.translate_resource(w, &triple.object());
            if let BlankNodeOrIRI::IRI(p) = triple.subject() {
                for triple2 in graph.iter().filter(|t| t.predicate() == p) {
                    if let Resource::IRI(o) = triple2.object() {
                        let s = WriterBlankNodeOrIRI::IRI(w.create_iri(&o));
                        w.add(&s, &rdf_type, &class);
                    }
                }
            }
        }
    }
}

/// For every triple with rdfs:subClassOf infer that the subject and the
/// object are rdfs:Class instances
///
/// CONSTRUCT {
///     ?a a rdfs:Class .
///     ?b a rdfs:Class
/// } WHERE {
///     ?a rdfs:subClassOf ?b
/// }
fn infer_class_from_sub_class_of<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let Some(rdfs_sub_class_of) = graph.find_iri(iri::rdfs::SUB_CLASS_OF) {
        let rdf_type = w.create_iri(&iri::rdf::TYPE);
        let rdfs_class = WriterResource::IRI(w.create_iri(&iri::rdfs::CLASS));
        for triple in graph.iter().filter(|triple| {
            !triple.object().is_literal() && triple.predicate() == rdfs_sub_class_of
        }) {
            let a = translator.translate_blank_node_or_iri(w, &triple.subject());
            w.add(&a, &rdf_type, &rdfs_class);
            if let Some(b) = triple.object().to_blank_node_or_iri() {
                let b = translator.translate_blank_node_or_iri(w, &b);
                w.add(&b, &rdf_type, &rdfs_class);
            }
        }
    }
}

/// CONSTRUCT {
///     ?a a rdfs:Property .
///     ?b a rdfs:Property
/// } WHERE {
///     ?a rdfs:subPropertyOf ?b
/// }
fn infer_property_from_sub_property_of<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let Some(sub_property) = graph.find_iri(iri::rdfs::SUB_PROPERTY_OF) {
        let rdf_type = w.create_iri(&iri::rdf::TYPE);
        let rdf_property = WriterResource::IRI(w.create_iri(&iri::rdf::PROPERTY));
        for triple in graph
            .iter()
            .filter(|triple| !triple.object().is_literal() && triple.predicate() == sub_property)
        {
            let a = translator.translate_blank_node_or_iri(w, &triple.subject());
            w.add(&a, &rdf_type, &rdf_property);
            if let Some(b) = triple.object().to_blank_node_or_iri() {
                let b = translator.translate_blank_node_or_iri(w, &b);
                w.add(&b, &rdf_type, &rdf_property);
            }
        }
    }
}

/// CONSTRUCT {
///   ?x ?r ?y
/// } WHERE {
///   ?x ?q ?y .
///   ?q rdfs:subPropertyOf ?r
/// }
fn infer_statement_from_sub_property_of<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let Some(sub_property) = graph.find_iri(iri::rdfs::SUB_PROPERTY_OF) {
        for triple in graph.iter().filter(|t| t.predicate() == sub_property) {
            if let (BlankNodeOrIRI::IRI(q), Resource::IRI(r)) = (triple.subject(), triple.object())
            {
                let r = w.create_iri(&r);
                for triple2 in graph.iter().filter(|t| t.predicate() == q) {
                    let x = translator.translate_blank_node_or_iri(w, &triple2.subject());
                    let y = translator.translate_resource(w, &triple2.object());
                    w.add(&x, &r, &y);
                }
            }
        }
    }
}

/// CONSTRUCT {
///   ?x ?p ?z
/// } WHERE {
///   ?x ?p ?y .
///   ?y ?p ?z .
///   ?p a owl:TransitiveProperty
/// }
fn infer_transitive_properties<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    if let (Some(transitive_propery), Some(rdf_type)) = (
        graph.find_iri(iri::owl::TRANSITIVE_PROPERTY),
        graph.find_iri(iri::rdf::TYPE),
    ) {
        for triple in graph.iter_o_p(&Resource::IRI(transitive_propery), &rdf_type) {
            if let BlankNodeOrIRI::IRI(p) = triple.subject() {
                for triple2 in graph.iter().filter(|t| t.predicate() == p) {
                    let subject = triple2.subject().to_resource();
                    for triple3 in graph.iter_o_p(&subject, &p) {
                        let x = translator.translate_blank_node_or_iri(w, &triple3.subject());
                        let p = w.create_iri(&p);
                        let y = translator.translate_resource(w, &triple2.object());
                        w.add(&x, &p, &y);
                    }
                }
            }
        }
    }
}

/// Infer properties
///
/// CONSTRUCT {
///   ?p a rdf:Property
/// } WHERE {
///   { ?s ?p ?o }
///   UNION
///   { ?p rdfs:domain ?d }
///   UNION
///   { ?p rdfs:range ?d }
/// }
fn infer_properties<'g, G, W, T>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    let rdf_type = w.create_iri(&iri::rdf::TYPE);
    let rdf_property = WriterResource::IRI(w.create_iri(&iri::rdf::PROPERTY));
    for triple in graph.iter() {
        let predicate = WriterBlankNodeOrIRI::IRI(w.create_iri(&triple.predicate()));
        w.add(&predicate, &rdf_type, &rdf_property);
        if let BlankNodeOrIRI::IRI(iri) = triple.subject() {
            let iri = iri.as_str();
            if iri == iri::rdfs::DOMAIN || iri == iri::rdfs::RANGE {
                let subject = translator.translate_blank_node_or_iri(w, &triple.subject());
                w.add(&subject, &rdf_type, &rdf_property);
            }
        }
    }
}

fn copy_triples<'g, T, G, W>(graph: &'g G, w: &mut W, translator: &mut T)
where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    for triple in graph.iter() {
        let subject = translator.translate_blank_node_or_iri(w, &triple.subject());
        let predicate = w.create_iri(&triple.predicate());
        let object = translator.translate_resource(w, &triple.object());
        w.add(&subject, &predicate, &object);
    }
}

/// CONSTRUCT {
///     ?a rdfs:subClassOf ?b
/// } WHERE {
///     ?a rdfs:subClassOf+ ?b
/// }
fn make_sub_class_of_entailment_concrete<'g, T, G, W>(
    w: &mut W,
    translator: &mut T,
    oa: &'g ontology_adapter::OntologyAdapter<'g, G>,
) where
    G: Graph<'g>,
    W: GraphWriter<'g>,
    T: ResourceTranslator<'g, Graph = G, GraphWriter = W> + 'g,
{
    // make subClassOf entailment concrete
    let rdfs_sub_class_of = w.create_iri(&iri::rdfs::SUB_CLASS_OF);
    for class in Class::iter(&oa) {
        let i = TransitiveIterator::new(class.sub_class_of(), |class: &Class<G>| {
            class.sub_class_of()
        });
        let c1 = translator
            .translate_blank_node_or_iri(w, &class.this().to_blank_node_or_iri().unwrap());
        for parent in i {
            let r = translator.translate_resource(w, parent.this());
            w.add(&c1, &rdfs_sub_class_of, &r);
        }
    }
}

fn infer<'g>(graph: &'g MyGraph) -> rome::Result<MyGraph> {
    let mut w = tel::GraphCreator::with_capacity(65000);
    let oa = ontology::adapter(graph);
    let mut translator = Translator::new();
    infer_class_from_domain(graph, &mut w, &mut translator);
    infer_class_from_range(graph, &mut w, &mut translator);
    infer_class_from_sub_class_of(graph, &mut w, &mut translator);
    infer_property_from_sub_property_of(graph, &mut w, &mut translator);
    infer_statement_from_sub_property_of(graph, &mut w, &mut translator);
    infer_transitive_properties(graph, &mut w, &mut translator);
    infer_properties(graph, &mut w, &mut translator);
    copy_triples(graph, &mut w, &mut translator);
    make_sub_class_of_entailment_concrete(&mut w, &mut translator, &oa);
    Ok(w.collect().sort_blank_nodes())
}

/// Infer new triples until no new triples can be inferred
fn infer_all<'g>(graph: &'g MyGraph) -> rome::Result<MyGraph> {
    let mut len_before = graph.iter().count();
    let mut new_graph = infer(graph)?;
    let mut len_after = new_graph.iter().count();
    if len_after == len_before {
        return Ok(new_graph);
    }
    loop {
        len_before = len_after;
        new_graph = infer(&new_graph)?;
        len_after = new_graph.iter().count();
        if len_before == len_after {
            return Ok(new_graph);
        }
    }
}

fn write_mod(o: &Output, iris: &[IoIri]) -> rome::Result<()> {
    let path = o.output_dir.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    mod_rs.write_all(b"/// Ontology classes\n")?;
    mod_rs.write_all(b"pub mod classes;\n")?;
    mod_rs.write_all(b"/// Ontology IRIs\n")?;
    mod_rs.write_all(b"pub mod iri;\n")?;
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
    mod_rs.write_all(
        b"pub fn adapter<'g, G>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
    where G: graph::Graph<'g>
{
    let mut iris = Vec::with_capacity(",
    )?;
    mod_rs.write_all(format!("{});\n", iris.len()).as_bytes())?;
    for iri in iris {
        mod_rs.write_all(
            format!(
                "    iris.push(graph.find_iri(iri::{}::{}));\n",
                iri.prefix,
                iri.upper_name()
            ).as_bytes(),
        )?;
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
    let graph = infer_all(&graph)?;
    Ok((prefixes, graph))
}

fn write_comment<'g, W, C>(r: &C, writer: &mut W) -> rome::Result<()>
where
    W: Write,
    C: 'g + Comment<'g>,
    <C as rome::resource::ResourceBase<'g>>::Graph: 'g,
{
    for comment in r.comment() {
        if let Some(l) = comment.this().as_literal() {
            writer.write_all(b"\n/// ")?;
            writer.write_all(comment_escape(l.as_str()).as_bytes())?;
        }
    }
    Ok(())
}

fn add_iri(prefixes: &Namespaces, iri: &str, iris: &mut BTreeSet<IoIri>) {
    if let Some((prefix, name)) = prefixes.find_prefix(iri) {
        if !name.is_empty() {
            let io_iri = IoIri::new(prefix, iri, name);
            iris.insert(io_iri);
        }
    }
}

fn collect_iris<'g, G: Graph<'g>>(
    prefixes: &Namespaces,
    graph: &'g G,
    iris: &mut BTreeSet<IoIri>,
) -> rome::Result<()> {
    for triple in graph.iter() {
        if let BlankNodeOrIRI::IRI(iri) = triple.subject() {
            add_iri(prefixes, iri.as_str(), iris);
        }
        let predicate = triple.predicate();
        add_iri(prefixes, predicate.as_str(), iris);
        if let Resource::IRI(iri) = triple.object() {
            add_iri(prefixes, iri.as_str(), iris);
        }
    }
    Ok(())
}

fn generate_classes(d: &OntoData, iris: &mut Vec<IoIri>) -> rome::Result<()> {
    let mut outputs = BTreeMap::new();
    for ns in d.prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
    }
    for class in &d.classes {
        let iri = class.as_str();
        if let Some((prefix, name)) = d.prefixes.find_prefix(iri) {
            if let Some(mut writer) = outputs.get_mut(prefix.as_bytes()) {
                writer.write_all(b"\nclass!(\n/// **")?;
                writer.write_all(prefix.as_bytes())?;
                writer.write_all(b":")?;
                writer.write_all(name.as_bytes())?;
                writer.write_all(b"**")?;
                write_comment(class, writer)?;
                let io_iri = IoIri::new(prefix, iri, name);
                writer.write_all(
                    format!(
                        "\n:{}::{}, {},\n{});\n",
                        &io_iri.prefix,
                        io_iri.upper_name(),
                        camel_case(name),
                        iris.len()
                    ).as_bytes(),
                )?;
                let mut done = HashSet::new();
                write_impl_properties(class, class, d, &mut done, &mut writer)?;
                iris.push(io_iri);
            }
        }
    }
    write_files(&d.o, &outputs, "classes", true, true)
}

fn generate_properties(d: &OntoData, iris: &mut Vec<IoIri>) -> rome::Result<()> {
    let mut outputs = BTreeMap::new();
    for ns in d.prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
    }
    for property in &d.properties {
        let iri = property.as_str();
        if let Some((prop_prefix, prop)) = d.prefixes.find_prefix(iri) {
            let io_iri = IoIri::new(prop_prefix, iri, prop);
            for range in property.range() {
                if let Some((prefix, range)) = range
                    .this()
                    .as_iri()
                    .and_then(|i| d.prefixes.find_prefix(i.as_str()))
                {
                    if let Some(mut writer) = outputs.get_mut(prop_prefix.as_bytes()) {
                        writer.write_all(b"\nproperty!(\n/// **")?;
                        writer.write_all(prop_prefix.as_bytes())?;
                        writer.write_all(b":")?;
                        writer.write_all(prop.as_bytes())?;
                        writer.write_all(b"**")?;
                        write_comment(property, writer)?;
                        writer.write_all(
                            format!(
                                "\n:{}::{}, {}, {},\n{}::classes::{}::{}<'g, \
                                 G>,\n{});\n",
                                &io_iri.prefix,
                                io_iri.upper_name(),
                                camel_case(prop),
                                snake_case(prop),
                                d.o.mod_name,
                                prefix,
                                camel_case(range),
                                iris.len()
                            ).as_bytes(),
                        )?;
                    }
                }
            }
            iris.push(io_iri);
        }
    }
    write_files(&d.o, &outputs, "properties", false, true)
}

fn generate_iris(d: &OntoData, iris: &BTreeSet<IoIri>) -> rome::Result<()> {
    let mut outputs = BTreeMap::new();
    for ns in d.prefixes.iter() {
        outputs.insert(Vec::from(ns.prefix()), Vec::new());
    }
    let iris: BTreeSet<IoIri> = iris.iter().map(|i| i.clone()).collect();
    for iri in iris {
        if let Some(mut writer) = outputs.get_mut(iri.prefix.as_bytes()) {
            writer.write_all(format!("/// {}\n", iri.iri()).as_bytes())?;
            writer.write_all(
                format!(
                    "pub const {}: &str = \"{}\";\n",
                    iri.upper_name(),
                    iri.iri()
                ).as_bytes(),
            )?;
        }
    }
    write_files(&d.o, &outputs, "iri", false, false)
}

fn uses(o: &Output, classes: bool, prefix: &str) -> String {
    let mut uses: Vec<String> = Vec::new();
    uses.push("std".into());
    if o.internal {
        uses.push("graph".into());
        uses.push("resource".into());
        uses.push(format!("ontology::iri::{}", prefix));
        if classes {
            uses.push("ontology_adapter".into());
        }
    } else {
        uses.push("rome::graph".into());
        uses.push("rome::resource".into());
        uses.push(format!("rome::ontology::iri::{}", prefix));
        if classes {
            uses.push("rome::ontology_adapter".into());
        }
    }
    uses.push(o.mod_name.clone());
    uses.sort();
    let mut s = String::new();
    for u in uses {
        s.push_str(&format!("use {};\n", u));
    }
    s
}

fn write_files(
    o: &Output,
    writers: &Writers,
    folder: &str,
    classes: bool,
    has_uses: bool,
) -> rome::Result<()> {
    let dir_path = o.output_dir.join(folder);
    if let Ok(metadata) = fs::metadata(&dir_path) {
        if !metadata.is_dir() {
            println!("{} is not a directory.", dir_path.display());
        }
    } else {
        fs::create_dir(&dir_path)?;
    }
    let path = dir_path.join("mod.rs");
    let mut mod_rs = fs::File::create(path)?;
    for (prefix, content) in writers.iter() {
        let uses = uses(o, classes, &String::from_utf8_lossy(prefix));
        if !content.is_empty() {
            let mut filename = String::from_utf8_lossy(prefix).into_owned();
            filename.push_str(".rs");
            let path = dir_path.join(filename);
            let mut file = fs::File::create(path)?;
            if has_uses {
                file.write_all(uses.as_bytes())?;
            }
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IoIri {
    prefix: String,
    namespace: String,
    name: String,
}

impl IoIri {
    fn new(prefix: &str, iri: &str, name: &str) -> IoIri {
        IoIri {
            prefix: prefix.into(),
            namespace: iri[0..iri.len() - name.len()].into(),
            name: name.into(),
        }
    }
    fn iri(&self) -> String {
        format!("{}{}", self.namespace, self.name)
    }
    fn upper_name(&self) -> String {
        if self.iri() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#type" {
            "TYPE".into()
        } else {
            snake_case(&self.name).to_uppercase()
        }
    }
}

fn generate(
    output_dir: PathBuf,
    mod_name: String,
    internal: bool,
    inputs: &[String],
) -> rome::Result<()> {
    let (prefixes, graph) = load_files(inputs)?;
    let oa = ontology::adapter(&graph);
    let mut iris = Vec::new();
    let mut all_iris = BTreeSet::new();
    let data = OntoData {
        o: Output {
            mod_name,
            output_dir,
            internal,
        },
        classes: IRI::iter(&oa).collect(),
        properties: IRI::iter(&oa).collect(),
        prefixes,
    };

    // rdf:type is always needed
    iris.push(IoIri {
        prefix: "rdf".into(),
        namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into(),
        name: "type".into(),
    });
    collect_iris(&data.prefixes, &graph, &mut all_iris)?;
    generate_classes(&data, &mut iris)?;
    generate_properties(&data, &mut iris)?;
    generate_iris(&data, &all_iris)?;
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
