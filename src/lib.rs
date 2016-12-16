#[macro_use]
extern crate nom;
use nom::*;
use nom::IResult::*;

fn is_alpha(a: char) -> bool {
  (a >= 'a' && a <= 'z') || (a >= 'A' && a <= 'Z')
}

fn is_alphanum(a: char) -> bool {
  (a >= 'a' && a <= 'z') || (a >= 'A' && a <= 'Z') || (a >= '0' && a <= '9')
}

named!(alpha<&str,&str>, take_while!(is_alpha));
named!(alphanumeric<&str,&str>, take_while!(is_alphanum));

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

named!(langtag<&str,String>, do_parse!(
  tag!("@") >>
  langtag: alpha >>
  opt!(langtag_suffix) >>
  (String::from(langtag))
));

named!(langtag_suffix<&str,&str>,
       do_parse!(tag!("-") >> langtag: alphanumeric >> (langtag)));

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Done(&" "[..], String::from("nl")));
}

#[derive(Debug,PartialEq,Eq)]
struct RDFLiteral {
  string: String,
  langtag: Option<String>
}

named!(rdfliteral<&str,RDFLiteral>, do_parse!(
  string: string >>
  langtag: opt!(langtag) >>
  (RDFLiteral{string:string,langtag:langtag})
));

#[test]
fn test_rdfliteral() {
    let r = RDFLiteral{string:String::from(""),langtag:None};
    assert_eq!(rdfliteral("'' "), Done(&" "[..], r));
}

pub fn parse() {
    rdfliteral("''");
}
