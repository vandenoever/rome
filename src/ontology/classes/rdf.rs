use crate::graph;
use crate::ontology;
use crate::ontology::iri::rdf;
use crate::ontology_adapter;
use crate::resource;

class!(
/// **rdf:Alt**
/// The class of containers of alternatives.
:rdf::ALT, Alt,
1);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Alt<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Alt<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Alt<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Alt<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Alt<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:Bag**
/// The class of unordered containers.
:rdf::BAG, Bag,
2);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Bag<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Bag<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Bag<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Bag<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Bag<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:HTML**
/// The datatype of RDF literals storing fragments of HTML content
:rdf::HTML, HTML,
3);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for HTML<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for HTML<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for HTML<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for HTML<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for HTML<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, HTML<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:List**
/// The class of RDF Lists.
:rdf::LIST, List,
4);
impl<'g, G: 'g> ontology::properties::rdf::First<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::First<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Rest<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Rest<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for List<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for List<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for List<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for List<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for List<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, List<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:PlainLiteral**
/// The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
:rdf::PLAIN_LITERAL, PlainLiteral,
5);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for PlainLiteral<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g>
    for resource::IRI<'g, PlainLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, PlainLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, PlainLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, PlainLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, PlainLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for PlainLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g>
    for resource::IRI<'g, PlainLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}

class!(
/// **rdf:Property**
/// The class of RDF properties.
:rdf::PROPERTY, Property,
6);
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Domain<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Range<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SubPropertyOf<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::EquivalentProperty<'g>
    for resource::IRI<'g, Property<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::PropertyDisjointWith<'g>
    for resource::IRI<'g, Property<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Property<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Property<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Property<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Property<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Property<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Property<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Property<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Property<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:Seq**
/// The class of ordered containers.
:rdf::SEQ, Seq,
7);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Seq<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Seq<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Seq<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Seq<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Seq<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:Statement**
/// The class of RDF statements.
:rdf::STATEMENT, Statement,
8);
impl<'g, G: 'g> ontology::properties::rdf::Object<'g> for Statement<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Object<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Predicate<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Predicate<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Subject<'g> for Statement<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdf::Subject<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for Statement<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for Statement<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for Statement<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for Statement<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, Statement<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, Statement<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, Statement<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for Statement<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for Statement<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, Statement<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:XMLLiteral**
/// The datatype of XML literal values.
:rdf::XMLLITERAL, XMLLiteral,
9);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for XMLLiteral<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for XMLLiteral<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for XMLLiteral<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, XMLLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, XMLLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, XMLLiteral<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for XMLLiteral<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, XMLLiteral<'g, G>> where
    G: graph::Graph<'g>
{
}

class!(
/// **rdf:langString**
/// The datatype of language-tagged string values
:rdf::LANG_STRING, LangString,
10);
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for LangString<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Type<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for LangString<'g, G> where G: graph::Graph<'g> {}
impl<'g, G: 'g> ontology::properties::rdf::Value<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Comment<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::IsDefinedBy<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for LangString<'g, G> where G: graph::Graph<'g>
{}
impl<'g, G: 'g> ontology::properties::rdfs::Label<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::Member<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::rdfs::SeeAlso<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedProperty<'g>
    for resource::IRI<'g, LangString<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedSource<'g>
    for resource::IRI<'g, LangString<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::AnnotatedTarget<'g>
    for resource::IRI<'g, LangString<'g, G>>
where
    G: graph::Graph<'g>,
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Deprecated<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::Members<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for LangString<'g, G> where
    G: graph::Graph<'g>
{
}
impl<'g, G: 'g> ontology::properties::owl::VersionInfo<'g> for resource::IRI<'g, LangString<'g, G>> where
    G: graph::Graph<'g>
{
}
