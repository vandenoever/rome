use graph;
use ontology;
use ontology_adapter;
use resource;
use std;

class!(
/// **rdfs:Class**
/// The class of classes.
:"http://www.w3.org/2000/01/rdf-schema#Class", Class,
11);
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Class<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **rdfs:Container**
/// The class of RDF containers.
:"http://www.w3.org/2000/01/rdf-schema#Container", Container,
12);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Container<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Container<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Container<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Container<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Container<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Container<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Container<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Container<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Container<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Container<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **rdfs:ContainerMembershipProperty**
/// The class of container membership properties, rdf:_1, rdf:_2, ...,                    all of which are sub-properties of 'member'.
:"http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty", ContainerMembershipProperty,
13);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for ContainerMembershipProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for ContainerMembershipProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for ContainerMembershipProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for ContainerMembershipProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for ContainerMembershipProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ContainerMembershipProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, ContainerMembershipProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **rdfs:Datatype**
/// The class of RDF datatypes.
:"http://www.w3.org/2000/01/rdf-schema#Datatype", Datatype,
14);
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Datatype<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Datatype<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Datatype<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Datatype<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Datatype<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Datatype<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **rdfs:Literal**
/// The class of literal values, eg. textual strings and integers.
:"http://www.w3.org/2000/01/rdf-schema#Literal", Literal,
15);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Literal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Literal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Literal<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Literal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Literal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Literal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Literal<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **rdfs:Resource**
/// The class resource, everything.
:"http://www.w3.org/2000/01/rdf-schema#Resource", Resource,
16);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Resource<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Resource<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Resource<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Resource<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Resource<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Resource<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Resource<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Resource<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Resource<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Resource<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Resource<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Resource<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Resource<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Resource<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Resource<'g, G>> where
    G: graph::Graph<'g>
{}
