use crate::graph;
use crate::ontology;
use crate::ontology::iri::rdfs;
use crate::resource;
use std;

property!(
/// **rdfs:comment**
/// A description of the subject resource.
:rdfs::COMMENT, Comment, comment,
ontology::classes::rdfs::Literal<'g, G>,
94);

property!(
/// **rdfs:domain**
/// A domain of the subject property.
:rdfs::DOMAIN, Domain, domain,
ontology::classes::rdfs::Class<'g, G>,
95);

property!(
/// **rdfs:isDefinedBy**
/// The defininition of the subject resource.
:rdfs::IS_DEFINED_BY, IsDefinedBy, is_defined_by,
ontology::classes::rdfs::Resource<'g, G>,
96);

property!(
/// **rdfs:label**
/// A human-readable name for the subject.
:rdfs::LABEL, Label, label,
ontology::classes::rdfs::Literal<'g, G>,
97);

property!(
/// **rdfs:member**
/// A member of the subject resource.
:rdfs::MEMBER, Member, member,
ontology::classes::rdfs::Resource<'g, G>,
98);

property!(
/// **rdfs:range**
/// A range of the subject property.
:rdfs::RANGE, Range, range,
ontology::classes::rdfs::Class<'g, G>,
99);

property!(
/// **rdfs:seeAlso**
/// Further information about the subject resource.
:rdfs::SEE_ALSO, SeeAlso, see_also,
ontology::classes::rdfs::Resource<'g, G>,
100);

property!(
/// **rdfs:subClassOf**
/// The subject is a subclass of a class.
:rdfs::SUB_CLASS_OF, SubClassOf, sub_class_of,
ontology::classes::rdfs::Class<'g, G>,
101);

property!(
/// **rdfs:subPropertyOf**
/// The subject is a subproperty of a property.
:rdfs::SUB_PROPERTY_OF, SubPropertyOf, sub_property_of,
ontology::classes::rdf::Property<'g, G>,
102);
