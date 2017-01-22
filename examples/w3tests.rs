extern crate rdfio;
extern crate time;
use std::env::args;
use std::fs;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::rc::Rc;
use rdfio::graph;
use rdfio::graph::*;
use rdfio::graphs::tel;
use rdfio::io::{TurtleParser, write_turtle, write_ntriples};
use rdfio::namespaces::Namespaces;

type MyGraph = tel::Graph64;
type MyIter<'g> = <MyGraph as Graph<'g>>::SPORangeIter;
type MyBlankNodeOrIRI<'g> =
    graph::BlankNodeOrIRI<'g, <MyGraph as Graph<'g>>::BlankNodePtr, <MyGraph as Graph<'g>>::IRIPtr>;
type MyResource<'g> = graph::Resource<'g,
                                      <MyGraph as Graph<'g>>::BlankNodePtr,
                                      <MyGraph as Graph<'g>>::IRIPtr,
                                      <MyGraph as Graph<'g>>::LiteralPtr>;

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
#[derive(PartialEq)]
struct Datatype {
    datatype: String,
}
impl<'g> DatatypePtr<'g> for Datatype {
    fn as_str(&self) -> &str {
        &self.datatype
    }
}
#[derive(Debug,Clone,Ord,PartialOrd,Eq,PartialEq)]
struct Literal {
    value: String,
    datatype: String,
    language: Option<String>,
}
impl<'g> LiteralPtr<'g> for Literal {
    type DatatypePtr = Datatype;
    fn as_str(&self) -> &str {
        &self.value
    }
    fn datatype(&self) -> Datatype {
        Datatype {
            datatype: self.datatype.clone()
        }
    }
    fn datatype_str(&self) -> &str {
        &self.datatype
    }
    fn language(&self) -> Option<&str> {
        match &self.language {
            &Some(ref l) => Some(l.as_str()),
            &None => None,
        }
    }
}

struct Cache<'g, W: 'g>
    where W: GraphWriter<'g>
{
    rdf_type: Option<W::IRI>,
    earl_assertion: Option<W::IRI>,
    earl_test_result: Option<W::IRI>,
    earl_test: Option<W::IRI>,
    earl_result: Option<W::IRI>,
    earl_passed: Option<W::IRI>,
    earl_failed: Option<W::IRI>,
    earl_cant_tell: Option<W::IRI>,
    earl_outcome: Option<W::IRI>,
    dc_date: Option<W::IRI>,
    xsd_date_time: Option<W::Datatype>,
}

impl<'g, W: 'g> Cache<'g, W>
    where W: GraphWriter<'g>
{
    fn new() -> Cache<'g, W> {
        Cache {
            rdf_type: None,
            earl_assertion: None,
            earl_test_result: None,
            earl_test: None,
            earl_result: None,
            earl_passed: None,
            earl_failed: None,
            earl_cant_tell: None,
            earl_outcome: None,
            dc_date: None,
            xsd_date_time: None,
        }
    }
    fn get<'a>(o: &'a mut Option<W::IRI>, w: &mut W, iri: &str) -> W::IRI {
        if o.is_none() {
            *o = Some(w.create_iri(&iri));
        }
        o.clone().unwrap()
    }
    fn rdf_type(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.rdf_type, w, &RDF_TYPE)
    }
    fn earl_assertion(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_assertion, w, &EARL_ASSERTION)
    }
    fn earl_test_result(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_test_result, w, &EARL_TEST_RESULT)
    }
    fn earl_test(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_test, w, &EARL_TEST)
    }
    fn earl_result(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_result, w, &EARL_RESULT)
    }
    fn earl_passed(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_passed, w, &EARL_PASSED)
    }
    fn earl_failed(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_failed, w, &EARL_FAILED)
    }
    fn earl_cant_tell(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_cant_tell, w, &EARL_CANT_TELL)
    }
    fn earl_outcome(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.earl_outcome, w, &EARL_OUTCOME)
    }
    fn dc_date(&mut self, w: &mut W) -> W::IRI {
        Cache::get(&mut self.dc_date, w, &DC_DATE)
    }
    fn xsd_date_time(&mut self, w: &mut W) -> W::Datatype {
        if self.xsd_date_time.is_none() {
            self.xsd_date_time = Some(w.create_datatype(&XSD_DATE_TIME));
        }
        self.xsd_date_time.clone().unwrap()
    }
}


