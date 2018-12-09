use crate::graph;
use crate::ontology;
use crate::ontology::iri::owl;
use crate::resource;
use std;

property!(
/// **owl:allValuesFrom**
/// The property that determines the class that a universal property restriction refers to.
:owl::ALL_VALUES_FROM, AllValuesFrom, all_values_from,
ontology::classes::rdfs::Class<'g, G>,
103);

property!(
/// **owl:annotatedProperty**
/// The property that determines the predicate of an annotated axiom or annotated annotation.
:owl::ANNOTATED_PROPERTY, AnnotatedProperty, annotated_property,
ontology::classes::rdfs::Resource<'g, G>,
104);

property!(
/// **owl:annotatedSource**
/// The property that determines the subject of an annotated axiom or annotated annotation.
:owl::ANNOTATED_SOURCE, AnnotatedSource, annotated_source,
ontology::classes::rdfs::Resource<'g, G>,
105);

property!(
/// **owl:annotatedTarget**
/// The property that determines the object of an annotated axiom or annotated annotation.
:owl::ANNOTATED_TARGET, AnnotatedTarget, annotated_target,
ontology::classes::rdfs::Resource<'g, G>,
106);

property!(
/// **owl:assertionProperty**
/// The property that determines the predicate of a negative property assertion.
:owl::ASSERTION_PROPERTY, AssertionProperty, assertion_property,
ontology::classes::rdf::Property<'g, G>,
107);

property!(
/// **owl:backwardCompatibleWith**
/// The annotation property that indicates that a given ontology is backward compatible with another ontology.
:owl::BACKWARD_COMPATIBLE_WITH, BackwardCompatibleWith, backward_compatible_with,
ontology::classes::owl::Ontology<'g, G>,
108);

property!(
/// **owl:bottomDataProperty**
/// The data property that does not relate any individual to any data value.
:owl::BOTTOM_DATA_PROPERTY, BottomDataProperty, bottom_data_property,
ontology::classes::rdfs::Literal<'g, G>,
109);

property!(
/// **owl:bottomObjectProperty**
/// The object property that does not relate any two individuals.
:owl::BOTTOM_OBJECT_PROPERTY, BottomObjectProperty, bottom_object_property,
ontology::classes::owl::Thing<'g, G>,
110);

property!(
/// **owl:cardinality**
/// The property that determines the cardinality of an exact cardinality restriction.
:owl::CARDINALITY, Cardinality, cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
111);

property!(
/// **owl:complementOf**
/// The property that determines that a given class is the complement of another class.
:owl::COMPLEMENT_OF, ComplementOf, complement_of,
ontology::classes::owl::Class<'g, G>,
112);

property!(
/// **owl:datatypeComplementOf**
/// The property that determines that a given data range is the complement of another data range with respect to the data domain.
:owl::DATATYPE_COMPLEMENT_OF, DatatypeComplementOf, datatype_complement_of,
ontology::classes::rdfs::Datatype<'g, G>,
113);

property!(
/// **owl:deprecated**
/// The annotation property that indicates that a given entity has been deprecated.
:owl::DEPRECATED, Deprecated, deprecated,
ontology::classes::rdfs::Resource<'g, G>,
114);

property!(
/// **owl:differentFrom**
/// The property that determines that two given individuals are different.
:owl::DIFFERENT_FROM, DifferentFrom, different_from,
ontology::classes::owl::Thing<'g, G>,
115);

property!(
/// **owl:disjointUnionOf**
/// The property that determines that a given class is equivalent to the disjoint union of a collection of other classes.
:owl::DISJOINT_UNION_OF, DisjointUnionOf, disjoint_union_of,
ontology::classes::rdf::List<'g, G>,
116);

property!(
/// **owl:disjointWith**
/// The property that determines that two given classes are disjoint.
:owl::DISJOINT_WITH, DisjointWith, disjoint_with,
ontology::classes::owl::Class<'g, G>,
117);

property!(
/// **owl:distinctMembers**
/// The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom.
:owl::DISTINCT_MEMBERS, DistinctMembers, distinct_members,
ontology::classes::rdf::List<'g, G>,
118);

