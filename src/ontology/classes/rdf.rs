use std;
use graph;
use resource;
use ontology_adapter;
use ontology;

class!(
/// **rdf:Alt**
/// The class of containers of alternatives.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt", Alt,
1);
impl<G> ontology::properties::rdf::Type for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Alt<G> where G: graph::Graph {}

class!(
/// **rdf:Bag**
/// The class of unordered containers.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag", Bag,
2);
impl<G> ontology::properties::rdf::Type for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Bag<G> where G: graph::Graph {}

class!(
/// **rdf:HTML**
/// The datatype of RDF literals storing fragments of HTML content
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML", HTML,
3);
impl<G> ontology::properties::rdf::Type for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for HTML<G> where G: graph::Graph {}

class!(
/// **rdf:List**
/// The class of RDF Lists.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#List", List,
4);
impl<G> ontology::properties::rdf::First for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Rest for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for List<G> where G: graph::Graph {}

class!(
/// **rdf:PlainLiteral**
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral", PlainLiteral,
5);
impl<G> ontology::properties::rdf::Type for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for PlainLiteral<G> where G: graph::Graph {}

class!(
/// **rdf:Property**
/// The class of RDF properties.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property,
6);
impl<G> ontology::properties::rdfs::Domain for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Range for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubPropertyOf for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Property<G> where G: graph::Graph {}

class!(
/// **rdf:Seq**
/// The class of ordered containers.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq", Seq,
7);
impl<G> ontology::properties::rdf::Type for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Seq<G> where G: graph::Graph {}

class!(
/// **rdf:Statement**
/// The class of RDF statements.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement", Statement,
8);
impl<G> ontology::properties::rdf::Object for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Predicate for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Subject for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Statement<G> where G: graph::Graph {}

class!(
/// **rdf:XMLLiteral**
/// The datatype of XML literal values.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral", XMLLiteral,
9);
impl<G> ontology::properties::rdf::Type for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for XMLLiteral<G> where G: graph::Graph {}

class!(
/// **rdf:langString**
/// The datatype of language-tagged string values
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#langString", LangString,
10);
impl<G> ontology::properties::rdf::Type for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for LangString<G> where G: graph::Graph {}
