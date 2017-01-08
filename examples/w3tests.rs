extern crate rdfio;
extern crate time;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use rdfio::graph;
use rdfio::graph::{Graph, GraphCreator, Triple};
use rdfio::graphs::tel;
use rdfio::io::{TurtleParser, write_turtle};
use rdfio::namespaces::Namespaces;

type MyGraph = tel::Graph64;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

const RDFS_COMMENT: &'static str = "http://www.w3.org/2000/01/rdf-schema#comment";
const MF_ACTION: &'static str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#action";
const MF_NAME: &'static str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#name";
const MF_RESULT: &'static str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#result";
const RDFT_APPROVAL: &'static str = "http://www.w3.org/ns/rdftest#approval";
const RDFT_APPROVED: &'static str = "http://www.w3.org/ns/rdftest#Approved";
const RDFT_PROPOSED: &'static str = "http://www.w3.org/ns/rdftest#Proposed";
const RDFT_REJECTED: &'static str = "http://www.w3.org/ns/rdftest#Rejected";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

const DC_DATE: &'static str = "http://purl.org/dc/elements/1.1/date";
const XSD_DATE_TIME: &'static str = "http://www.w3.org/2001/XMLSchema#dateTime";
const EARL_ASSERTION: &'static str = "http://www.w3.org/ns/earl#Assertion";
const EARL_RESULT: &'static str = "http://www.w3.org/ns/earl#result";
const EARL_TEST_RESULT: &'static str = "http://www.w3.org/ns/earl#TestResult";
const EARL_TEST: &'static str = "http://www.w3.org/ns/earl#test";
const EARL_OUTCOME: &'static str = "http://www.w3.org/ns/earl#outcome";
const EARL_PASSED: &'static str = "http://www.w3.org/ns/earl#passed";
const EARL_FAILED: &'static str = "http://www.w3.org/ns/earl#failed";
const EARL_CANT_TELL: &'static str = "http://www.w3.org/ns/earl#cantTell";

#[derive (Debug)]
struct TestTurtleEval {
    id: Rc<String>,
    name: String,
    comment: String,
    approval: Approval,
    action: String,
    result: String,
}

#[derive (Debug)]
struct TestTurtlePositiveSyntax {
    id: Rc<String>,
    name: String,
    comment: String,
    approval: Approval,
    action: String,
}

#[derive (Debug)]
struct TestTurtleNegativeSyntax {
    id: Rc<String>,
    name: String,
    comment: String,
    approval: Approval,
    action: String,
}

#[derive (Debug)]
struct TestTurtleNegativeEval {
    id: Rc<String>,
    name: String,
    comment: String,
    approval: Approval,
    action: String,
}

/// run all w3 RDF 1.1 Turtle tests from https://www.w3.org/TR/rdf11-testcases/
#[derive (Debug)]
enum Approval {
    Approved,
    Proposed,
    Rejected,
}

// prefix earl: <http://www.w3.org/ns/earl#>
#[derive (PartialEq,Debug)]
enum Outcome {
    Passed,
    Failed,
    CannotTell, /* NotApplicable,
                 * NotTested, */
}

#[derive (Debug)]
struct Assertion {
    test: Rc<String>,
    date: time::Tm,
    input_file: String,
    result: TestResult,
}

#[derive (Debug)]
struct TestResult {
    outcome: Outcome,
    date: String,
    info: String,
}

fn write_assertion<'a, 'g, W>(assertion: &'a Assertion, output: &mut W) -> rdfio::Result<()>
    where W: GraphCreator<'g>
{
    let assertion_blank_node = output.create_blank_node();
    output.add(assertion_blank_node, RDF_TYPE, EARL_ASSERTION);
    let date = format!("{}", assertion.date.rfc3339());
    output.add(assertion_blank_node,
               DC_DATE,
               graph::Literal {
                   lexical: date.as_str(),
                   datatype: XSD_DATE_TIME,
                   language: None,
               });
    let result_blank_node = output.create_blank_node();
    output.add(result_blank_node, RDF_TYPE, EARL_TEST_RESULT);
    output.add(assertion_blank_node, EARL_RESULT, result_blank_node);
    output.add(assertion_blank_node, EARL_RESULT, result_blank_node);
    let outcome = match assertion.result.outcome {
        Outcome::Passed => EARL_PASSED,
        Outcome::Failed => EARL_FAILED,
        Outcome::CannotTell => EARL_CANT_TELL,
    };
    output.add(result_blank_node, EARL_OUTCOME, outcome);
    output.add(assertion_blank_node, EARL_TEST, assertion.test.as_str());
    Ok(())
}

fn read_file(path: &str) -> io::Result<String> {
    let mut f = match fs::File::open(path) {
        Err(e) => {
            println_stderr!("Cannot open file {}.", path);
            return Err(e);
        }
        Ok(f) => f,
    };
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn load_graph(data: &str, base: &str) -> rdfio::Result<MyGraph> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    let mut triples = try!(TurtleParser::new(data, base));
    while let Some(triple) = triples.next() {
        writer.add_triple(&try!(triple));
    }
    Ok(writer.collect().sort_blank_nodes())
}

fn read<'g, T, F, R>(mut last: Option<T>,
                     i: &mut Iterator<Item = T>,
                     predicate: &str,
                     convert: F)
                     -> Result<(R, Option<T>), String>
    where T: graph::Triple<'g>,
          F: FnOnce(&graph::Object) -> Result<R, String>
{
    last = last.or_else(|| i.next());
    while let Some(triple) = last {
        if triple.predicate() == predicate {
            return Ok((try!(convert(&triple.object())), None));
        }
        last = i.next();
    }
    Err(format!("Cannot find {}.", predicate))
}