property!(
/// **owl:equivalentClass**
/// The property that determines that two given classes are equivalent, and that is used to specify datatype definitions.
:owl::EQUIVALENT_CLASS, EquivalentClass, equivalent_class,
ontology::classes::rdfs::Class<'g, G>,
119);

property!(
/// **owl:equivalentProperty**
/// The property that determines that two given properties are equivalent.
:owl::EQUIVALENT_PROPERTY, EquivalentProperty, equivalent_property,
ontology::classes::rdf::Property<'g, G>,
120);

property!(
/// **owl:hasKey**
/// The property that determines the collection of properties that jointly build a key.
:owl::HAS_KEY, HasKey, has_key,
ontology::classes::rdf::List<'g, G>,
121);

property!(
/// **owl:hasSelf**
/// The property that determines the property that a self restriction refers to.
:owl::HAS_SELF, HasSelf, has_self,
ontology::classes::rdfs::Resource<'g, G>,
122);

property!(
/// **owl:hasValue**
/// The property that determines the individual that a has-value restriction refers to.
:owl::HAS_VALUE, HasValue, has_value,
ontology::classes::rdfs::Resource<'g, G>,
123);

property!(
/// **owl:imports**
/// The property that is used for importing other ontologies into a given ontology.
:owl::IMPORTS, Imports, imports,
ontology::classes::owl::Ontology<'g, G>,
124);

property!(
/// **owl:incompatibleWith**
/// The annotation property that indicates that a given ontology is incompatible with another ontology.
:owl::INCOMPATIBLE_WITH, IncompatibleWith, incompatible_with,
ontology::classes::owl::Ontology<'g, G>,
125);

property!(
/// **owl:intersectionOf**
/// The property that determines the collection of classes or data ranges that build an intersection.
:owl::INTERSECTION_OF, IntersectionOf, intersection_of,
ontology::classes::rdf::List<'g, G>,
126);

property!(
/// **owl:inverseOf**
/// The property that determines that two given properties are inverse.
:owl::INVERSE_OF, InverseOf, inverse_of,
ontology::classes::owl::ObjectProperty<'g, G>,
127);

property!(
/// **owl:maxCardinality**
/// The property that determines the cardinality of a maximum cardinality restriction.
:owl::MAX_CARDINALITY, MaxCardinality, max_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
128);

property!(
/// **owl:maxQualifiedCardinality**
/// The property that determines the cardinality of a maximum qualified cardinality restriction.
:owl::MAX_QUALIFIED_CARDINALITY, MaxQualifiedCardinality, max_qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
129);

property!(
/// **owl:members**
/// The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom.
:owl::MEMBERS, Members, members,
ontology::classes::rdf::List<'g, G>,
130);

property!(
/// **owl:minCardinality**
/// The property that determines the cardinality of a minimum cardinality restriction.
:owl::MIN_CARDINALITY, MinCardinality, min_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
131);

property!(
/// **owl:minQualifiedCardinality**
/// The property that determines the cardinality of a minimum qualified cardinality restriction.
:owl::MIN_QUALIFIED_CARDINALITY, MinQualifiedCardinality, min_qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
132);

property!(
/// **owl:onClass**
/// The property that determines the class that a qualified object cardinality restriction refers to.
:owl::ON_CLASS, OnClass, on_class,
ontology::classes::owl::Class<'g, G>,
133);

property!(
/// **owl:onDataRange**
/// The property that determines the data range that a qualified data cardinality restriction refers to.
:owl::ON_DATA_RANGE, OnDataRange, on_data_range,
ontology::classes::rdfs::Datatype<'g, G>,
134);

property!(
/// **owl:onDatatype**
/// The property that determines the datatype that a datatype restriction refers to.
:owl::ON_DATATYPE, OnDatatype, on_datatype,
ontology::classes::rdfs::Datatype<'g, G>,
135);

property!(
/// **owl:onProperties**
/// The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to.
:owl::ON_PROPERTIES, OnProperties, on_properties,
ontology::classes::rdf::List<'g, G>,
136);

