use std;
use graph;
use resource;
use ontology_adapter;
use ontology;

class!(
/// **rdfs:Class**
/// The class of classes.
:"http://www.w3.org/2000/01/rdf-schema#Class", Class,
11);
impl<G> ontology::properties::rdfs::SubClassOf for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubClassOf for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<Class<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Class<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<Class<G>> where G: graph::Graph {}

class!(
/// **rdfs:Container**
/// The class of RDF containers.
:"http://www.w3.org/2000/01/rdf-schema#Container", Container,
12);
impl<G> ontology::properties::rdf::Type for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<Container<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Container<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<Container<G>> where G: graph::Graph {}

class!(
/// **rdfs:ContainerMembershipProperty**
/// The class of container membership properties, rdf:_1, rdf:_2, ...,                    all of which are sub-properties of 'member'.
:"http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty", ContainerMembershipProperty,
13);
impl<G> ontology::properties::rdfs::Domain for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Domain for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Range for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Range for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubPropertyOf for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubPropertyOf for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for ContainerMembershipProperty<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<ContainerMembershipProperty<G>> where G: graph::Graph {}

class!(
/// **rdfs:Datatype**
/// The class of RDF datatypes.
:"http://www.w3.org/2000/01/rdf-schema#Datatype", Datatype,
14);
impl<G> ontology::properties::rdfs::SubClassOf for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SubClassOf for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<Datatype<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Datatype<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<Datatype<G>> where G: graph::Graph {}

class!(
/// **rdfs:Literal**
/// The class of literal values, eg. textual strings and integers.
:"http://www.w3.org/2000/01/rdf-schema#Literal", Literal,
15);
impl<G> ontology::properties::rdf::Type for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<Literal<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Literal<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<Literal<G>> where G: graph::Graph {}

class!(
/// **rdfs:Resource**
/// The class resource, everything.
:"http://www.w3.org/2000/01/rdf-schema#Resource", Resource,
16);
impl<G> ontology::properties::rdf::Type for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Type for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdf::Value for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Comment for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::IsDefinedBy for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Label for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::Member for resource::IRI<Resource<G>> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for Resource<G> where G: graph::Graph {}
impl<G> ontology::properties::rdfs::SeeAlso for resource::IRI<Resource<G>> where G: graph::Graph {}
