#[macro_use]
extern crate nom;
use nom::*;
use nom::IResult::*;

fn is_alpha(a: char) -> bool {
  (a >= 'a' && a <= 'z') || (a >= 'A' && a <= 'Z')
}

fn is_alphanum(a: char) -> bool {
  is_alpha(a) || is_digit(a)
}

fn is_digit(a: char) -> bool {
  a >= '0' && a <= '9'
}

named!(alpha<&str,&str>, take_while!(is_alpha));
named!(alphanumeric<&str,&str>, take_while!(is_alphanum));
named!(digit<&str,&str>, take_while!(is_digit));

// IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_ref<&str,RDFLiteralType>,
  delimited!(
    tag!("<"),
    map!(
      take_while!(is_iri_ref),
      |i| (RDFLiteralType::DataType(String::from(i)))),
    tag!(">")
  )
);

#[inline]
fn is_string_literal_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

named!(string_literal_quote<&str,String>,
  delimited!(
    tag!("\""),
    map!(
      take_while!(is_string_literal_quote),
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
      take_while!(is_string_literal_single_quote),
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
      take_while!(is_string_literal_long_single_quote),
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
      take_while!(is_string_literal_long_quote),
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

named!(rdfliteral<&str,Literal>, do_parse!(
  string: string >>
  data_type: opt!(alt!(langtag | iri_ref)) >>
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

pub fn parse() {
    literal("true");
}
