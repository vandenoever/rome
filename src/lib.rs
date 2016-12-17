#[macro_use]
extern crate nom;
use nom::IResult;
use nom::ErrorKind;
use nom::IResult::Done;
use nom::Needed;
use std::io;
use std::io::Read;
use std::fs::File;

macro_rules! one_if (
  ($i:expr, $f: expr) => (
    {
      if let Some(c) =  $i.chars().next() {
        if $f(c) {
          IResult::Done(&$i[1..], c)
        } else {
          IResult::Error(error_position!($crate::ErrorKind::OneOf, $i))
        }
      } else {
        IResult::Incomplete::<_, _>(Needed::Size(1))
      }
    }
  );
);


#[derive(Debug,PartialEq,Eq,Clone)]
enum IRI {
    IRI(String),
    PrefixedName(String,String)
}

#[derive(Debug,PartialEq,Eq)]
enum RDFLiteralType {
    LangTag(String),
    DataType(IRI)
}

#[derive(Debug,PartialEq,Eq)]
struct RDFLiteral {
    string: String,
    data_type: Option<RDFLiteralType>
}

#[derive(Debug,PartialEq,Eq)]
enum Literal {
    RDFLiteral(RDFLiteral),
    Integer(i64),
    Boolean(bool)
}

#[derive(Debug,PartialEq,Eq)]
enum Object {
    IRI(IRI),
    Literal(Literal),
}

#[derive(Debug,PartialEq,Eq)]
struct PredicatedObjects {
    verb: IRI,
    objects: Vec<Object>,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Triples {
    subject: IRI,
    predicated_objects_list: Vec<PredicatedObjects>,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Statement {
    Prefix(String,String),
    Base(String),
    Triples(Triples),
}

/// [1] turtleDoc ::= statement*
named!(turtle<&str,Vec<Statement>>, delimited!(
    take_while_s!(is_ws),
    separated_list!(take_while_s!(is_ws), statement),
    take_while_s!(is_ws)
));

/// [2] statement ::= directive | triples '.'
named!(statement<&str,Statement>, alt!(statement_triples | prefix_id | base | sparql_base | sparql_prefix));

named!(statement_triples<&str,Statement>, do_parse!(
    triples: triples >>
    take_while_s!(is_ws) >>
    tag_s!(".") >>
    take_while_s!(is_ws) >>
    (Statement::Triples(triples))
));

/// [4] prefixID ::= '@prefix' PNAME_NS IRIREF '.'
named!(prefix_id<&str,Statement>, do_parse!(
    tag_s!("@prefix") >>
    take_while1_s!(is_ws) >>
    pn_prefix: pn_prefix >>
    take_while_s!(is_ws) >>
    tag_s!(":") >>
    take_while_s!(is_ws) >>
    iri_ref: iri_ref >>
    take_while_s!(is_ws) >>
    tag_s!(".") >>
    (Statement::Prefix(pn_prefix, iri_ref))
));

/// [5] base ::= '@base' IRIREF '.'
named!(base<&str,Statement>, do_parse!(
    tag_s!("@base") >>
    take_while1_s!(is_ws) >>
    iri_ref: iri_ref >>
    take_while_s!(is_ws) >>
    tag_s!(".") >>
    (Statement::Base(iri_ref))
));

/// [5s] sparqlBase ::= "BASE" IRIREF
named!(sparql_base<&str,Statement>, do_parse!(
    tag_s!("BASE") >>
    take_while1_s!(is_ws) >>
    iri_ref: iri_ref >>
    (Statement::Base(iri_ref))
));

/// [6s] sparqlPrefix ::= "PREFIX" PNAME_NS IRIREF
named!(sparql_prefix<&str,Statement>, do_parse!(
    tag_s!("PREFIX") >>
    take_while1_s!(is_ws) >>
    pn_prefix: pn_prefix >>
    take_while_s!(is_ws) >>
    tag_s!(":") >>
    take_while_s!(is_ws) >>
    iri_ref: iri_ref >>
    (Statement::Prefix(pn_prefix, iri_ref))
));

/// [6] triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?
named!(triples<&str,Triples>, do_parse!(
    subject: iri >>
    take_while_s!(is_ws) >>
    predicated_objects_list: predicated_objects_list >>
    (Triples{
        subject: subject,
        predicated_objects_list: predicated_objects_list
    })
));

fn is_ws(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn in_range(c: char, lower: u32, upper: u32) -> bool {
    c as u32 >= lower && c as u32 <= upper
}

fn is_pn_chars_base(c: char) -> bool {
    is_alpha(c)
    || in_range(c, 0xC0, 0x00D6)
    || in_range(c, 0x00D8, 0x00F6)
    || in_range(c, 0x00F8, 0x02FF)
    || in_range(c, 0x0370, 0x037D)
    || in_range(c, 0x037F, 0x1FFF)
    || in_range(c, 0x200C, 0x200D)
    || in_range(c, 0x2070, 0x218F)
    || in_range(c, 0x2C00, 0x2FEF)
    || in_range(c, 0x3001, 0xD7FF)
    || in_range(c, 0xF900, 0xFDCF)
    || in_range(c, 0xFDF0, 0xFFFD)
    || in_range(c, 0x10000, 0xEFFFF)
}

fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c) || c == '-' || is_digit(c) || c == 0xB7 as char
    || in_range(c, 0x0300, 0x036F)
    || in_range(c, 0x203F, 0x2040)
}

named!(pn_prefix_group<&str,String>,  do_parse!(
    tag_s!(".") >>
    p: take_while1_s!(is_pn_chars) >>
    ({
        let mut s = String::from(".");
        s.push_str(p);
        s
    })
));

named!(pn_prefix<&str,String>, map!(recognize!(do_parse!(
    one_if!(is_alpha) >>
    take_while_s!(is_pn_chars) >>
    many0!(pn_prefix_group) >> ()
)),String::from));

named!(pn_local<&str,String>, map!(recognize!(do_parse!(
    one_if!(is_alpha) >>
    take_while_s!(is_alphanum) >> ()
)), String::from));

named!(prefixed_name<&str,IRI>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    pn_local: opt!(pn_local) >>
    (IRI::PrefixedName(
        pn_prefix.unwrap_or(String::new()),
        pn_local.unwrap_or(String::new())
    ))
));

