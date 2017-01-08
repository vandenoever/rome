use std;
use graph;
use resource;
use ontology;

property!(
/// **rdfs:comment**
/// A description of the subject resource.
:"http://www.w3.org/2000/01/rdf-schema#comment", Comment, comment,
ontology::classes::rdfs::Literal<'g, G>,
24);

property!(
/// **rdfs:domain**
/// A domain of the subject property.
:"http://www.w3.org/2000/01/rdf-schema#domain", Domain, domain,
ontology::classes::rdfs::Class<'g, G>,
25);

property!(
/// **rdfs:isDefinedBy**
/// The defininition of the subject resource.
:"http://www.w3.org/2000/01/rdf-schema#isDefinedBy", IsDefinedBy, is_defined_by,
ontology::classes::rdfs::Resource<'g, G>,
26);

property!(
/// **rdfs:label**
/// A human-readable name for the subject.
:"http://www.w3.org/2000/01/rdf-schema#label", Label, label,
ontology::classes::rdfs::Literal<'g, G>,
27);

property!(
/// **rdfs:member**
/// A member of the subject resource.
:"http://www.w3.org/2000/01/rdf-schema#member", Member, member,
ontology::classes::rdfs::Resource<'g, G>,
28);

property!(
/// **rdfs:range**
/// A range of the subject property.
:"http://www.w3.org/2000/01/rdf-schema#range", Range, range,
ontology::classes::rdfs::Class<'g, G>,
29);

property!(
/// **rdfs:seeAlso**
/// Further information about the subject resource.
:"http://www.w3.org/2000/01/rdf-schema#seeAlso", SeeAlso, see_also,
ontology::classes::rdfs::Resource<'g, G>,
30);

property!(
/// **rdfs:subClassOf**
/// The subject is a subclass of a class.
:"http://www.w3.org/2000/01/rdf-schema#subClassOf", SubClassOf, sub_class_of,
ontology::classes::rdfs::Class<'g, G>,
31);

property!(
/// **rdfs:subPropertyOf**
/// The subject is a subproperty of a property.
:"http://www.w3.org/2000/01/rdf-schema#subPropertyOf", SubPropertyOf, sub_property_of,
ontology::classes::rdf::Property<'g, G>,
32);