fn to_string(object: &graph::Object) -> Result<String, String> {
    match *object {
        graph::Object::IRI(iri) => Ok(String::from(iri)),
        graph::Object::Literal(ref l) => Ok(String::from(l.lexical)),
        _ => Err(String::from("object is not an iri or literal")),
    }
}

fn to_approval(object: &graph::Object) -> Result<Approval, String> {
    match *object {
        graph::Object::IRI(iri) if iri == RDFT_APPROVED => Ok(Approval::Approved),
        graph::Object::IRI(iri) if iri == RDFT_PROPOSED => Ok(Approval::Proposed),
        graph::Object::IRI(iri) if iri == RDFT_REJECTED => Ok(Approval::Rejected),
        _ => Err(String::from("object is not the right value for approval")),
    }
}

fn load_test_turtle_eval(graph: &MyGraph, subject: &Rc<String>) -> Result<TestTurtleEval, String> {
    let mut i = graph.iter_subject(&graph::Subject::IRI(subject.as_str()));
    let (comment, prev) = try!(read(None, &mut i, RDFS_COMMENT, to_string));
    let (action, prev) = try!(read(prev, &mut i, MF_ACTION, to_string));
    let (name, prev) = try!(read(prev, &mut i, MF_NAME, to_string));
    let (result, prev) = try!(read(prev, &mut i, MF_RESULT, to_string));
    let (approval, _) = try!(read(prev, &mut i, RDFT_APPROVAL, to_approval));
    Ok(TestTurtleEval {
        id: subject.clone(),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
        result: result,
    })
}
fn load_positive_syntax(graph: &MyGraph,
                        subject: &Rc<String>)
                        -> Result<TestTurtlePositiveSyntax, String> {
    let mut i = graph.iter_subject(&graph::Subject::IRI(subject.as_str()));
    let (comment, prev) = try!(read(None, &mut i, RDFS_COMMENT, to_string));
    let (action, prev) = try!(read(prev, &mut i, MF_ACTION, to_string));
    let (name, prev) = try!(read(prev, &mut i, MF_NAME, to_string));
    let (approval, _) = try!(read(prev, &mut i, RDFT_APPROVAL, to_approval));
    Ok(TestTurtlePositiveSyntax {
        id: subject.clone(),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
    })
}
fn load_negative_syntax(graph: &MyGraph,
                        subject: &Rc<String>)
                        -> Result<TestTurtleNegativeSyntax, String> {
    let mut i = graph.iter_subject(&graph::Subject::IRI(subject.as_str()));
    let (comment, prev) = try!(read(None, &mut i, RDFS_COMMENT, to_string));
    let (action, prev) = try!(read(prev, &mut i, MF_ACTION, to_string));
    let (name, prev) = try!(read(prev, &mut i, MF_NAME, to_string));
    let (approval, _) = try!(read(prev, &mut i, RDFT_APPROVAL, to_approval));
    Ok(TestTurtleNegativeSyntax {
        id: subject.clone(),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
    })
}
fn load_negative_eval(graph: &MyGraph,
                      subject: &Rc<String>)
                      -> Result<TestTurtleNegativeEval, String> {
    let mut i = graph.iter_subject(&graph::Subject::IRI(subject.as_str()));
    let (comment, prev) = try!(read(None, &mut i, RDFS_COMMENT, to_string));
    let (action, prev) = try!(read(prev, &mut i, MF_ACTION, to_string));
    let (name, prev) = try!(read(prev, &mut i, MF_NAME, to_string));
    let (approval, _) = try!(read(prev, &mut i, RDFT_APPROVAL, to_approval));
    Ok(TestTurtleNegativeEval {
        id: subject.clone(),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
    })
}

