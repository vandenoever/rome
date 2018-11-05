use graph;
use ontology;
use ontology::iri::rdf;
use resource;
use std;

property!(
/// **rdf:first**
/// The first item in the subject RDF list.
:rdf::FIRST, First, first,
ontology::classes::rdfs::Resource<'g, G>,
87);

property!(
/// **rdf:object**
/// The object of the subject RDF statement.
:rdf::OBJECT, Object, object,
ontology::classes::rdfs::Resource<'g, G>,
88);

property!(
/// **rdf:predicate**
/// The predicate of the subject RDF statement.
:rdf::PREDICATE, Predicate, predicate,
ontology::classes::rdfs::Resource<'g, G>,
89);

property!(
/// **rdf:rest**
/// The rest of the subject RDF list after the first item.
:rdf::REST, Rest, rest,
ontology::classes::rdf::List<'g, G>,
90);

property!(
/// **rdf:subject**
/// The subject of the subject RDF statement.
:rdf::SUBJECT, Subject, subject,
ontology::classes::rdfs::Resource<'g, G>,
91);

property!(
/// **rdf:type**
/// The subject is an instance of a class.
:rdf::TYPE, Type, a,
ontology::classes::rdfs::Class<'g, G>,
92);

property!(
/// **rdf:value**
/// Idiomatic property used for structured values.
:rdf::VALUE, Value, value,
ontology::classes::rdfs::Resource<'g, G>,
93);
