use std;
use graph;
use resource;
use ontology::rdf;
use ontology::rdfs;

/// rdf:Property
class!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property", Property);

/// rdfs:Class
/// The class of classes.
class!("http://www.w3.org/2000/01/rdf-schema#Class", Class);

/// rdfs:Container
/// The class of RDF containers.
class!("http://www.w3.org/2000/01/rdf-schema#Container", Container);

/// rdfs:ContainerMembershipProperty
/// The class of container membership properties, rdf:_1, rdf:_2, ...,                    all of which are sub-properties of 'member'.
class!("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty", ContainerMembershipProperty);

/// rdfs:Datatype
/// The class of RDF datatypes.
class!("http://www.w3.org/2000/01/rdf-schema#Datatype", Datatype);

/// rdfs:Literal
/// The class of literal values, eg. textual strings and integers.
class!("http://www.w3.org/2000/01/rdf-schema#Literal", Literal);

/// rdfs:Resource
/// The class resource, everything.
class!("http://www.w3.org/2000/01/rdf-schema#Resource", Resource);

/// rdfs:comment
/// A description of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, rdfs::Literal<G>);
impl<G> rdfs::Comment<G> for rdf::Property<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:domain
/// A domain of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, rdfs::Class<G>);
impl<G> rdfs::Domain<G> for rdf::Property<G> where G: graph::Graph {}

/// rdfs:isDefinedBy
/// The defininition of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#isDefinedBy", IsDefinedBy, is_defined_by, rdfs::Resource<G>);
impl<G> rdfs::IsDefinedBy<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:label
/// A human-readable name for the subject.
property!("http://www.w3.org/2000/01/rdf-schema#label", Label, label, rdfs::Literal<G>);
impl<G> rdfs::Label<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:member
/// A member of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#member", Member, member, rdfs::Resource<G>);
impl<G> rdfs::Member<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:range
/// A range of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#range", Range, range, rdfs::Class<G>);
impl<G> rdfs::Range<G> for rdf::Property<G> where G: graph::Graph {}

/// rdfs:seeAlso
/// Further information about the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#seeAlso", SeeAlso, see_also, rdfs::Resource<G>);
impl<G> rdfs::SeeAlso<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:subClassOf
/// The subject is a subclass of a class.
property!("http://www.w3.org/2000/01/rdf-schema#subClassOf", SubClassOf, sub_class_of, rdfs::Class<G>);
impl<G> rdfs::SubClassOf<G> for rdfs::Class<G> where G: graph::Graph {}

/// rdfs:subPropertyOf
/// The subject is a subproperty of a property.
property!("http://www.w3.org/2000/01/rdf-schema#subPropertyOf", SubPropertyOf, sub_property_of, rdf::Property<G>);
impl<G> rdfs::SubPropertyOf<G> for rdf::Property<G> where G: graph::Graph {}
