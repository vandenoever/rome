pub mod rdf;
pub mod rdfs;
use graph;
use std;
use ontology_adapter;

pub fn adapter<G>(graph: &std::rc::Rc<G>) -> ontology_adapter::OntologyAdapter<G>
    where G: graph::Graph
{
    let mut iris = Vec::with_capacity(33);
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#List"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Class"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Container"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Datatype"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Literal"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#Resource"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#first"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#object"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"));
    iris.push(graph.predicate_ptr("http://www.w3.org/1999/02/22-rdf-syntax-ns#value"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#comment"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#domain"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#isDefinedBy"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#label"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#member"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#range"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#seeAlso"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#subClassOf"));
    iris.push(graph.predicate_ptr("http://www.w3.org/2000/01/rdf-schema#subPropertyOf"));
    ontology_adapter::OntologyAdapter::new(graph, iris)
}
