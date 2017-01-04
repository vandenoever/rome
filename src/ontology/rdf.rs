use std;
use graph;
use resource;
use ontology::rdf;
use ontology::rdfs;

/// rdf:Alt
/// The class of containers of alternatives.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt", Alt);

/// rdf:Bag
/// The class of unordered containers.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag", Bag);

/// rdf:HTML
/// The datatype of RDF literals storing fragments of HTML content
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML", HTML);

/// rdf:List
/// The class of RDF Lists.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#List", List);

/// rdf:PlainLiteral
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral", PlainLiteral);

/// rdf:Property
/// The class of RDF properties.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property);

/// rdf:Seq
/// The class of ordered containers.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq", Seq);

/// rdf:Statement
/// The class of RDF statements.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement", Statement);

/// rdf:XMLLiteral
/// The datatype of XML literal values.
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral", XMLLiteral);

/// rdf:langString
/// The datatype of language-tagged string values
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString", LangString);

/// rdfs:Container
class!("http://www.w3.org/2000/01/rdf-schema#Container", Container);

/// rdfs:Literal
class!("http://www.w3.org/2000/01/rdf-schema#Literal", Literal);

/// rdfs:Resource
class!("http://www.w3.org/2000/01/rdf-schema#Resource", Resource);

/// rdf:first
/// The first item in the subject RDF list.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first", First, first, rdfs::Resource<G>);
impl<G> rdf::First<G> for rdf::List<G> where G: graph::Graph {}

/// rdf:object
/// The object of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#object", Object, object, rdfs::Resource<G>);
impl<G> rdf::Object<G> for rdf::Statement<G> where G: graph::Graph {}

/// rdf:predicate
/// The predicate of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate", Predicate, predicate, rdfs::Resource<G>);
impl<G> rdf::Predicate<G> for rdf::Statement<G> where G: graph::Graph {}

/// rdf:rest
/// The rest of the subject RDF list after the first item.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest", Rest, rest, rdf::List<G>);
impl<G> rdf::Rest<G> for rdf::List<G> where G: graph::Graph {}

/// rdf:subject
/// The subject of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject", Subject, subject, rdfs::Resource<G>);
impl<G> rdf::Subject<G> for rdf::Statement<G> where G: graph::Graph {}

/// rdf:type
/// The subject is an instance of a class.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", Type, a, rdfs::Class<G>);
impl<G> rdf::Type<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdf:value
/// Idiomatic property used for structured values.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#value", Value, value, rdfs::Resource<G>);
impl<G> rdf::Value<G> for rdfs::Resource<G> where G: graph::Graph {}
