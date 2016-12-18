use grammar_structs::*;
use nom::IResult;
use nom::IResult::Done;
use nom::Needed;

/// Take one character if it fits the function
macro_rules! one_if (
  ($i:expr, $f:expr) => (
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

named!(comment<&str,()>, value!((), tuple!(
    tag_s!("#"),
    take_until_either_and_consume!("\r\n")
)));

/// whitespace that my contain comments
fn tws(mut str: &str) -> IResult<&str, ()> {
    loop {
        match comment(str) {
            Done(left, _) => {
                str = left;
            }
            IResult::Error(_) => {}
            IResult::Incomplete(_) => return Done(str, ()),
        }
        match take_while_s!(str, is_ws) {
            Done(left, "") => return Done(left, ()),
            Done(left, _) => {
                str = left;
            }
            IResult::Error(e) => return IResult::Error(e),
            IResult::Incomplete(_) => return Done(str, ()),
        }
    }
}

/// [1] turtleDoc ::= statement*
named!(pub turtle<&str,Vec<Statement>>, do_parse!(
    tws >>
    s: many0!(do_parse!(
        s: statement >>
        tws >>
        (s))) >>
    (s)
));

/// [2] statement ::= directive | triples '.'
named!(statement<&str,Statement>, alt!(statement_triples
        | prefix_id | base | sparql_base | sparql_prefix));

named!(statement_triples<&str,Statement>, do_parse!(
    triples: triples >>
    tws >>
    tag_s!(".") >>
    (Statement::Triples(triples))
));

/// [4] prefixID ::= '@prefix' PNAME_NS IRIREF '.'
named!(prefix_id<&str,Statement>, do_parse!(
    tag_s!("@prefix") >>
    tws >>
    pname_ns: pname_ns >>
    tws >>
    iri_ref: iri_ref >>
    tws >>
    tag_s!(".") >>
    (Statement::Prefix(String::from(pname_ns), iri_ref))
));

/// [5] base ::= '@base' IRIREF '.'
named!(base<&str,Statement>, do_parse!(
    tag_s!("@base") >>
    tws >>
    iri_ref: iri_ref >>
    tws >>
    tag_s!(".") >>
    (Statement::Base(iri_ref))
));

/// [5s] sparqlBase ::= "BASE" IRIREF
named!(sparql_base<&str,Statement>, do_parse!(
    tag_s!("BASE") >>
    tws >>
    iri_ref: iri_ref >>
    (Statement::Base(iri_ref))
));

/// [6s] sparqlPrefix ::= "PREFIX" PNAME_NS IRIREF
named!(sparql_prefix<&str,Statement>, do_parse!(
    tag_s!("PREFIX") >>
    tws >>
    pname_ns: pname_ns >>
    tws >>
    iri_ref: iri_ref >>
    (Statement::Prefix(String::from(pname_ns), iri_ref))
));

/// [6] triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?
named!(triples<&str,Triples>, do_parse!(
    subject: iri >>
    tws >>
    predicated_objects_list: predicated_objects_list >>
    (Triples{
        subject: subject,
        predicated_objects_list: predicated_objects_list
    })
));

/// [7] predicateObjectList ::= verb objectList (';' (verb objectList)?)*
named!(predicated_objects_list<&str,Vec<PredicatedObjects>>,
    separated_nonempty_list!(
        tuple!(
            tws,
            tag_s!(";"),
            tws
        ),
        predicated_objects
    )
);

named!(predicated_objects<&str,PredicatedObjects>, do_parse!(
    verb: verb >>
    tws >>
    objects: object_list >>
    (PredicatedObjects{
        verb:verb,
        objects:objects
    })
));

/// [8] objectList ::= object (',' object)*
named!(object_list<&str,Vec<Object>>, separated_nonempty_list!(
    tuple!(
        tws,
        tag_s!(","),
        tws
    ),
    object
));

/// [9] verb ::= predicate | 'a'
named!(verb<&str,IRI>, alt!(iri|a));

named!(a<&str,IRI>, value!(
    IRI::IRI(String::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")),
    tag_s!("a")
));

/// [10] subject ::= iri | BlankNode | collection

/// [11] predicate ::= iri

/// [12] object ::= iri | BlankNode | collection | blankNodePropertyList | literal
named!(object<&str,Object>, alt!(
    map!(literal, Object::Literal) |
    map!(iri, Object::IRI)
));

/// [13] literal ::= RDFLiteral | NumericLiteral | BooleanLiteral
named!(literal<&str,Literal>, alt!(rdfliteral | boolean | integer));

/// [14] blankNodePropertyList ::= '[' predicateObjectList ']'
/// [15] collection ::= '(' object* ')'
/// [16] NumericLiteral ::= INTEGER | DECIMAL | DOUBLE

/// [128s]  RDFLiteral ::= String (LANGTAG | '^^' iri)?
named!(rdfliteral<&str,Literal>, do_parse!(
    string: string >>
    datatype: opt!(alt!(langtag | iri_ref_literal)) >>
    ({
        match datatype {
            Some(RDFLiteralType::LangTag(langtag)) => {
                Literal::LangString(string, langtag)
            },
            Some(RDFLiteralType::DataType(datatype)) => {
                Literal::TypedLiteral(string, datatype)
            },
            None => {
                Literal::XsdString(string)
            }
        }
    })
));

named!(iri_ref_literal<&str,RDFLiteralType>, do_parse!(
    tag_s!("^^") >>
    iri: iri >>
    (RDFLiteralType::DataType(iri))
));

/// [133s] BooleanLiteral ::= 'true' | 'false'
named!(boolean<&str,Literal>, do_parse!(
    b: alt!(tag_s!("true") | tag_s!("false")) >>
    (Literal::XsdBoolean(b == "true"))
));

/// [17] String ::= STRING_LITERAL_QUOTE | STRING_LITERAL_SINGLE_QUOTE
///                 | STRING_LITERAL_LONG_SINGLE_QUOTE | STRING_LITERAL_LONG_QUOTE
named!(string<&str,String>, alt!(string_literal_quote | string_literal_single_quote
    | string_literal_long_single_quote | string_literal_long_quote));

/// [135s] iri ::= IRIREF | PrefixedName
named!(iri<&str,IRI>, alt!(iri_iri|prefixed_name));

/// [136s]  PrefixedName ::= PNAME_LN | PNAME_NS
named!(prefixed_name<&str,IRI>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    pn_local: opt!(pn_local) >>
    (IRI::PrefixedName(
        String::from(pn_prefix.unwrap_or("")),
        pn_local.unwrap_or(String::new())
    ))
));

/// [137s]  BlankNode ::= BLANK_NODE_LABEL | ANON

/// [18] IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'
/// #x00=NULL #01-#x1F=control codes #x20=space
named!(iri_ref<&str,String>, delimited!(
    tag_s!("<"),
    map!(
        take_while_s!(is_iri_ref),
        String::from),
    tag_s!(">")
));

/// [139s] PNAME_NS ::= PN_PREFIX? ':'
named!(pname_ns<&str,&str>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    (pn_prefix.unwrap_or(""))
));

/// [140s] PNAME_LN ::= PNAME_NS PN_LOCAL
/// [141s] BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
/// [144s] LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*
named!(langtag<&str,RDFLiteralType>, do_parse!(
    tag_s!("@") >>
    langtag: recognize!(tuple!(
        alpha,
        opt!(tuple!(tag_s!("-"), alphanumeric))
    )) >>
    (RDFLiteralType::LangTag(String::from(langtag)))
));

/// [19] INTEGER ::= [+-]? [0-9]+
named!(integer<&str,Literal>, do_parse!(
    sign: opt!(alt!(tag_s!("+") | tag_s!("-"))) >>
    digit: digit >>
    (Literal::XsdInteger({
        let v = i64::from_str_radix(digit, 10).unwrap();
        if sign == Some("-") { -v } else { v }
    }))
));

/// [20] DECIMAL ::= [+-]? [0-9]* '.' [0-9]+
/// [21] DOUBLE ::= [+-]? ([0-9]+ '.' [0-9]* EXPONENT | '.' [0-9]+ EXPONENT | [0-9]+ EXPONENT)
/// [154s] EXPONENT ::= [eE] [+-]? [0-9]+
/// [22] STRING_LITERAL_QUOTE ::= '"' ([^#x22#x5C#xA#xD] | ECHAR | UCHAR)* '"'
/// /* #x22=" #x5C=\ #xA=new line #xD=carriage return */
named!(string_literal_quote<&str,String>, delimited!(
    tag_s!("\""),
    map!(
        take_while_s!(is_string_literal_quote),
        String::from
    ),
    tag_s!("\"")
));

#[inline]
fn is_string_literal_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

/// [23] STRING_LITERAL_SINGLE_QUOTE ::= "'" ([^#x27#x5C#xA#xD] | ECHAR | UCHAR)* "'"
/// /* #x27=' #x5C=\ #xA=new line #xD=carriage return */
named!(string_literal_single_quote<&str,String>, delimited!(
    tag_s!("'"),
    map!(
        take_while_s!(is_string_literal_single_quote),
        String::from
    ),
    tag_s!("'")
));

#[inline]
fn is_string_literal_single_quote(chr: char) -> bool {
    chr != '\'' && chr != '\\' && chr != '\n' && chr != '\r'
}

/// [24] STRING_LITERAL_LONG_SINGLE_QUOTE ::= "'''" (("'" | "''")? ([^'\] | ECHAR | UCHAR))* "'''"
named!(string_literal_long_single_quote<&str,String>, delimited!(
    tag_s!("'''"),
    map!(
        take_while_s!(is_string_literal_long_single_quote),
        String::from
    ),
    tag_s!("'''")
));

#[inline]
fn is_string_literal_long_single_quote(chr: char) -> bool {
    chr != '\'' as char && chr != '\\' && chr != '\n' && chr != '\r'
}

/// [25] STRING_LITERAL_LONG_QUOTE ::= '"""' (('"' | '""')? ([^"\] | ECHAR | UCHAR))* '"""'
named!(string_literal_long_quote<&str,String>, delimited!(
    tag_s!("\"\"\""),
    map!(
        take_while_s!(is_string_literal_long_quote),
        String::from
    ),
    tag_s!("\"\"\"")
));

#[inline]
fn is_string_literal_long_quote(chr: char) -> bool {
    chr != '"' && chr != '\\' && chr != '\n' && chr != '\r'
}

/// [26] UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX
/// [159s] ECHAR ::= '\' [tbnrf"'\]
/// [161s] WS ::= #x20 | #x9 | #xD | #xA
/// /* #x20=space #x9=character tabulation #xD=carriage return #xA=new line */
fn is_ws(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

/// [162s] ANON ::= '[' WS* ']'
/// [163s] PN_CHARS_BASE ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6]
/// | [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D]
/// | [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF]
/// | [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]
fn is_pn_chars_base(c: char) -> bool {
    is_alpha(c) || in_range(c, 0xC0, 0x00D6) || in_range(c, 0x00D8, 0x00F6) ||
    in_range(c, 0x00F8, 0x02FF) || in_range(c, 0x0370, 0x037D) ||
    in_range(c, 0x037F, 0x1FFF) || in_range(c, 0x200C, 0x200D) ||
    in_range(c, 0x2070, 0x218F) || in_range(c, 0x2C00, 0x2FEF) || in_range(c, 0x3001, 0xD7FF) ||
    in_range(c, 0xF900, 0xFDCF) || in_range(c, 0xFDF0, 0xFFFD) || in_range(c, 0x10000, 0xEFFFF)
}

/// [164s] PN_CHARS_U ::= PN_CHARS_BASE | '_'
fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

/// [166s] PN_CHARS ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c) || c == '-' || is_digit(c) || c == 0xB7 as char ||
    in_range(c, 0x0300, 0x036F) || in_range(c, 0x203F, 0x2040)
}

/// [167s] PN_PREFIX ::= PN_CHARS_BASE ((PN_CHARS | '.')* PN_CHARS)?
named!(pn_prefix<&str,&str>, recognize!(tuple!(
    one_if!(is_pn_chars_base),
    take_while_s!(is_pn_chars),
    many0!(tuple!(
        tag_s!("."),
        take_while1_s!(is_pn_chars)
    ))
)));

/// [168s] PN_LOCAL ::= (PN_CHARS_U | ':' | [0-9] | PLX) ((PN_CHARS | '.' | ':'
/// | PLX)* (PN_CHARS | ':' | PLX))?
named!(pn_local<&str,String>, map!(recognize!(tuple!(
    one_if!(is_pn_chars_u),
    take_while_s!(is_alphanum)
)), String::from));

/// [169s] PLX ::= PERCENT | PN_LOCAL_ESC
/// [170s] PERCENT ::= '%' HEX HEX
/// [171s] HEX ::= [0-9] | [A-F] | [a-f]
/// [172s] PN_LOCAL_ESC ::= '\' ('_' | '~' | '.' | '-' | '!' | '$' | '&' | "'"
/// | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%')
// named!(pn_local_esc<&str,&str>, recognize!(tuple!(
//    char!("\\"), one_of!("_~.-!$&'()*+,;=/?#@%"))));

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

#[test]
fn test_prefixed_name() {
    assert_eq!(prefixed_name("a:a"), Done(&""[..],
            IRI::PrefixedName(String::from("a"),String::from("a"))));
    assert_eq!(prefixed_name(": "), Done(&" "[..],
            IRI::PrefixedName(String::new(),String::new())));
}

named!(alpha<&str,&str>, take_while1_s!(is_alpha));
named!(alphanumeric<&str,&str>, take_while1_s!(is_alphanum));
named!(digit<&str,&str>, take_while1_s!(is_digit));

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_iri<&str,IRI>, map!(iri_ref, IRI::IRI));

