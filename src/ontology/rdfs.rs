use std;
use graph;
use resource;
use ontology::rdf;
use ontology::rdfs;

/// rdfs:Class
/// The class of classes.
class!("http://www.w3.org/2000/01/rdf-schema#Class", Class);
impl<G> rdfs::SubClassOf<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::Class<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::Class<G> where G: graph::Graph {}

/// rdfs:Container
/// The class of RDF containers.
class!("http://www.w3.org/2000/01/rdf-schema#Container", Container);
impl<G> rdf::Type<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::Container<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::Container<G> where G: graph::Graph {}

/// rdfs:ContainerMembershipProperty
/// The class of container membership properties, rdf:_1, rdf:_2, ...,                    all of which are sub-properties of 'member'.
class!("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty", ContainerMembershipProperty);
impl<G> rdfs::Domain<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::Range<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::SubPropertyOf<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::ContainerMembershipProperty<G> where G: graph::Graph {}

/// rdfs:Datatype
/// The class of RDF datatypes.
class!("http://www.w3.org/2000/01/rdf-schema#Datatype", Datatype);
impl<G> rdfs::SubClassOf<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdf::Type<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::Datatype<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::Datatype<G> where G: graph::Graph {}

/// rdfs:Literal
/// The class of literal values, eg. textual strings and integers.
class!("http://www.w3.org/2000/01/rdf-schema#Literal", Literal);
impl<G> rdf::Type<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::Literal<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::Literal<G> where G: graph::Graph {}

/// rdfs:Resource
/// The class resource, everything.
class!("http://www.w3.org/2000/01/rdf-schema#Resource", Resource);
impl<G> rdf::Type<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdf::Value<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdfs::Comment<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdfs::IsDefinedBy<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdfs::Label<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdfs::Member<G> for rdfs::Resource<G> where G: graph::Graph {}
impl<G> rdfs::SeeAlso<G> for rdfs::Resource<G> where G: graph::Graph {}

/// rdfs:comment
/// A description of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment, rdfs::Literal<G>);

/// rdfs:domain
/// A domain of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain, rdfs::Class<G>);

/// rdfs:isDefinedBy
/// The defininition of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#isDefinedBy", IsDefinedBy, is_defined_by, rdfs::Resource<G>);

/// rdfs:label
/// A human-readable name for the subject.
property!("http://www.w3.org/2000/01/rdf-schema#label", Label, label, rdfs::Literal<G>);

/// rdfs:member
/// A member of the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#member", Member, member, rdfs::Resource<G>);

/// rdfs:range
/// A range of the subject property.
property!("http://www.w3.org/2000/01/rdf-schema#range", Range, range, rdfs::Class<G>);

/// rdfs:seeAlso
/// Further information about the subject resource.
property!("http://www.w3.org/2000/01/rdf-schema#seeAlso", SeeAlso, see_also, rdfs::Resource<G>);

/// rdfs:subClassOf
/// The subject is a subclass of a class.
property!("http://www.w3.org/2000/01/rdf-schema#subClassOf", SubClassOf, sub_class_of, rdfs::Class<G>);

/// rdfs:subPropertyOf
/// The subject is a subproperty of a property.
property!("http://www.w3.org/2000/01/rdf-schema#subPropertyOf", SubPropertyOf, sub_property_of, rdf::Property<G>);