property!(
/// **owl:onProperty**
/// The property that determines the property that a property restriction refers to.
:owl::ON_PROPERTY, OnProperty, on_property,
ontology::classes::rdf::Property<'g, G>,
137);

property!(
/// **owl:oneOf**
/// The property that determines the collection of individuals or data values that build an enumeration.
:owl::ONE_OF, OneOf, one_of,
ontology::classes::rdf::List<'g, G>,
138);

property!(
/// **owl:priorVersion**
/// The annotation property that indicates the predecessor ontology of a given ontology.
:owl::PRIOR_VERSION, PriorVersion, prior_version,
ontology::classes::owl::Ontology<'g, G>,
139);

property!(
/// **owl:propertyChainAxiom**
/// The property that determines the n-tuple of properties that build a sub property chain of a given property.
:owl::PROPERTY_CHAIN_AXIOM, PropertyChainAxiom, property_chain_axiom,
ontology::classes::rdf::List<'g, G>,
140);

property!(
/// **owl:propertyDisjointWith**
/// The property that determines that two given properties are disjoint.
:owl::PROPERTY_DISJOINT_WITH, PropertyDisjointWith, property_disjoint_with,
ontology::classes::rdf::Property<'g, G>,
141);

property!(
/// **owl:qualifiedCardinality**
/// The property that determines the cardinality of an exact qualified cardinality restriction.
:owl::QUALIFIED_CARDINALITY, QualifiedCardinality, qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
142);

property!(
/// **owl:sameAs**
/// The property that determines that two given individuals are equal.
:owl::SAME_AS, SameAs, same_as,
ontology::classes::owl::Thing<'g, G>,
143);

property!(
/// **owl:someValuesFrom**
/// The property that determines the class that an existential property restriction refers to.
:owl::SOME_VALUES_FROM, SomeValuesFrom, some_values_from,
ontology::classes::rdfs::Class<'g, G>,
144);

property!(
/// **owl:sourceIndividual**
/// The property that determines the subject of a negative property assertion.
:owl::SOURCE_INDIVIDUAL, SourceIndividual, source_individual,
ontology::classes::owl::Thing<'g, G>,
145);

property!(
/// **owl:targetIndividual**
/// The property that determines the object of a negative object property assertion.
:owl::TARGET_INDIVIDUAL, TargetIndividual, target_individual,
ontology::classes::owl::Thing<'g, G>,
146);

property!(
/// **owl:targetValue**
/// The property that determines the value of a negative data property assertion.
:owl::TARGET_VALUE, TargetValue, target_value,
ontology::classes::rdfs::Literal<'g, G>,
147);

property!(
/// **owl:topDataProperty**
/// The data property that relates every individual to every data value.
:owl::TOP_DATA_PROPERTY, TopDataProperty, top_data_property,
ontology::classes::rdfs::Literal<'g, G>,
148);

property!(
/// **owl:topObjectProperty**
/// The object property that relates every two individuals.
:owl::TOP_OBJECT_PROPERTY, TopObjectProperty, top_object_property,
ontology::classes::owl::Thing<'g, G>,
149);

property!(
/// **owl:unionOf**
/// The property that determines the collection of classes or data ranges that build a union.
:owl::UNION_OF, UnionOf, union_of,
ontology::classes::rdf::List<'g, G>,
150);

property!(
/// **owl:versionIRI**
/// The property that identifies the version IRI of an ontology.
:owl::VERSION_IRI, VersionIRI, version_iri,
ontology::classes::owl::Ontology<'g, G>,
151);

property!(
/// **owl:versionInfo**
/// The annotation property that provides version information for an ontology or another OWL construct.
:owl::VERSION_INFO, VersionInfo, version_info,
ontology::classes::rdfs::Resource<'g, G>,
152);

property!(
/// **owl:withRestrictions**
/// The property that determines the collection of facet-value pairs that define a datatype restriction.
:owl::WITH_RESTRICTIONS, WithRestrictions, with_restrictions,
ontology::classes::rdf::List<'g, G>,
153);
