use graph;
use ontology;
use resource;
use std;

property!(
/// **owl:allValuesFrom**
/// The property that determines the class that a universal property restriction refers to.
:"http://www.w3.org/2002/07/owl#allValuesFrom", AllValuesFrom, all_values_from,
ontology::classes::rdfs::Class<'g, G>,
101);

property!(
/// **owl:annotatedProperty**
/// The property that determines the predicate of an annotated axiom or annotated annotation.
:"http://www.w3.org/2002/07/owl#annotatedProperty", AnnotatedProperty, annotated_property,
ontology::classes::rdfs::Resource<'g, G>,
102);

property!(
/// **owl:annotatedSource**
/// The property that determines the subject of an annotated axiom or annotated annotation.
:"http://www.w3.org/2002/07/owl#annotatedSource", AnnotatedSource, annotated_source,
ontology::classes::rdfs::Resource<'g, G>,
103);

property!(
/// **owl:annotatedTarget**
/// The property that determines the object of an annotated axiom or annotated annotation.
:"http://www.w3.org/2002/07/owl#annotatedTarget", AnnotatedTarget, annotated_target,
ontology::classes::rdfs::Resource<'g, G>,
104);

property!(
/// **owl:assertionProperty**
/// The property that determines the predicate of a negative property assertion.
:"http://www.w3.org/2002/07/owl#assertionProperty", AssertionProperty, assertion_property,
ontology::classes::rdf::Property<'g, G>,
105);

property!(
/// **owl:cardinality**
/// The property that determines the cardinality of an exact cardinality restriction.
:"http://www.w3.org/2002/07/owl#cardinality", Cardinality, cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
106);

property!(
/// **owl:complementOf**
/// The property that determines that a given class is the complement of another class.
:"http://www.w3.org/2002/07/owl#complementOf", ComplementOf, complement_of,
ontology::classes::owl::Class<'g, G>,
107);

property!(
/// **owl:datatypeComplementOf**
/// The property that determines that a given data range is the complement of another data range with respect to the data domain.
:"http://www.w3.org/2002/07/owl#datatypeComplementOf", DatatypeComplementOf, datatype_complement_of,
ontology::classes::rdfs::Datatype<'g, G>,
108);

property!(
/// **owl:differentFrom**
/// The property that determines that two given individuals are different.
:"http://www.w3.org/2002/07/owl#differentFrom", DifferentFrom, different_from,
ontology::classes::owl::Thing<'g, G>,
109);

property!(
/// **owl:disjointUnionOf**
/// The property that determines that a given class is equivalent to the disjoint union of a collection of other classes.
:"http://www.w3.org/2002/07/owl#disjointUnionOf", DisjointUnionOf, disjoint_union_of,
ontology::classes::rdf::List<'g, G>,
110);

property!(
/// **owl:disjointWith**
/// The property that determines that two given classes are disjoint.
:"http://www.w3.org/2002/07/owl#disjointWith", DisjointWith, disjoint_with,
ontology::classes::owl::Class<'g, G>,
111);

property!(
/// **owl:distinctMembers**
/// The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom.
:"http://www.w3.org/2002/07/owl#distinctMembers", DistinctMembers, distinct_members,
ontology::classes::rdf::List<'g, G>,
112);

property!(
/// **owl:equivalentClass**
/// The property that determines that two given classes are equivalent, and that is used to specify datatype definitions.
:"http://www.w3.org/2002/07/owl#equivalentClass", EquivalentClass, equivalent_class,
ontology::classes::rdfs::Class<'g, G>,
113);

property!(
/// **owl:equivalentProperty**
/// The property that determines that two given properties are equivalent.
:"http://www.w3.org/2002/07/owl#equivalentProperty", EquivalentProperty, equivalent_property,
ontology::classes::rdf::Property<'g, G>,
114);

property!(
/// **owl:hasKey**
/// The property that determines the collection of properties that jointly build a key.
:"http://www.w3.org/2002/07/owl#hasKey", HasKey, has_key,
ontology::classes::rdf::List<'g, G>,
115);

property!(
/// **owl:hasSelf**
/// The property that determines the property that a self restriction refers to.
:"http://www.w3.org/2002/07/owl#hasSelf", HasSelf, has_self,
ontology::classes::rdfs::Resource<'g, G>,
116);

property!(
/// **owl:hasValue**
/// The property that determines the individual that a has-value restriction refers to.
:"http://www.w3.org/2002/07/owl#hasValue", HasValue, has_value,
ontology::classes::rdfs::Resource<'g, G>,
117);

property!(
/// **owl:intersectionOf**
/// The property that determines the collection of classes or data ranges that build an intersection.
:"http://www.w3.org/2002/07/owl#intersectionOf", IntersectionOf, intersection_of,
ontology::classes::rdf::List<'g, G>,
118);

property!(
/// **owl:inverseOf**
/// The property that determines that two given properties are inverse.
:"http://www.w3.org/2002/07/owl#inverseOf", InverseOf, inverse_of,
ontology::classes::owl::ObjectProperty<'g, G>,
119);

property!(
/// **owl:maxCardinality**
/// The property that determines the cardinality of a maximum cardinality restriction.
:"http://www.w3.org/2002/07/owl#maxCardinality", MaxCardinality, max_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
120);