fn write_assertion<'a, 'g, W: 'g>(assertion: &'a Assertion,
                                  w: &mut W,
                                  cache: &mut Cache<'g, W>)
                                  -> rdfio::Result<()>
    where W: GraphWriter<'g>
{
    let rdf_type = cache.rdf_type(w);
    let earl_assertion = cache.earl_assertion(w);
    let earl_test_result = cache.earl_test_result(w);
    let earl_result = cache.earl_result(w);
    let earl_passed = cache.earl_passed(w);
    let earl_failed = cache.earl_failed(w);
    let earl_cant_tell = cache.earl_cant_tell(w);
    let earl_outcome = cache.earl_outcome(w);
    let earl_test = cache.earl_test(w);
    let dc_date = cache.dc_date(w);

    let assertion_blank_node = w.create_blank_node();
    w.add_blank_iri(&assertion_blank_node, &rdf_type, &earl_assertion);
    let date = format!("{}", assertion.date.rfc3339());
    let date_time = cache.xsd_date_time(w);
    let literal = w.create_literal_datatype(&date, &date_time);
    w.add_blank_literal(&assertion_blank_node, &dc_date, &literal);
    let result_blank_node = w.create_blank_node();
    w.add_blank_iri(&result_blank_node, &rdf_type, &earl_test_result);
    w.add_blank_blank(&assertion_blank_node, &earl_result, &result_blank_node);
    let outcome = match assertion.result.outcome {
        Outcome::Passed => &earl_passed,
        Outcome::Failed => &earl_failed,
        Outcome::CannotTell => &earl_cant_tell,
    };
    w.add_blank_iri(&result_blank_node, &earl_outcome, outcome);
    let test = w.create_iri(&assertion.test.as_str());
    w.add_blank_iri(&assertion_blank_node, &earl_test, &test);
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
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn load_graph(data: &str, base: &str) -> rdfio::Result<MyGraph> {
    let mut writer = tel::GraphCreator::with_capacity(65000);
    {
        let mut triples = TurtleParser::new(data, base, &mut writer)?;
        while let Some(step) = triples.next() {
            step?;
        }
    }
    Ok(writer.collect().sort_blank_nodes())
}

fn read<'g, T, B: 'g, I: 'g, L: 'g, F, R>(mut last: Option<T>,
                                          i: &mut Iterator<Item = T>,
                                          predicate: &str,
                                          convert: F)
                                          -> Result<(R, Option<T>), String>
    where T: graph::Triple<'g, B, I, L>,
          B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>,
          F: Fn(Resource<'g, B, I, L>) -> Result<R, String>
{
    last = last.or_else(|| i.next());
    while let Some(triple) = last {
        if triple.predicate().as_str() == predicate {
            return Ok((convert(triple.object())?, None));
        }
        last = i.next();
    }
    Err(format!("Cannot find {}.", predicate))
}

fn to_string<'g, B, I, L>(object: Resource<'g, B, I, L>) -> Result<String, String>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    match object {
        graph::Resource::IRI(iri) => Ok(String::from(iri.as_str())),
        graph::Resource::Literal(l) => Ok(String::from(l.as_str())),
        _ => Err(String::from("object is not an iri or literal")),
    }
}

