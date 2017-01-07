use std;
use graph;
use resource;
use ontology;

property!(
/// **rdf:first**
/// The first item in the subject RDF list.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#first", First, first,
ontology::classes::rdfs::Resource<'g,G>,
17);

property!(
/// **rdf:object**
/// The object of the subject RDF statement.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#object", Object, object,
ontology::classes::rdfs::Resource<'g,G>,
18);

property!(
/// **rdf:predicate**
/// The predicate of the subject RDF statement.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate", Predicate, predicate,
ontology::classes::rdfs::Resource<'g,G>,
19);

property!(
/// **rdf:rest**
/// The rest of the subject RDF list after the first item.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#rest", Rest, rest,
ontology::classes::rdf::List<'g,G>,
20);

property!(
/// **rdf:subject**
/// The subject of the subject RDF statement.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#subject", Subject, subject,
ontology::classes::rdfs::Resource<'g,G>,
21);

property!(
/// **rdf:type**
/// The subject is an instance of a class.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#type", Type, a,
ontology::classes::rdfs::Class<'g,G>,
22);

property!(
/// **rdf:value**
/// Idiomatic property used for structured values.
:"http://www.w3.org/1999/02/22-rdf-syntax-ns#value", Value, value,
ontology::classes::rdfs::Resource<'g,G>,
23);