fn eval_result(r: &Assertion) {
    if r.result.outcome != Outcome::Passed {
        println_stderr!("{:?}", r.input_file);
        println_stderr!("{:?}", r.result.outcome);
        println_stderr!("{}", r.result.info);
        println_stderr!("");
    }
}

fn subject_to_string<'g, T>(triple: &T) -> Rc<String>
    where T: graph::Triple<'g>
{
    match triple.subject() {
        graph::Subject::IRI(iri) => Rc::new(String::from(iri)),
        _ => {
            panic!("a blank node as subject is not expected");
        }
    }
}

fn run_tests<'a>(graph: &'a MyGraph, base: &str, base_dir: &str) -> rdfio::Result<Vec<Assertion>> {
    let mut assertions = Vec::new();
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleEval", RDF_TYPE) {
        let s = subject_to_string(&t);
        let test = try!(load_test_turtle_eval(&graph, &s));
        let r = try!(run_eval(&test, base, base_dir));
        eval_result(&r);
        assertions.push(r);
    }
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtlePositiveSyntax",
                                        RDF_TYPE) {
        let s = subject_to_string(&t);
        let test = try!(load_positive_syntax(&graph, &s));
        let r = try!(run_eval_positive_syntax(&test, base, base_dir));
        eval_result(&r);
        assertions.push(r);
    }
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleNegativeSyntax",
                                        RDF_TYPE) {
        let s = subject_to_string(&t);
        let test = try!(load_negative_syntax(&graph, &s));
        let r = try!(run_eval_negative_syntax(&test, base, base_dir));
        eval_result(&r);
        assertions.push(r);
    }
    for t in graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleNegativeEval",
                                   RDF_TYPE) {
        let s = subject_to_string(&t);
        let test = try!(load_negative_eval(&graph, &s));
        let r = try!(run_eval_negative_eval(&test, base, base_dir));
        eval_result(&r);
        assertions.push(r);
    }
    Ok(assertions)
}

fn change_base(iri: &str, old_base: &str, new_base: &str) -> String {
    let mut n = String::from(new_base);
    n.push_str(&iri[(old_base.rfind("/").unwrap() + 1)..]);
    n
}

fn fail(test: &Rc<String>, input_file: &String, info: String) -> rdfio::Result<Assertion> {
    Ok(Assertion {
        test: test.clone(),
        input_file: input_file.clone(),
        result: TestResult {
            outcome: Outcome::Failed,
            date: String::new(),
            info: info,
        },
        date: time::now_utc(),
    })
}

fn pass(test: &Rc<String>, input_file: &String) -> rdfio::Result<Assertion> {
    Ok(Assertion {
        test: test.clone(),
        input_file: input_file.clone(),
        result: TestResult {
            outcome: Outcome::Passed,
            date: String::new(),
            info: String::new(),
        },
        date: time::now_utc(),
    })
}

