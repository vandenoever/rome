use std;
use graph;
use resource;
use ontology_adapter;
use ontology;

/// rdfs:Class
/// The class of classes.
class!("http://www.w3.org/2000/01/rdf-schema#Class", Class, 11);
impl<G> ontology::properties::rdfs::SubClassOf<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Class<G> where G: graph::Graph {}

/// rdfs:Container
/// The class of RDF containers.
class!("http://www.w3.org/2000/01/rdf-schema#Container", Container, 12);
impl<G> ontology::properties::rdf::Type<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Container<G> where G: graph::Graph {}

/// rdfs:ContainerMembershipProperty
/// The class of container membership properties, rdf:_1, rdf:_2, ...,                    all of which are sub-properties of 'member'.
class!("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty", ContainerMembershipProperty, 13);
impl<G> ontology::properties::rdfs::Domain<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Range<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubPropertyOf<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for ContainerMembershipProperty<G> where G: graph::Graph {}

/// rdfs:Datatype
/// The class of RDF datatypes.
class!("http://www.w3.org/2000/01/rdf-schema#Datatype", Datatype, 14);
impl<G> ontology::properties::rdfs::SubClassOf<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Datatype<G> where G: graph::Graph {}

/// rdfs:Literal
/// The class of literal values, eg. textual strings and integers.
class!("http://www.w3.org/2000/01/rdf-schema#Literal", Literal, 15);
impl<G> ontology::properties::rdf::Type<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Literal<G> where G: graph::Graph {}

/// rdfs:Resource
/// The class resource, everything.
class!("http://www.w3.org/2000/01/rdf-schema#Resource", Resource, 16);
impl<G> ontology::properties::rdf::Type<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member<G> for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso<G> for Resource<G> where G: graph::Graph {}