#[test]
fn test_prefixed_name() {
    assert_eq!(prefixed_name("a:a"), Done(&""[..], IRI::PrefixedName(String::from("a"),String::from("a"))));
    assert_eq!(prefixed_name(": "), Done(&" "[..], IRI::PrefixedName(String::new(),String::new())));
}

named!(alpha<&str,&str>, take_while1_s!(is_alpha));
named!(alphanumeric<&str,&str>, take_while1_s!(is_alphanum));
named!(digit<&str,&str>, take_while1_s!(is_digit));

// IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_ref<&str,String>, delimited!(
    tag_s!("<"),
    map!(
        take_while_s!(is_iri_ref),
        String::from),
    tag_s!(">")
));

named!(iri_iri<&str,IRI>, map!(iri_ref, IRI::IRI));

named!(iri<&str,IRI>, alt!(iri_iri|prefixed_name));

#[test]
fn test_iri() {
    assert_eq!(iri("<urn:123>"), Done(&""[..],IRI::IRI(String::from("urn:123"))));
}

named!(iri_ref_literal<&str,RDFLiteralType>, do_parse!(
    tag_s!("^^") >>
    iri: iri >>
    (RDFLiteralType::DataType(iri))
));

#[inline]
fn is_string_literal_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_quote<&str,String>, delimited!(
    tag_s!("\""),
    map!(
        take_while_s!(is_string_literal_quote),
        String::from
    ),
    tag_s!("\"")
));

#[test]
fn test_string_literal_quote() {
    assert_eq!(string_literal_quote("\"\""), Done(&""[..], String::new()));
}

#[inline]
fn is_string_literal_single_quote(chr: char) -> bool {
    chr != '\'' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_single_quote<&str,String>, delimited!(
    tag_s!("'"),
    map!(
        take_while_s!(is_string_literal_single_quote),
        String::from
    ),
    tag_s!("'")
));

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(string_literal_single_quote("''"), Done(&""[..], String::new()));
}

#[inline]
fn is_string_literal_long_single_quote(chr: char) -> bool {
    chr != '\'' as char && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_long_single_quote<&str,String>, delimited!(
    tag_s!("'''"),
    map!(
        take_while_s!(is_string_literal_long_single_quote),
        String::from
    ),
    tag_s!("'''")
));

#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(string_literal_long_single_quote("''''''"), Done(&""[..], String::new()));
}

#[inline]
fn is_string_literal_long_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_long_quote<&str,String>, delimited!(
    tag_s!("\"\"\""),
    map!(
        take_while_s!(is_string_literal_long_quote),
        String::from
    ),
    tag_s!("\"\"\"")
));

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(string_literal_long_quote("\"\"\"\"\"\""), Done(&""[..], String::new()));
}

named!(string<&str,String>, alt!(string_literal_quote | string_literal_single_quote
    | string_literal_long_single_quote | string_literal_long_quote));

named!(langtag<&str,RDFLiteralType>, do_parse!(
    tag_s!("@") >>
    langtag: alpha >>
    opt!(langtag_suffix) >>
    (RDFLiteralType::LangTag(String::from(langtag)))
));

named!(langtag_suffix<&str,&str>, do_parse!(
    tag_s!("-") >>
    langtag: alphanumeric >>
    (langtag)
));

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Done(&" "[..], RDFLiteralType::LangTag(String::from("nl"))));
}

named!(rdfliteral<&str,Literal>, do_parse!(
    string: string >>
    data_type: opt!(alt!(langtag | iri_ref_literal)) >>
    (Literal::RDFLiteral(RDFLiteral{
        string:string,
        data_type:data_type
    }))
));

#[test]
fn test_rdfliteral() {
    let r = RDFLiteral{string:String::new(),data_type:None};
    assert_eq!(rdfliteral("'' "), Done(&" "[..], Literal::RDFLiteral(r)));
}