fn run_eval(test: &TestTurtleEval, base: &str, base_dir: &str) -> rdfio::Result<Assertion> {
    let ttl_path = change_base(test.action.as_str(), base, base_dir);
    let nt_path = change_base(test.result.as_str(), base, base_dir);
    let ttl = try!(read_file(&ttl_path));
    let nt = try!(read_file(&nt_path));
    let nt_graph = match load_graph(nt.as_str(), test.result.as_str()) {
        Ok(graph) => graph,
        Err(err) => {
            return Ok(Assertion {
                test: test.id.clone(),
                input_file: ttl_path.clone(),
                result: TestResult {
                    outcome: Outcome::CannotTell,
                    date: String::new(),
                    info: format!("parsing of result failed {}", err),
                },
                date: time::now_utc(),
            });
        }
    };
    let ttl_graph = match load_graph(ttl.as_str(), test.action.as_str()) {
        Ok(graph) => graph,
        Err(err) => {
            return fail(&test.id,
                        &ttl_path,
                        format!("error parsing the graph: {}", err));
        }
    };
    if ttl_graph.len() != nt_graph.len() {
        return fail(&test.id,
                    &ttl_path,
                    format!("different amounts of triples: {} vs {}",
                            ttl_graph.len(),
                            nt_graph.len()));
    }
    let iter = ttl_graph.iter().zip(nt_graph.iter());
    for i in iter {
        if i.0 != i.1 {
            return fail(&test.id,
                        &ttl_path,
                        format!("unequal triples:\n {:?} {:?} {:?}\n !=\n {:?} {:?} {:?}",
                                i.0.subject(),
                                i.0.predicate(),
                                i.0.object(),
                                i.1.subject(),
                                i.1.predicate(),
                                i.1.object()));
        }
    }
    pass(&test.id, &ttl_path)
}
fn run_eval_positive_syntax<'a>(test: &TestTurtlePositiveSyntax,
                                base: &str,
                                base_dir: &str)
                                -> rdfio::Result<Assertion> {
    let ttl_path = change_base(test.action.as_str(), base, base_dir);
    let ttl = try!(read_file(&ttl_path));
    if let Err(err) = load_graph(ttl.as_str(), test.action.as_str()) {
        return fail(&test.id,
                    &ttl_path,
                    format!("error parsing the graph: {}", err));
    };
    pass(&test.id, &ttl_path)
}
fn run_eval_negative_syntax<'a>(test: &TestTurtleNegativeSyntax,
                                base: &str,
                                base_dir: &str)
                                -> rdfio::Result<Assertion> {
    let ttl_path = change_base(test.action.as_str(), base, base_dir);
    let ttl = try!(read_file(&ttl_path));
    if let Ok(graph) = load_graph(ttl.as_str(), test.action.as_str()) {
        return fail(&test.id,
                    &ttl_path,
                    format!("no error parsing the graph, {} triples.", graph.len()));
    };
    pass(&test.id, &ttl_path)
}
fn run_eval_negative_eval<'a>(test: &TestTurtleNegativeEval,
                              base: &str,
                              base_dir: &str)
                              -> rdfio::Result<Assertion> {
    let ttl_path = change_base(test.action.as_str(), base, base_dir);
    let ttl = try!(read_file(&ttl_path));
    if let Ok(graph) = load_graph(ttl.as_str(), test.action.as_str()) {
        return fail(&test.id,
                    &ttl_path,
                    format!("no error parsing the graph, {} triples.", graph.len()));
    };
    pass(&test.id, &ttl_path)
}

fn output_as_turtle(assertions: &Vec<Assertion>) -> rdfio::Result<()> {
    let mut writer = tel::GraphCreator::with_capacity(100000);
    for a in assertions {
        try!(write_assertion(&a, &mut writer));
    }
    let graph: MyGraph = writer.collect().sort_blank_nodes();
    let mut ns = Namespaces::new();
    ns.set(b"dc", "http://purl.org/dc/elements/1.1/");
    ns.set(b"earl", "http://www.w3.org/ns/earl#");
    ns.set(b"xsd", "http://www.w3.org/2001/XMLSchema#");
    ns.set(b"test", "http://www.w3.org/2013/TurtleTests/manifest.ttl#");
    try!(write_turtle(&ns, graph.iter(), &mut ::std::io::stdout()));
    Ok(())
}

fn run(manifest_path: &str, output_turtle: bool) -> rdfio::Result<()> {
    // read the manifest file
    let path = Path::new(manifest_path);
    let mut dir = String::from(path.parent().unwrap().to_str().unwrap());
    dir.push('/');

    let manifest = try!(read_file(path.to_str().unwrap()));
    let base = "http://www.w3.org/2013/TurtleTests/manifest.ttl";
    let graph = try!(load_graph(manifest.as_str(), base));
    let assertions = try!(run_tests(&graph, base, dir.as_str()));
    if output_turtle {
        try!(output_as_turtle(&assertions));
    }
    Ok(())
}

fn check_manifest<P: AsRef<Path>>(path: P) -> bool {
    match fs::metadata(path) {
        Ok(meta) => meta.is_file(),
        _ => false,
    }
}

fn main() {
    let mut args = args();
    args.next();
    let mut output_turtle = false;
    let mut args_ok = false;
    let mut manifest_path = String::new();
    if args.len() == 1 {
        manifest_path = args.next().unwrap();
        args_ok = check_manifest(&manifest_path);
    } else if args.len() == 2 {
        let arg1 = args.next().unwrap();
        manifest_path = args.next().unwrap();
        args_ok = check_manifest(&manifest_path);
        if arg1 == "--output-turtle" {
            output_turtle = true;
        }
    }
    if !args_ok {
        println_stderr!("Usage: w3tests [--output-turtle] MANIFEST_FILE");
        std::process::exit(-1);
    }
    if let Err(e) = run(manifest_path.as_str(), output_turtle) {
        println_stderr!("ERROR {:?}", e);
        std::process::exit(-1);
    }
}
