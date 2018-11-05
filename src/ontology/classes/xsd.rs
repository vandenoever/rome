use graph;
use ontology;
use ontology::iri::xsd;
use ontology_adapter;
use resource;
use std;

class!(
/// **xsd:ENTITY**
:xsd::ENTITY, ENTITY,
17);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ENTITY<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ENTITY<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, ENTITY<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ENTITY<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ENTITY<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ENTITY<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for ENTITY<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, ENTITY<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:ID**
:xsd::ID, ID,
18);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ID<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ID<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ID<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for ID<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, ID<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:IDREF**
:xsd::IDREF, IDREF,
19);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for IDREF<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for IDREF<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for IDREF<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for IDREF<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for IDREF<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for IDREF<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, IDREF<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:NCName**
:xsd::NCNAME, NCName,
20);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NCName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NCName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NCName<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NCName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NCName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NCName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NCName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, NCName<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:NMTOKEN**
:xsd::NMTOKEN, NMTOKEN,
21);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NMTOKEN<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NMTOKEN<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NMTOKEN<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NMTOKEN<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NMTOKEN<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NMTOKEN<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NMTOKEN<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, NMTOKEN<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:NOTATION**
:xsd::NOTATION, NOTATION,
22);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NOTATION<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NOTATION<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NOTATION<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NOTATION<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NOTATION<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NOTATION<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NOTATION<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NOTATION<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NOTATION<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NOTATION<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NOTATION<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NOTATION<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NOTATION<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NOTATION<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NOTATION<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, NOTATION<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:Name**
:xsd::NAME, Name,
23);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Name<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Name<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Name<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Name<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Name<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Name<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:QName**
:xsd::QNAME, QName,
24);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for QName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for QName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for QName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for QName<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for QName<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for QName<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, QName<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:anySimpleType**
:xsd::ANY_SIMPLE_TYPE, AnySimpleType,
25);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AnySimpleType<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AnySimpleType<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AnySimpleType<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AnySimpleType<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AnySimpleType<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AnySimpleType<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AnySimpleType<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:anyURI**
:xsd::ANY_URI, AnyURI,
26);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AnyURI<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AnyURI<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AnyURI<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AnyURI<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AnyURI<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AnyURI<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AnyURI<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, AnyURI<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:base64Binary**
:xsd::BASE64_BINARY, Base64Binary,
27);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Base64Binary<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, Base64Binary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Base64Binary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Base64Binary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Base64Binary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Base64Binary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Base64Binary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, Base64Binary<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:boolean**
:xsd::BOOLEAN, Boolean,
28);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Boolean<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Boolean<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Boolean<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Boolean<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Boolean<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Boolean<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Boolean<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Boolean<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:byte**
:xsd::BYTE, Byte,
29);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Byte<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Byte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Byte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Byte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Byte<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Byte<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:date**
:xsd::DATE, Date,
30);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Date<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Date<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Date<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Date<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Date<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Date<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:dateTime**
:xsd::DATE_TIME, DateTime,
31);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DateTime<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DateTime<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DateTime<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DateTime<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DateTime<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DateTime<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DateTime<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DateTime<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, DateTime<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DateTime<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, DateTime<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DateTime<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, DateTime<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DateTime<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for DateTime<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, DateTime<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:decimal**
:xsd::DECIMAL, Decimal,
32);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Decimal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Decimal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Decimal<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Decimal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Decimal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Decimal<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Decimal<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Decimal<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:double**
:xsd::DOUBLE, Double,
33);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Double<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Double<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Double<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Double<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Double<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Double<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Double<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Double<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:duration**
:xsd::DURATION, Duration,
34);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Duration<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Duration<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Duration<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Duration<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Duration<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Duration<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Duration<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Duration<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Duration<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Duration<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Duration<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Duration<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Duration<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Duration<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Duration<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Duration<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:float**
:xsd::FLOAT, Float,
35);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Float<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Float<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Float<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Float<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Float<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Float<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Float<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:gDay**
:xsd::G_DAY, GDay,
36);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GDay<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for GDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, GDay<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:gMonth**
:xsd::G_MONTH, GMonth,
37);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, GMonth<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for GMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, GMonth<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:gMonthDay**
:xsd::G_MONTH_DAY, GMonthDay,
38);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GMonthDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GMonthDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GMonthDay<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GMonthDay<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, GMonthDay<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, GMonthDay<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, GMonthDay<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GMonthDay<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for GMonthDay<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, GMonthDay<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:gYear**
:xsd::G_YEAR, GYear,
39);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GYear<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GYear<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GYear<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GYear<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GYear<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for GYear<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, GYear<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:gYearMonth**
:xsd::G_YEAR_MONTH, GYearMonth,
40);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for GYearMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for GYearMonth<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for GYearMonth<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, GYearMonth<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, GYearMonth<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, GYearMonth<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for GYearMonth<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, GYearMonth<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:hexBinary**
:xsd::HEX_BINARY, HexBinary,
41);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for HexBinary<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for HexBinary<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for HexBinary<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for HexBinary<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, HexBinary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, HexBinary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, HexBinary<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for HexBinary<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for HexBinary<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, HexBinary<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:int**
:xsd::INT, Int,
42);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Int<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Int<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Int<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Int<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Int<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:integer**
:xsd::INTEGER, Integer,
43);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Integer<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Integer<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Integer<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Integer<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Integer<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Integer<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Integer<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Integer<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:language**
:xsd::LANGUAGE, Language,
44);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Language<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Language<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Language<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Language<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Language<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Language<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Language<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Language<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Language<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Language<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Language<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Language<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Language<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Language<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Language<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Language<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:long**
:xsd::LONG, Long,
45);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Long<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Long<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Long<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Long<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Long<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Long<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:negativeInteger**
:xsd::NEGATIVE_INTEGER, NegativeInteger,
46);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, NegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, NegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, NegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:nonNegativeInteger**
:xsd::NON_NEGATIVE_INTEGER, NonNegativeInteger,
47);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NonNegativeInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NonNegativeInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, NonNegativeInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:nonPositiveInteger**
:xsd::NON_POSITIVE_INTEGER, NonPositiveInteger,
48);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NonPositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NonPositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, NonPositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:normalizedString**
:xsd::NORMALIZED_STRING, NormalizedString,
49);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, NormalizedString<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, NormalizedString<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, NormalizedString<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NormalizedString<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, NormalizedString<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:positiveInteger**
:xsd::POSITIVE_INTEGER, PositiveInteger,
50);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PositiveInteger<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for PositiveInteger<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, PositiveInteger<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:short**
:xsd::SHORT, Short,
51);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Short<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Short<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Short<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Short<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Short<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Short<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Short<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:string**
:xsd::STRING, String,
52);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for String<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for String<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, String<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for String<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for String<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for String<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for String<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, String<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:time**
:xsd::TIME, Time,
53);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Time<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Time<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Time<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Time<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Time<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Time<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:token**
:xsd::TOKEN, Token,
54);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Token<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Token<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Token<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Token<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Token<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Token<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Token<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:unsignedByte**
:xsd::UNSIGNED_BYTE, UnsignedByte,
55);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UnsignedByte<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, UnsignedByte<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, UnsignedByte<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, UnsignedByte<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, UnsignedByte<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UnsignedByte<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for UnsignedByte<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, UnsignedByte<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:unsignedInt**
:xsd::UNSIGNED_INT, UnsignedInt,
56);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UnsignedInt<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, UnsignedInt<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, UnsignedInt<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, UnsignedInt<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, UnsignedInt<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for UnsignedInt<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, UnsignedInt<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **xsd:unsignedLong**
:xsd::UNSIGNED_LONG, UnsignedLong,
57);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UnsignedLong<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, UnsignedLong<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, UnsignedLong<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, UnsignedLong<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, UnsignedLong<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UnsignedLong<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for UnsignedLong<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, UnsignedLong<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **xsd:unsignedShort**
:xsd::UNSIGNED_SHORT, UnsignedShort,
58);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, UnsignedShort<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, UnsignedShort<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, UnsignedShort<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, UnsignedShort<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, UnsignedShort<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for UnsignedShort<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, UnsignedShort<'g, G>>
where
    G: graph::Graph<'g>,
{}
