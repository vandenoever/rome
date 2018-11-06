use graph;
use ontology;
use ontology::iri::owl;
use ontology_adapter;
use resource;
use std;

class!(
/// **owl:AllDifferent**
/// The class of collections of pairwise different individuals.
:owl::ALL_DIFFERENT, AllDifferent,
59);
impl<'g, G: 'g> ontology::properties::owl::DistinctMembers<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DistinctMembers<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AllDifferent<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, AllDifferent<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AllDifferent<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AllDifferent<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:AllDisjointClasses**
/// The class of collections of pairwise disjoint classes.
:owl::ALL_DISJOINT_CLASSES, AllDisjointClasses,
60);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AllDisjointClasses<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AllDisjointClasses<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AllDisjointClasses<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:AllDisjointProperties**
/// The class of collections of pairwise disjoint properties.
:owl::ALL_DISJOINT_PROPERTIES, AllDisjointProperties,
61);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AllDisjointProperties<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AllDisjointProperties<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Annotation**
/// The class of annotated annotations for which the RDF serialization consists of an annotated subject, predicate and object.
:owl::ANNOTATION, Annotation,
62);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Annotation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Annotation<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Annotation<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Annotation<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Annotation<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Annotation<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Annotation<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Annotation<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:AnnotationProperty**
/// The class of annotation properties.
:owl::ANNOTATION_PROPERTY, AnnotationProperty,
63);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AnnotationProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AnnotationProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AnnotationProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:AsymmetricProperty**
/// The class of asymmetric properties.
:owl::ASYMMETRIC_PROPERTY, AsymmetricProperty,
64);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, AsymmetricProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for AsymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, AsymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Axiom**
/// The class of annotated axioms for which the RDF serialization consists of an annotated subject, predicate and object.
:owl::AXIOM, Axiom,
65);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Axiom<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Axiom<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Axiom<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Axiom<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Axiom<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Axiom<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Axiom<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:Class**
/// The class of OWL classes.
:owl::CLASS, Class,
66);
impl<'g, G: 'g> ontology::properties::owl::ComplementOf<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::ComplementOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointUnionOf<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointUnionOf<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointWith<'g> for Class<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointWith<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasKey<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::HasKey<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
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
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Class<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Class<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Class<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:DataRange**
/// The class of OWL data ranges, which are special kinds of datatypes. Note: The use of the IRI owl:DataRange has been deprecated as of OWL 2. The IRI rdfs:Datatype SHOULD be used instead.
:owl::DATA_RANGE, DataRange,
67);
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for DataRange<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for DataRange<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DataRange<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DataRange<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DataRange<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DataRange<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DataRange<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DatatypeComplementOf<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnDatatype<'g> for resource::IRI<'g, DataRange<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g> for DataRange<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::WithRestrictions<'g>
    for resource::IRI<'g, DataRange<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:DatatypeProperty**
/// The class of data properties.
:owl::DATATYPE_PROPERTY, DatatypeProperty,
68);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, DatatypeProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DatatypeProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DatatypeProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DatatypeProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for DatatypeProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, DatatypeProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:DeprecatedClass**
/// The class of deprecated classes.
:owl::DEPRECATED_CLASS, DeprecatedClass,
69);
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, DeprecatedClass<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for DeprecatedClass<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, DeprecatedClass<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:DeprecatedProperty**
/// The class of deprecated properties.
:owl::DEPRECATED_PROPERTY, DeprecatedProperty,
70);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, DeprecatedProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for DeprecatedProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, DeprecatedProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:FunctionalProperty**
/// The class of functional properties.
:owl::FUNCTIONAL_PROPERTY, FunctionalProperty,
71);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, FunctionalProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for FunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, FunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:InverseFunctionalProperty**
/// The class of inverse-functional properties.
:owl::INVERSE_FUNCTIONAL_PROPERTY, InverseFunctionalProperty,
72);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for InverseFunctionalProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for InverseFunctionalProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for InverseFunctionalProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for InverseFunctionalProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for InverseFunctionalProperty<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, InverseFunctionalProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:IrreflexiveProperty**
/// The class of irreflexive properties.
:owl::IRREFLEXIVE_PROPERTY, IrreflexiveProperty,
73);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for IrreflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, IrreflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:NamedIndividual**
/// The class of named individuals.
:owl::NAMED_INDIVIDUAL, NamedIndividual,
74);
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g>
    for resource::IRI<'g, NamedIndividual<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g>
    for resource::IRI<'g, NamedIndividual<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g>
    for resource::IRI<'g, NamedIndividual<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, NamedIndividual<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g>
    for resource::IRI<'g, NamedIndividual<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g> for NamedIndividual<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g>
    for resource::IRI<'g, NamedIndividual<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:NegativePropertyAssertion**