#[test]
fn test_iri() {
    assert_eq!(iri("<urn:123>"), Done(&""[..],IRI::IRI(String::from("urn:123"))));
}

#[test]
fn test_string_literal_quote() {
    assert_eq!(string_literal_quote("\"\""), Done(&""[..], String::new()));
}

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(string_literal_single_quote("''"), Done(&""[..], String::new()));
}

#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(string_literal_long_single_quote("''''''"), Done(&""[..], String::new()));
}

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(string_literal_long_quote("\"\"\"\"\"\""), Done(&""[..], String::new()));
}

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Done(&" "[..],
            RDFLiteralType::LangTag(String::from("nl"))));
    assert_eq!(langtag("@nl-NL "), Done(&" "[..],
            RDFLiteralType::LangTag(String::from("nl-NL"))));
}

#[test]
fn test_rdfliteral() {
    let r = Literal::XsdString(String::new());
    assert_eq!(rdfliteral("'' "), Done(&" "[..], r));
}

#[test]
fn test_integer() {
    assert_eq!(integer("1"), Done(&""[..], Literal::XsdInteger(1)));
    assert_eq!(integer("+1"), Done(&""[..], Literal::XsdInteger(1)));
    assert_eq!(integer("-1"), Done(&""[..], Literal::XsdInteger(-1)));
}

