#[macro_use]
extern crate nom;
use nom::*;
use nom::IResult::*;

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
  tag!(".") >>
  p: take_while1_s!(is_pn_chars) >>
  ({
  		let mut s = String::from(".");
  		s.push_str(p);
  		s
  })
));

named!(pn_prefix<&str,String>, do_parse!(
  p1: take_s!(1) >>
  p2: take_while_s!(is_pn_chars) >>
  ps: many0!(pn_prefix_group) >>
  ({
  		let mut s = String::from(p1);
  		s.push_str(p2);
  		for p in ps {
  			s.push_str(p.as_str());
  		}
  		s
  })
));

named!(alpha<&str,&str>, take_while_s!(is_alpha));
named!(alphanumeric<&str,&str>, take_while_s!(is_alphanum));
named!(digit<&str,&str>, take_while_s!(is_digit));

// IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_ref<&str,String>,
  delimited!(
    tag!("<"),
    map!(
      take_while_s!(is_iri_ref),
      String::from),
    tag!(">")
  )
);

named!(iri<&str,String>, call!(iri_ref));

#[test]
fn test_iri() {
    assert_eq!(iri("<urn:123>"), Done(&""[..],String::from("urn:123")));
}

named!(iri_ref_literal<&str,RDFLiteralType>, do_parse!(
  tag!("^^") >>
  iri: iri >>
  (RDFLiteralType::DataType(iri))
));

#[inline]
fn is_string_literal_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_quote<&str,String>,
  delimited!(
    tag!("\""),
    map!(
      take_while_s!(is_string_literal_quote),
      String::from),
    tag!("\"")
  )
);

#[test]
fn test_string_literal_quote() {
    assert_eq!(string_literal_quote("\"\""), Done(&""[..], String::from("")));
}

#[inline]
fn is_string_literal_single_quote(chr: char) -> bool {
    chr != '\'' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_single_quote<&str,String>,
  delimited!(
    tag!("'"),
    map!(
      take_while_s!(is_string_literal_single_quote),
      String::from),
    tag!("'")
  )
);

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(string_literal_single_quote("''"), Done(&""[..], String::from("")));
}

#[inline]
fn is_string_literal_long_single_quote(chr: char) -> bool {
    chr != '\'' as char && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_long_single_quote<&str,String>,
  delimited!(
    tag!("'''"),
    map!(
      take_while_s!(is_string_literal_long_single_quote),
      String::from),
    tag!("'''")
  )
);


#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(string_literal_long_single_quote("''''''"), Done(&""[..], String::from("")));
}

#[inline]
fn is_string_literal_long_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_long_quote<&str,String>,
  delimited!(
    tag!("\"\"\""),
    map!(
      take_while_s!(is_string_literal_long_quote),
      String::from),
    tag!("\"\"\"")
  )
);

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(string_literal_long_quote("\"\"\"\"\"\""), Done(&""[..], String::from("")));
}

named!(string<&str,String>, alt!(string_literal_quote | string_literal_single_quote
    | string_literal_long_single_quote | string_literal_long_quote));

named!(langtag<&str,RDFLiteralType>, do_parse!(
  tag!("@") >>
  langtag: alpha >>
  opt!(langtag_suffix) >>
  (RDFLiteralType::LangTag(String::from(langtag)))
));

named!(langtag_suffix<&str,&str>,
       do_parse!(tag!("-") >> langtag: alphanumeric >> (langtag)));

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Done(&" "[..], RDFLiteralType::LangTag(String::from("nl"))));
}

#[derive(Debug,PartialEq,Eq)]
enum RDFLiteralType {
  LangTag(String),
  DataType(String)
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
	IRI(String),
	Literal(Literal),
}

#[derive(Debug,PartialEq,Eq)]
struct PredicatedObjects {
	verb: String,
	objects: Vec<Object>,
}

#[derive(Debug,PartialEq,Eq)]
struct Triples {
	subject: String,
	predicated_objects_list: Vec<PredicatedObjects>,
}

#[derive(Debug,PartialEq,Eq)]
enum Statement {
	Prefix(String,String),
	Base(String),
	Triples(Triples),
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
    let r = RDFLiteral{string:String::from(""),data_type:None};
    assert_eq!(rdfliteral("'' "), Done(&" "[..], Literal::RDFLiteral(r)));
}