// INTEGER ::= [+-]? [0-9]+
named!(integer<&str,Literal>, do_parse!(
    sign: opt!(alt!(tag_s!("+") | tag_s!("-"))) >>
    digit: digit >>
    (Literal::Integer({
        let v = i64::from_str_radix(digit, 10).unwrap();
        if sign == Some("-") { -v } else { v }
    }))
));

#[test]
fn test_integer() {
    assert_eq!(integer("1"), Done(&""[..], Literal::Integer(1)));
    assert_eq!(integer("+1"), Done(&""[..], Literal::Integer(1)));
    assert_eq!(integer("-1"), Done(&""[..], Literal::Integer(-1)));
}

named!(boolean<&str,Literal>, do_parse!(
    b: alt!(tag_s!("true") | tag_s!("false")) >>
    (Literal::Boolean(b == "true"))
));

#[test]
fn test_boolean() {
    assert_eq!(boolean("true"), Done(&""[..], Literal::Boolean(true)));
    assert_eq!(boolean("false"), Done(&""[..], Literal::Boolean(false)));
}

named!(literal<&str,Literal>, alt!(rdfliteral | boolean| integer));

#[test]
fn test_literal() {
    assert_eq!(literal("true"), Done(&""[..], Literal::Boolean(true)));
    assert_eq!(literal("false"), Done(&""[..], Literal::Boolean(false)));
}

named!(object<&str,Object>, alt!(
    map!(literal,Object::Literal) |
    map!(iri,Object::IRI)
));

named!(object_list_separator<&str,()>, do_parse!(
    take_while_s!(is_ws) >>
    tag_s!(",") >>
    take_while_s!(is_ws) >> ()
));

named!(object_list<&str,Vec<Object>>, separated_nonempty_list!(call!(object_list_separator),object));

#[test]
fn test_object_list() {
    let v = vec![
    Object::Literal(Literal::Boolean(true)),
    Object::Literal(Literal::Integer(1)),
    Object::Literal(Literal::Boolean(false))];
    assert_eq!(object_list("true, 1 , false"), Done(&""[..],v));
}

named!(a<&str,IRI>, do_parse!(
    tag_s!("a") >>
    (IRI::IRI(String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")))
));

named!(verb<&str,IRI>, alt!(iri|a));

named!(predicated_objects<&str,PredicatedObjects>, do_parse!(
    verb: verb >>
    take_while_s!(is_ws) >>
    objects: object_list >>
    (PredicatedObjects{
        verb:verb,
        objects:objects
    })
));

#[test]
fn test_predicated_objects() {
    let v = vec![Object::Literal(Literal::Integer(1))];
    let po = PredicatedObjects{verb:IRI::IRI(String::from("urn:123")),objects:v};
    assert_eq!(predicated_objects("<urn:123> 1"), Done(&""[..],po));
}

named!(predicated_objects_list_list_separator<&str,()>, do_parse!(
    take_while_s!(is_ws) >>
    tag_s!(";") >>
    take_while_s!(is_ws) >> ()
));

named!(predicated_objects_list<&str,Vec<PredicatedObjects>>, separated_nonempty_list!(call!(predicated_objects_list_list_separator),predicated_objects));


#[test]
fn test_triples() {
    let v = vec![Object::Literal(Literal::Integer(1))];
    let i = IRI::IRI(String::from("urn:123"));
    let po = vec![PredicatedObjects{verb:i.clone(),objects:v}];
    let t = Triples{subject:i,predicated_objects_list:po};
    assert_eq!(triples("<urn:123> <urn:123> 1"), Done(&""[..],t));
}

#[test]
fn test_statement_triples() {
    let v = vec![Object::Literal(Literal::Integer(1))];
    let i = IRI::IRI(String::from("urn:123"));
    let po = vec![PredicatedObjects{verb:i.clone(),objects:v}];
    let t = Triples{subject:i,predicated_objects_list:po};
    let s = Statement::Triples(t);
    assert_eq!(statement_triples("<urn:123> <urn:123> 1."), Done(&""[..],s));
}

#[test]
fn test_prefix_id() {
    assert_eq!(prefix_id("@prefix a.b.c : <urn> ."), Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
}

#[test]
fn test_base() {
    assert_eq!(base("@base <urn> ."), Done(&""[..],Statement::Base(String::from("urn"))));
}

#[test]
fn test_sparql_base() {
    assert_eq!(sparql_base("BASE <urn>"), Done(&""[..],Statement::Base(String::from("urn"))));
}

#[test]
fn test_sparql_prefix() {
    assert_eq!(sparql_prefix("PREFIX a.b.c : <urn>"), Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
}

pub fn parse(data: &str) -> nom::IResult<&str,Vec<Statement>> {
    turtle(data)
}

pub fn run(path: &str) -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));
    let r = parse(s.as_str());
    println!("{}", s);
    println!("{:?}", r);
    Ok(())
}

#[test]
fn test_run() {
    let path = "/tmp/tracker/tests/libtracker-data/update/delete-insert-where-1.ontology";
    if let Err(e) = run(&path) {
        println!("{:?}", e);
    }
}