fn to_approval<'g, B, I, L>(object: Resource<'g, B, I, L>) -> Result<Approval, String>
    where B: BlankNodePtr<'g>,
          I: IRIPtr<'g>,
          L: LiteralPtr<'g>
{
    match &object {
        &graph::Resource::IRI(ref iri) if iri.as_str() == RDFT_APPROVED => Ok(Approval::Approved),
        &graph::Resource::IRI(ref iri) if iri.as_str() == RDFT_PROPOSED => Ok(Approval::Proposed),
        &graph::Resource::IRI(ref iri) if iri.as_str() == RDFT_REJECTED => Ok(Approval::Rejected),
        _ => Err(String::from("object is not the right value for approval")),
    }
}

fn load_test_turtle_eval(graph: &MyGraph,
                         subject: MyBlankNodeOrIRI)
                         -> Result<TestTurtleEval, String> {
    let mut i = graph.iter_subject(&subject);
    let (comment, prev) = read(None, &mut i, RDFS_COMMENT, to_string)?;
    let (action, prev) = read(prev, &mut i, MF_ACTION, to_string)?;
    let (name, prev) = read(prev, &mut i, MF_NAME, to_string)?;
    let (result, prev) = read(prev, &mut i, MF_RESULT, to_string)?;
    let (approval, _) = read(prev, &mut i, RDFT_APPROVAL, to_approval)?;
    Ok(TestTurtleEval {
        id: subject_to_string(&subject),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
        result: result,
    })
}
fn load_positive_syntax(graph: &MyGraph,
                        subject: MyBlankNodeOrIRI)
                        -> Result<TestTurtlePositiveSyntax, String> {
    let mut i = graph.iter_subject(&subject);
    let (comment, prev) = read(None, &mut i, RDFS_COMMENT, to_string)?;
    let (action, prev) = read(prev, &mut i, MF_ACTION, to_string)?;
    let (name, prev) = read(prev, &mut i, MF_NAME, to_string)?;
    let (approval, _) = read(prev, &mut i, RDFT_APPROVAL, to_approval)?;
    Ok(TestTurtlePositiveSyntax {
        id: subject_to_string(&subject),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
    })
}
fn load_negative_syntax(graph: &MyGraph,
                        subject: MyBlankNodeOrIRI)
                        -> Result<TestTurtleNegativeSyntax, String> {
    let mut i = graph.iter_subject(&subject);
    let (comment, prev) = read(None, &mut i, RDFS_COMMENT, to_string)?;
    let (action, prev) = read(prev, &mut i, MF_ACTION, to_string)?;
    let (name, prev) = read(prev, &mut i, MF_NAME, to_string)?;
    let (approval, _) = read(prev, &mut i, RDFT_APPROVAL, to_approval)?;
    Ok(TestTurtleNegativeSyntax {
        id: subject_to_string(&subject),
        name: name,
        comment: comment,
        approval: approval,
        action: action,
    })
}
fn load_negative_eval(graph: &MyGraph,
                      subject: MyBlankNodeOrIRI)
                      -> Result<TestTurtleNegativeEval, String> {
    let mut i = graph.iter_subject(&subject);
    let (comment, prev) = read(None, &mut i, RDFS_COMMENT, to_string)?;
    let (action, prev) = read(prev, &mut i, MF_ACTION, to_string)?;
    let (name, prev) = read(prev, &mut i, MF_NAME, to_string)?;
    let (approval, _) = read(prev, &mut i, RDFT_APPROVAL, to_approval)?;
    Ok(TestTurtleNegativeEval {
        id: subject_to_string(&subject),
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

fn subject_to_string(subject: &MyBlankNodeOrIRI) -> Rc<String> {
    match subject {
        &graph::BlankNodeOrIRI::IRI(ref iri) => Rc::new(String::from(iri.as_str())),
        _ => {
            panic!("a blank node as subject is not expected");
        }
    }
}

fn run_tests(graph: &MyGraph, base: &str, base_dir: &str) -> rdfio::Result<Vec<Assertion>> {
    let mut assertions = Vec::new();
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleEval", RDF_TYPE) {
        let test = load_test_turtle_eval(&graph, t.subject())?;
        let r = run_eval(&test, base, base_dir)?;
        eval_result(&r);
        assertions.push(r);
    }
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtlePositiveSyntax",
                                        RDF_TYPE) {
        let test = load_positive_syntax(&graph, t.subject())?;
        let r = run_eval_positive_syntax(&test, base, base_dir)?;
        eval_result(&r);
        assertions.push(r);
    }
    for t in
        graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleNegativeSyntax",
                                        RDF_TYPE) {
        let test = load_negative_syntax(&graph, t.subject())?;
        let r = run_eval_negative_syntax(&test, base, base_dir)?;
        eval_result(&r);
        assertions.push(r);
    }
    for t in graph.iter_object_iri_predicate("http://www.w3.org/ns/rdftest#TestTurtleNegativeEval",
                                   RDF_TYPE) {
        let test = load_negative_eval(&graph, t.subject())?;
        let r = run_eval_negative_eval(&test, base, base_dir)?;
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
    let ttl = read_file(&ttl_path)?;
    let nt = read_file(&nt_path)?;
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
    let graph1 = graph_to_ntriples(&ttl_graph)?;
    let graph2 = graph_to_ntriples(&nt_graph)?;
    if graph1 != graph2 {
        return fail(&test.id,
                    &ttl_path,
                    format!("unequal graphs:\n{}\n{}\n", graph1, graph2));
    }
    pass(&test.id, &ttl_path)
}
fn graph_to_ntriples(graph: &MyGraph) -> rdfio::Result<String> {
    let mut bytes = Vec::new();
    write_ntriples(graph.iter(), graph, &mut bytes)?;
    let string = String::from_utf8(bytes)?;
    Ok(string)
}
fn run_eval_positive_syntax<'a>(test: &TestTurtlePositiveSyntax,
                                base: &str,
                                base_dir: &str)
                                -> rdfio::Result<Assertion> {
    let ttl_path = change_base(test.action.as_str(), base, base_dir);
    let ttl = read_file(&ttl_path)?;
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
    let ttl = read_file(&ttl_path)?;
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
    let ttl = read_file(&ttl_path)?;
    if let Ok(graph) = load_graph(ttl.as_str(), test.action.as_str()) {
        return fail(&test.id,
                    &ttl_path,
                    format!("no error parsing the graph, {} triples.", graph.len()));
    };
    pass(&test.id, &ttl_path)
}

