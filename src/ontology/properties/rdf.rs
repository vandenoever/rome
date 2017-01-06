use std;
use graph;
use resource;
use ontology;

/// rdf:first
/// The first item in the subject RDF list.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first", First, first, ontology::classes::rdfs::Resource<G>, 17);

/// rdf:object
/// The object of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#object", Object, object, ontology::classes::rdfs::Resource<G>, 18);

/// rdf:predicate
/// The predicate of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate", Predicate, predicate, ontology::classes::rdfs::Resource<G>, 19);

/// rdf:rest
/// The rest of the subject RDF list after the first item.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest", Rest, rest, ontology::classes::rdf::List<G>, 20);

/// rdf:subject
/// The subject of the subject RDF statement.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject", Subject, subject, ontology::classes::rdfs::Resource<G>, 21);

/// rdf:type
/// The subject is an instance of a class.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type", Type, a, ontology::classes::rdfs::Class<G>, 22);

/// rdf:value
/// Idiomatic property used for structured values.
property!("http://www.w3.org/1999/02/22-rdf-syntax-ns#value", Value, value, ontology::classes::rdfs::Resource<G>, 23);