// INTEGER ::= [+-]? [0-9]+
named!(integer<&str,Literal>, do_parse!(
  sign: opt!(alt!(tag!("+") | tag!("-"))) >>
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
    b: alt!(tag!("true") | tag!("false")) >>
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
  tag!(",") >>	
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

named!(predicated_objects<&str,PredicatedObjects>, do_parse!(
	verb: iri >>
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
	let po = PredicatedObjects{verb:String::from("urn:123"),objects:v};
    assert_eq!(predicated_objects("<urn:123> 1"), Done(&""[..],po));
}

named!(predicated_objects_list_list_separator<&str,()>, do_parse!(
  take_while_s!(is_ws) >>
  tag!(";") >>	
  take_while_s!(is_ws) >> ()
));

named!(predicated_objects_list<&str,Vec<PredicatedObjects>>, separated_nonempty_list!(call!(predicated_objects_list_list_separator),predicated_objects));

named!(triples<&str,Triples>, do_parse!(
	subject: iri >>
    take_while_s!(is_ws) >>
	predicated_objects_list: predicated_objects_list >>
	(Triples{
	   subject: subject,
	   predicated_objects_list: predicated_objects_list
	})
));

#[test]
fn test_triples() {
	let v = vec![Object::Literal(Literal::Integer(1))];
	let po = vec![PredicatedObjects{verb:String::from("urn:123"),objects:v}];
	let t = Triples{subject:String::from("urn:123"),predicated_objects_list:po};
    assert_eq!(triples("<urn:123> <urn:123> 1"), Done(&""[..],t));
}

named!(statement_triples<&str,Statement>, do_parse!(
  triples: triples >>
  take_while_s!(is_ws) >>
  tag!(".") >>
  (Statement::Triples(triples))
));

#[test]
fn test_statement_triples() {
	let v = vec![Object::Literal(Literal::Integer(1))];
	let po = vec![PredicatedObjects{verb:String::from("urn:123"),objects:v}];
	let t = Triples{subject:String::from("urn:123"),predicated_objects_list:po};
	let s = Statement::Triples(t);
    assert_eq!(statement_triples("<urn:123> <urn:123> 1."), Done(&""[..],s));
}

named!(prefix_id<&str,Statement>, do_parse!(
  tag!("@prefix") >>
  take_while1_s!(is_ws) >>
  pn_prefix: pn_prefix >>
  take_while_s!(is_ws) >>
  tag!(":") >>
  take_while_s!(is_ws) >>
  iri_ref: iri_ref >>
  take_while_s!(is_ws) >>
  tag!(".") >>
  (Statement::Prefix(pn_prefix, iri_ref))
));

#[test]
fn test_prefix_id() {
	assert_eq!(prefix_id("@prefix a.b.c : <urn> ."), Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
}

named!(base<&str,Statement>, do_parse!(
  tag!("@base") >>
  take_while1_s!(is_ws) >>
  iri_ref: iri_ref >>
  take_while_s!(is_ws) >>
  tag!(".") >>
  (Statement::Base(iri_ref))
));

#[test]
fn test_base() {
	assert_eq!(base("@base <urn> ."), Done(&""[..],Statement::Base(String::from("urn"))));
}

named!(sparql_base<&str,Statement>, do_parse!(
  tag!("BASE") >>
  take_while1_s!(is_ws) >>
  iri_ref: iri_ref >>
  (Statement::Base(iri_ref))
));

#[test]
fn test_sparql_base() {
	assert_eq!(sparql_base("BASE <urn>"), Done(&""[..],Statement::Base(String::from("urn"))));
}

named!(sparql_prefix<&str,Statement>, do_parse!(
  tag!("PREFIX") >>
  take_while1_s!(is_ws) >>
  pn_prefix: pn_prefix >>
  take_while_s!(is_ws) >>
  tag!(":") >>
  take_while_s!(is_ws) >>
  iri_ref: iri_ref >>
  (Statement::Prefix(pn_prefix, iri_ref))
));

#[test]
fn test_sparql_prefix() {
	assert_eq!(sparql_prefix("PREFIX a.b.c : <urn>"), Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
}

named!(statement<&str,Statement>, alt!(statement_triples | prefix_id | base | sparql_base | sparql_prefix));

pub fn parse() {
    statement("<urn:123><urn:123>1");
}