fn output_as_turtle(assertions: &Vec<Assertion>) -> rdfio::Result<()> {
    let mut writer = tel::GraphCreator::with_capacity(100000);
    let mut cache = Cache::new();
    for a in assertions {
        write_assertion(&a, &mut writer, &mut cache)?;
    }
    let graph: MyGraph = writer.collect().sort_blank_nodes();
    let mut ns = Namespaces::new();
    ns.set(b"dc", "http://purl.org/dc/elements/1.1/");
    ns.set(b"earl", "http://www.w3.org/ns/earl#");
    ns.set(b"xsd", "http://www.w3.org/2001/XMLSchema#");
    ns.set(b"test", "http://www.w3.org/2013/TurtleTests/manifest.ttl#");
    write_turtle(&ns, graph.iter(), &graph, &mut ::std::io::stdout())?;
    Ok(())
}

fn run(manifest_path: &str, output_turtle: bool) -> rdfio::Result<()> {
    // read the manifest file
    let path = Path::new(manifest_path);
    let mut dir = String::from(path.parent().unwrap().to_str().unwrap());
    dir.push('/');

    let manifest = read_file(path.to_str().unwrap())?;
    let base = "http://www.w3.org/2013/TurtleTests/manifest.ttl";
    let graph = load_graph(manifest.as_str(), base)?;
    let assertions = run_tests(&graph, base, dir.as_str())?;
    if output_turtle {
        output_as_turtle(&assertions)?;
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
