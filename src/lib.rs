mod turtle;
use turtle::*;
#[macro_use]
extern crate nom;
use nom::*;
use nom::IResult::*;

pub fn parse_string_with_lalrpop(data: &str) {
    let r = parse_LangTag(data);
    println!("{:?}", r);
}

pub fn run() {
    let data = "@nl";
    parse_string_with_lalrpop(data);
}

#[test]
fn test_langtag() {
    assert_eq!(parse_LangTag("@nl").unwrap(), "nl");
    assert!(parse_LangTag("@nl-NL").is_ok());
}

named!(alpha, take_while!(is_alphabetic));
named!(alphanumeric, take_while!(is_alphanumeric));

named!(langtag,
       do_parse!(tag!("@") >> langtag: alpha >> opt!(langtag_suffix) >> (langtag)));

#[inline]
fn is_string_literal_quote(chr: u8) -> bool {
    chr != 0x22 && chr != 0x5C && chr != 0xA && chr != 0xD
}

named!(string_literal_quote,
       delimited!(tag!("\""), take_while!(is_string_literal_quote), tag!("\"")));

named!(langtag_suffix,
       do_parse!(tag!("-") >> langtag: alphanumeric >> (langtag)));

// named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));
named!(parens, ws!(delimited!(tag!("("), is_not!(")"), tag!(")"))));

#[test]
fn test_string_literal_quote() {

    assert_eq!(string_literal_quote(b"\"\""), Done(&b""[..], &b""[..]));
}

#[test]
fn test_langtag_nom() {
    assert_eq!(langtag(b"@nl "), Done(&b" "[..], &b"nl"[..]));
    assert_eq!(parens(b"(hi)"), Done(&b""[..], &b"hi"[..]));
    assert_eq!(parens(b" (hi) "), Done(&b""[..], &b"hi"[..]));
}