/// The class of negative property assertions.
:owl::NEGATIVE_PROPERTY_ASSERTION, NegativePropertyAssertion,
75);
impl<'g, G: 'g> ontology::properties::owl::AssertionProperty<'g>
    for NegativePropertyAssertion<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AssertionProperty<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::SourceIndividual<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::SourceIndividual<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::TargetIndividual<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TargetIndividual<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::TargetValue<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TargetValue<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for NegativePropertyAssertion<'g, G>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for NegativePropertyAssertion<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, NegativePropertyAssertion<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Nothing**
/// This is the empty class.
:owl::NOTHING, Nothing,
76);
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g> for Nothing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g>
    for resource::IRI<'g, Nothing<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g> for Nothing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g>
    for resource::IRI<'g, Nothing<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for Nothing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, Nothing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for Nothing<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, Nothing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g> for Nothing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g> for resource::IRI<'g, Nothing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g> for Nothing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g>
    for resource::IRI<'g, Nothing<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:ObjectProperty**
/// The class of object properties.
:owl::OBJECT_PROPERTY, ObjectProperty,
77);
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, ObjectProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for ObjectProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, ObjectProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Ontology**
/// The class of ontologies.
:owl::ONTOLOGY, Ontology,
78);
impl<'g, G: 'g> ontology::properties::owl::BackwardCompatibleWith<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BackwardCompatibleWith<'g>
    for resource::IRI<'g, Ontology<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Imports<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Imports<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IncompatibleWith<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IncompatibleWith<'g>
    for resource::IRI<'g, Ontology<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PriorVersion<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PriorVersion<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionIRI<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionIRI<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Ontology<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Ontology<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Ontology<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Ontology<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Ontology<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Ontology<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Ontology<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Ontology<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:OntologyProperty**
/// The class of ontology properties.
:owl::ONTOLOGY_PROPERTY, OntologyProperty,
79);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, OntologyProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, OntologyProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, OntologyProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, OntologyProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for OntologyProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, OntologyProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:ReflexiveProperty**
/// The class of reflexive properties.
:owl::REFLEXIVE_PROPERTY, ReflexiveProperty,
80);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, ReflexiveProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, ReflexiveProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for ReflexiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, ReflexiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Restriction**
/// The class of property restrictions.
:owl::RESTRICTION, Restriction,
81);
impl<'g, G: 'g> ontology::properties::owl::AllValuesFrom<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AllValuesFrom<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Cardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Cardinality<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasSelf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasSelf<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasValue<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasValue<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::MaxCardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::MaxCardinality<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::MaxQualifiedCardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::MaxQualifiedCardinality<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::MinCardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::MinCardinality<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::MinQualifiedCardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::MinQualifiedCardinality<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OnClass<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnClass<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnDataRange<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnDataRange<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnProperties<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnProperties<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OnProperty<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OnProperty<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::QualifiedCardinality<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::QualifiedCardinality<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::SomeValuesFrom<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::SomeValuesFrom<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubClassOf<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentClass<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::IntersectionOf<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for Restriction<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::OneOf<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::UnionOf<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Restriction<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Restriction<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::ComplementOf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::ComplementOf<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointUnionOf<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointUnionOf<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointWith<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DisjointWith<'g>
    for resource::IRI<'g, Restriction<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::HasKey<'g> for Restriction<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::HasKey<'g> for resource::IRI<'g, Restriction<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:SymmetricProperty**
/// The class of symmetric properties.
:owl::SYMMETRIC_PROPERTY, SymmetricProperty,
82);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, SymmetricProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, SymmetricProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for SymmetricProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, SymmetricProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}

class!(
/// **owl:Thing**
/// The class of OWL individuals.
:owl::THING, Thing,
83);
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g> for Thing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomDataProperty<'g>
    for resource::IRI<'g, Thing<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g> for Thing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::BottomObjectProperty<'g>
    for resource::IRI<'g, Thing<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for Thing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::DifferentFrom<'g> for resource::IRI<'g, Thing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for Thing<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::SameAs<'g> for resource::IRI<'g, Thing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g> for Thing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopDataProperty<'g> for resource::IRI<'g, Thing<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g> for Thing<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::TopObjectProperty<'g> for resource::IRI<'g, Thing<'g, G>> where
    G: graph::Graph<'g>
{}

class!(
/// **owl:TransitiveProperty**
/// The class of transitive properties.
:owl::TRANSITIVE_PROPERTY, TransitiveProperty,
84);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, TransitiveProperty<'g, G>> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::InverseOf<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g> for TransitiveProperty<'g, G> where
    G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::PropertyChainAxiom<'g>
    for resource::IRI<'g, TransitiveProperty<'g, G>>
where
    G: graph::Graph<'g>,
{}
