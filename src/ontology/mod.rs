/// Ontology classes
pub mod classes;
/// Ontology properties
pub mod properties;
use graph;
use ontology_adapter;
/// Adapter to access RDF data in graph via the ontology
pub fn adapter<'g, G>(graph: &'g G) -> ontology_adapter::OntologyAdapter<'g, G>
where
    G: graph::Graph<'g>,
{
    let mut iris = Vec::with_capacity(33);
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#List"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#Class"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#Container"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#Datatype"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#Literal"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#Resource"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#first"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    iris.push(graph.find_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#value"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#comment"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#domain"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#isDefinedBy"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#label"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#member"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#range"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#seeAlso"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#subClassOf"));
    iris.push(graph.find_iri("http://www.w3.org/2000/01/rdf-schema#subPropertyOf"));
    ontology_adapter::OntologyAdapter::new(graph, iris)
}
