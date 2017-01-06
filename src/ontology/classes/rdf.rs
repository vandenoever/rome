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
impl<G> ontology::properties::rdf::Type<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Alt<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Alt<G> where G: graph::Graph {}

class!(
/// **rdf:Bag**
/// The class of unordered containers.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag", Bag,
2);
impl<G> ontology::properties::rdf::Type<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Bag<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Bag<G> where G: graph::Graph {}

class!(
/// **rdf:HTML**
/// The datatype of RDF literals storing fragments of HTML content
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML", HTML,
3);
impl<G> ontology::properties::rdf::Type<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for HTML<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for HTML<G> where G: graph::Graph {}

class!(
/// **rdf:List**
/// The class of RDF Lists.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#List", List,
4);
impl<G> ontology::properties::rdf::First<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Rest<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for List<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for List<G> where G: graph::Graph {}

class!(
/// **rdf:PlainLiteral**
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral", PlainLiteral,
5);
impl<G> ontology::properties::rdf::Type<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for PlainLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for PlainLiteral<G> where G: graph::Graph {}

class!(
/// **rdf:Property**
/// The class of RDF properties.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property,
6);
impl<G> ontology::properties::rdfs::Domain<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Range<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubPropertyOf<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Property<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Property<G> where G: graph::Graph {}

class!(
/// **rdf:Seq**
/// The class of ordered containers.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq", Seq,
7);
impl<G> ontology::properties::rdf::Type<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Seq<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Seq<G> where G: graph::Graph {}

class!(
/// **rdf:Statement**
/// The class of RDF statements.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement", Statement,
8);
impl<G> ontology::properties::rdf::Object<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Predicate<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Subject<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Statement<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Statement<G> where G: graph::Graph {}

class!(
/// **rdf:XMLLiteral**
/// The datatype of XML literal values.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral", XMLLiteral,
9);
impl<G> ontology::properties::rdf::Type<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for XMLLiteral<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for XMLLiteral<G> where G: graph::Graph {}

class!(
/// **rdf:langString**
/// The datatype of language-tagged string values
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#langString", LangString,
10);
impl<G> ontology::properties::rdf::Type<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for LangString<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for LangString<G> where G: graph::Graph {}
