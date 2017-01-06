use std;
use graph;
use resource;
use ontology;

/// rdfs:comment
/// A description of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, ontology::classes::rdfs::Literal<G>, 24);

/// rdfs:domain
/// A domain of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, ontology::classes::rdfs::Class<G>, 25);

/// rdfs:isDefinedBy
/// The defininition of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#isDefinedBy", IsDefinedBy, is_defined_by, ontology::classes::rdfs::Resource<G>, 26);

/// rdfs:label
/// A human-readable name for the subject.
property!("http://www.w3.org/2000/01/rdf-schema#label", Label, label, ontology::classes::rdfs::Literal<G>, 27);

/// rdfs:member
/// A member of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#member", Member, member, ontology::classes::rdfs::Resource<G>, 28);

/// rdfs:range
/// A range of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#range", Range, range, ontology::classes::rdfs::Class<G>, 29);

/// rdfs:seeAlso
/// Further information about the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#seeAlso", SeeAlso, see_also, ontology::classes::rdfs::Resource<G>, 30);

/// rdfs:subClassOf
/// The subject is a subclass of a class.
property!("http://www.w3.org/2000/01/rdf-schema#subClassOf", SubClassOf, sub_class_of, ontology::classes::rdfs::Class<G>, 31);

/// rdfs:subPropertyOf
/// The subject is a subproperty of a property.
property!("http://www.w3.org/2000/01/rdf-schema#subPropertyOf", SubPropertyOf, sub_property_of, ontology::classes::rdf::Property<G>, 32);
