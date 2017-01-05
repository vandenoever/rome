use std;
use graph;
use resource;
use ontology_adapter;
use ontology::rdfs;
use ontology::rdf;

/// rdf:Alt
/// The class of containers of alternatives.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt", Alt, 0);
impl<G> rdf::Type<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::Alt<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::Alt<G> where G: graph::Graph {}

/// rdf:Bag
/// The class of unordered containers.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag", Bag, 1);
impl<G> rdf::Type<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::Bag<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::Bag<G> where G: graph::Graph {}

/// rdf:HTML
/// The datatype of RDF literals storing fragments of HTML content
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML", HTML, 2);
impl<G> rdf::Type<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::HTML<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::HTML<G> where G: graph::Graph {}

/// rdf:List
/// The class of RDF Lists.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#List", List, 3);
impl<G> rdf::First<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdf::Rest<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::List<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::List<G> where G: graph::Graph {}

/// rdf:PlainLiteral
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral", PlainLiteral, 4);
impl<G> rdf::Type<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::PlainLiteral<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::PlainLiteral<G> where G: graph::Graph {}

/// rdf:Property
/// The class of RDF properties.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property, 5);
impl<G> rdfs::Domain<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::Range<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::SubPropertyOf<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::Property<G> where G: graph::Graph {}

/// rdf:Seq
/// The class of ordered containers.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq", Seq, 6);
impl<G> rdf::Type<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::Seq<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::Seq<G> where G: graph::Graph {}

/// rdf:Statement
/// The class of RDF statements.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement", Statement, 7);
impl<G> rdf::Object<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdf::Predicate<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdf::Subject<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::Statement<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::Statement<G> where G: graph::Graph {}

/// rdf:XMLLiteral
/// The datatype of XML literal values.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral", XMLLiteral, 8);
impl<G> rdf::Type<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::XMLLiteral<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::XMLLiteral<G> where G: graph::Graph {}

/// rdf:langString
/// The datatype of language-tagged string values
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString", LangString, 9);
impl<G> rdf::Type<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdf::LangString<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdf::LangString<G> where G: graph::Graph {}

/// rdf:first
/// The first item in the subject RDF list.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first", First, first, rdfs::Resource<G>, 16);

/// rdf:object
/// The object of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#object", Object, object, rdfs::Resource<G>, 17);

/// rdf:predicate
/// The predicate of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate", Predicate, predicate, rdfs::Resource<G>, 18);

/// rdf:rest
/// The rest of the subject RDF list after the first item.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest", Rest, rest, rdf::List<G>, 19);

/// rdf:subject
/// The subject of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject", Subject, subject, rdfs::Resource<G>, 20);

/// rdf:type
/// The subject is an instance of a class.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", Type, a, rdfs::Class<G>, 21);

/// rdf:value
/// Idiomatic property used for structured values.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#value", Value, value, rdfs::Resource<G>, 22);