property!(
/// **owl:maxQualifiedCardinality**
/// The property that determines the cardinality of a maximum qualified cardinality restriction.
:"http://www.w3.org/2002/07/owl#maxQualifiedCardinality", MaxQualifiedCardinality, max_qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
121);

property!(
/// **owl:members**
/// The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom.
:"http://www.w3.org/2002/07/owl#members", Members, members,
ontology::classes::rdf::List<'g, G>,
122);

property!(
/// **owl:minCardinality**
/// The property that determines the cardinality of a minimum cardinality restriction.
:"http://www.w3.org/2002/07/owl#minCardinality", MinCardinality, min_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
123);

property!(
/// **owl:minQualifiedCardinality**
/// The property that determines the cardinality of a minimum qualified cardinality restriction.
:"http://www.w3.org/2002/07/owl#minQualifiedCardinality", MinQualifiedCardinality, min_qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
124);

property!(
/// **owl:onClass**
/// The property that determines the class that a qualified object cardinality restriction refers to.
:"http://www.w3.org/2002/07/owl#onClass", OnClass, on_class,
ontology::classes::owl::Class<'g, G>,
125);

property!(
/// **owl:onDataRange**
/// The property that determines the data range that a qualified data cardinality restriction refers to.
:"http://www.w3.org/2002/07/owl#onDataRange", OnDataRange, on_data_range,
ontology::classes::rdfs::Datatype<'g, G>,
126);

property!(
/// **owl:onDatatype**
/// The property that determines the datatype that a datatype restriction refers to.
:"http://www.w3.org/2002/07/owl#onDatatype", OnDatatype, on_datatype,
ontology::classes::rdfs::Datatype<'g, G>,
127);

property!(
/// **owl:onProperties**
/// The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to.
:"http://www.w3.org/2002/07/owl#onProperties", OnProperties, on_properties,
ontology::classes::rdf::List<'g, G>,
128);

property!(
/// **owl:onProperty**
/// The property that determines the property that a property restriction refers to.
:"http://www.w3.org/2002/07/owl#onProperty", OnProperty, on_property,
ontology::classes::rdf::Property<'g, G>,
129);

property!(
/// **owl:oneOf**
/// The property that determines the collection of individuals or data values that build an enumeration.
:"http://www.w3.org/2002/07/owl#oneOf", OneOf, one_of,
ontology::classes::rdf::List<'g, G>,
130);

property!(
/// **owl:propertyChainAxiom**
/// The property that determines the n-tuple of properties that build a sub property chain of a given property.
:"http://www.w3.org/2002/07/owl#propertyChainAxiom", PropertyChainAxiom, property_chain_axiom,
ontology::classes::rdf::List<'g, G>,
131);

property!(
/// **owl:propertyDisjointWith**
/// The property that determines that two given properties are disjoint.
:"http://www.w3.org/2002/07/owl#propertyDisjointWith", PropertyDisjointWith, property_disjoint_with,
ontology::classes::rdf::Property<'g, G>,
132);

property!(
/// **owl:qualifiedCardinality**
/// The property that determines the cardinality of an exact qualified cardinality restriction.
:"http://www.w3.org/2002/07/owl#qualifiedCardinality", QualifiedCardinality, qualified_cardinality,
ontology::classes::xsd::NonNegativeInteger<'g, G>,
133);

property!(
/// **owl:sameAs**
/// The property that determines that two given individuals are equal.
:"http://www.w3.org/2002/07/owl#sameAs", SameAs, same_as,
ontology::classes::owl::Thing<'g, G>,
134);

property!(
/// **owl:someValuesFrom**
/// The property that determines the class that an existential property restriction refers to.
:"http://www.w3.org/2002/07/owl#someValuesFrom", SomeValuesFrom, some_values_from,
ontology::classes::rdfs::Class<'g, G>,
135);

property!(
/// **owl:sourceIndividual**
/// The property that determines the subject of a negative property assertion.
:"http://www.w3.org/2002/07/owl#sourceIndividual", SourceIndividual, source_individual,
ontology::classes::owl::Thing<'g, G>,
136);

property!(
/// **owl:targetIndividual**
/// The property that determines the object of a negative object property assertion.
:"http://www.w3.org/2002/07/owl#targetIndividual", TargetIndividual, target_individual,
ontology::classes::owl::Thing<'g, G>,
137);

property!(
/// **owl:targetValue**
/// The property that determines the value of a negative data property assertion.
:"http://www.w3.org/2002/07/owl#targetValue", TargetValue, target_value,
ontology::classes::rdfs::Literal<'g, G>,
138);

property!(
/// **owl:unionOf**
/// The property that determines the collection of classes or data ranges that build a union.
:"http://www.w3.org/2002/07/owl#unionOf", UnionOf, union_of,
ontology::classes::rdf::List<'g, G>,
139);

property!(
/// **owl:withRestrictions**
/// The property that determines the collection of facet-value pairs that define a datatype restriction.
:"http://www.w3.org/2002/07/owl#withRestrictions", WithRestrictions, with_restrictions,
ontology::classes::rdf::List<'g, G>,
140);