#[test]
fn test_boolean() {
    assert_eq!(boolean("true"), Done(&""[..], Literal::XsdBoolean(true)));
    assert_eq!(boolean("false"), Done(&""[..], Literal::XsdBoolean(false)));
}

#[test]
fn test_literal() {
    assert_eq!(literal("true"), Done(&""[..], Literal::XsdBoolean(true)));
    assert_eq!(literal("false"), Done(&""[..], Literal::XsdBoolean(false)));
}

#[test]
fn test_object_list() {
    let v = vec![
    Object::Literal(Literal::XsdBoolean(true)),
    Object::Literal(Literal::XsdInteger(1)),
    Object::Literal(Literal::XsdBoolean(false))];
    assert_eq!(object_list("true, 1 , false"), Done(&""[..],v));
}

#[test]
fn test_predicated_objects() {
    let v = vec![Object::Literal(Literal::XsdInteger(1))];
    let po = PredicatedObjects {
        verb: IRI::IRI(String::from("urn:123")),
        objects: v,
    };
    assert_eq!(predicated_objects("<urn:123> 1"), Done(&""[..],po));
}

#[test]
fn test_triples() {
    let v = vec![Object::Literal(Literal::XsdInteger(1))];
    let i = IRI::IRI(String::from("urn:123"));
    let po = vec![PredicatedObjects{verb:i.clone(),objects:v}];
    let t = Triples {
        subject: i,
        predicated_objects_list: po,
    };
    assert_eq!(triples("<urn:123> <urn:123> 1"), Done(&""[..],t));
}

#[test]
fn test_statement_triples() {
    let i = IRI::PrefixedName(String::new(), String::new());
    let po = vec![PredicatedObjects{verb:i.clone(),objects:vec![Object::IRI(i.clone())]}];
    let t = Triples {
        subject: i,
        predicated_objects_list: po,
    };
    let s = Statement::Triples(t);
    assert_eq!(statement_triples(": : :."), Done(&""[..],s));
}

#[test]
fn test_prefix_id() {
    assert_eq!(prefix_id("@prefix a.b.c: <urn> ."),
        Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
    assert_eq!(prefix_id("@prefix : <urn> ."),
        Done(&""[..],Statement::Prefix(String::from(""),String::from("urn"))));
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
    assert_eq!(sparql_prefix("PREFIX a.b.c: <urn>"),
        Done(&""[..],Statement::Prefix(String::from("a.b.c"),String::from("urn"))));
}
