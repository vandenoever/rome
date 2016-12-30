use grammar_structs::*;
use grammar_helper::*;
use nom::IResult;
use nom::IResult::Done;
use nom::{Needed, ErrorKind};

/// Take one character if it fits the function
macro_rules! one_if (
  ($i:expr, $f:expr) => (
    {
      if let Some(c) =  $i.chars().next() {
        if $f(c) {
          IResult::Done(&$i[1..], &$i[..1])
        } else {
          IResult::Error(error_position!($crate::ErrorKind::OneOf, $i))
        }
      } else {
        IResult::Incomplete::<_, _>(Needed::Size(1))
      }
    }
  );
);

pub const RDF_LANG_STRING: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
const XSD_STRING: &'static str = "http://www.w3.org/2001/XMLSchema#string";
const XSD_BOOLEAN: &'static str = "http://www.w3.org/2001/XMLSchema#boolean";
const XSD_DECIMAL: &'static str = "http://www.w3.org/2001/XMLSchema#decimal";
const XSD_DOUBLE: &'static str = "http://www.w3.org/2001/XMLSchema#double";
const XSD_INTEGER: &'static str = "http://www.w3.org/2001/XMLSchema#integer";
const RDF_TYPE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

fn comment(str: &str) -> IResult<&str, ()> {
    if str.len() == 0 {
        return IResult::Incomplete(Needed::Size(2));
    }
    if &str[0..1] != "#" {
        return IResult::Error(ErrorKind::Custom(0));
    }
    if let Some(pos) = str.find(|c| c == '\n' || c == '\r') {
        IResult::Done(&str[pos + 1..], ())
    } else {
        IResult::Incomplete(Needed::Size(1))
    }
}

/// whitespace that may contain comments
pub fn tws(mut str: &str) -> IResult<&str, ()> {
    let mut last_len = str.len();
    loop {
        match comment(str) {
            Done(left, _) => {
                str = left;
            }
            _ => {}
        }
        match take_while_s!(str, is_ws) {
            Done(left, _) => {
                str = left;
            }
            _ => {}
        }
        if str.len() == last_len {
            return Done(str, ());
        }
        last_len = str.len();
    }
}

/// [2] statement ::= directive | triples '.'
/// [3] directive ::= prefixID | base | sparqlPrefix | sparqlBase
named!(pub statement<&str,Statement>, alt!(prefix_id | base | sparql_prefix
        | sparql_base | statement_triples));

named!(statement_triples<&str,Statement>, do_parse!(
    triples: triples >> tws >>
    tag_s!(".") >>
    (Statement::Triples(triples))
));

/// [4] prefixID ::= '@prefix' PNAME_NS IRIREF '.'
named!(prefix_id<&str,Statement>, do_parse!(
    tag_s!("@prefix") >> tws >>
    pname_ns: pname_ns >> tws >>
    iri_ref: iri_ref >> tws >>
    tag_s!(".") >>
    (Statement::Prefix(pname_ns, iri_ref))
));

/// [5] base ::= '@base' IRIREF '.'
named!(base<&str,Statement>, do_parse!(
    tag_s!("@base") >> tws >>
    iri_ref: iri_ref >> tws >>
    tag_s!(".") >>
    (Statement::Base(iri_ref))
));

/// [5s] sparqlBase ::= "BASE" IRIREF
named!(sparql_base<&str,Statement>, do_parse!(
    tag_no_case_s!("BASE") >> tws >>
    iri_ref: iri_ref >>
    (Statement::Base(iri_ref))
));

/// [6s] sparqlPrefix ::= "PREFIX" PNAME_NS IRIREF
named!(sparql_prefix<&str,Statement>, do_parse!(
    tag_no_case_s!("PREFIX") >> tws >>
    pname_ns: pname_ns >> tws >>
    iri_ref: iri_ref >>
    (Statement::Prefix(pname_ns, iri_ref))
));

/// [6] triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?
named!(triples<&str,Triples>, alt!(triples_subject | triples_blank));

named!(triples_subject<&str,Triples>, do_parse!(
    subject: subject >> tws >>
    predicated_objects_list: predicated_objects_list >>
    (Triples{
        subject: subject,
        predicated_objects_list: predicated_objects_list
    })
));

fn triples_blank(str: &str) -> IResult<&str, Triples> {
    match blank_node_property_list(str) {
        Done(mut left, mut blank) => {
            match tws(left) {
                Done(l, _) => {
                    match predicated_objects_list(l) {
                        Done(l, mut pol) => {
                            left = l;
                            blank.append(&mut pol);
                        }
                        IResult::Incomplete(i) => return IResult::Incomplete(i),
                        _ => {}
                    }
                }
                _ => {}
            }
            let t = Triples {
                subject: Subject::BlankNode(BlankNode::Anon),
                predicated_objects_list: blank,
            };
            Done(left, t)
        }
        IResult::Error(e) => IResult::Error(e),
        IResult::Incomplete(i) => IResult::Incomplete(i),
    }
}

/// [7] predicateObjectList ::= verb objectList (';' (verb objectList)?)*
fn predicated_objects_list(mut str: &str) -> IResult<&str, Vec<PredicatedObjects>> {
    let mut v = Vec::new();
    match tws(str) {
        Done(left, _) => {
            str = left;
        }
        _ => {}
    }
    match predicated_object(str) {
        Done(left, po) => {
            v.push(po);
            str = left;
        }
        IResult::Error(e) => return IResult::Error(e),
        IResult::Incomplete(n) => return IResult::Incomplete(n),
    }
    loop {
        match predicated_object_sep(str) {
            Done(left, _) => {
                str = left;
            }
            _ => return Done(str, v),
        }
        match predicated_object(str) {
            Done(left, po) => {
                v.push(po);
                str = left;
            }
            _ => return Done(str, v),
        }
    }
}

named!(predicated_object_sep<&str,()>,
    fold_many1!(tuple!(tws, tag_s!(";"), tws),(),|_,_|())
);

named!(predicated_object<&str,PredicatedObjects>, do_parse!(
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
    tuple!(tws, tag_s!(","), tws),
    object
));

/// [9] verb ::= predicate | 'a'
named!(verb<&str,IRI>, alt!(iri|a));

named!(a<&str,IRI>, value!(
    IRI::IRI(RDF_TYPE),
    tag_s!("a")
));

/// [10] subject ::= iri | BlankNode | collection
named!(subject<&str,Subject>, alt!(
    map!(iri, Subject::IRI) |
    map!(blank_node, Subject::BlankNode) |
    map!(collection, Subject::Collection)
));

/// [11] predicate ::= iri

/// [12] object ::= iri | BlankNode | collection | blankNodePropertyList | literal
named!(object<&str,Object>, alt!(
    map!(literal, Object::Literal) |
    map!(iri, Object::IRI) |
    map!(blank_node, Object::BlankNode) |
    map!(collection, Object::Collection) |
    map!(blank_node_property_list, Object::BlankNodePropertyList)
));

/// [13] literal ::= RDFLiteral | NumericLiteral | BooleanLiteral
named!(literal<&str,Literal>, alt!(rdfliteral | boolean | double | decimal | integer));

/// [14] blankNodePropertyList ::= '[' predicateObjectList ']'
named!(blank_node_property_list<&str,Vec<PredicatedObjects>>, do_parse!(
    tag_s!("[") >> tws >>
    pol: predicated_objects_list >> tws >>
    tag_s!("]") >> (pol)
));

/// [15] collection ::= '(' object* ')'
named!(collection<&str,Vec<Object>>, do_parse!(
    tag_s!("(") >> tws >>
    objects: many0!(do_parse!(
        object: object >> tws >>
        (object))) >>
    tag_s!(")") >> (objects)
));

/// [16] NumericLiteral ::= INTEGER | DECIMAL | DOUBLE

/// [128s]  RDFLiteral ::= String (LANGTAG | '^^' iri)?
named!(rdfliteral<&str,Literal>, do_parse!(
    string: string >>
    datatype: opt!(alt!(langtag | iri_ref_literal)) >>
    ({
        match datatype {
            Some(RDFLiteralType::LangTag(langtag)) => {
                Literal {
                    lexical: string,
                    datatype: IRI::IRI(RDF_LANG_STRING),
                    language: Some(langtag)
                }
            },
            Some(RDFLiteralType::DataType(datatype)) => {
                Literal {
                    lexical: string,
                    datatype: datatype,
                    language: None
                }
            },
            None => {
                Literal {
                    lexical: string,
                    datatype: IRI::IRI(XSD_STRING),
                    language: None
                }
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
    (Literal {
        lexical: b,
        datatype: IRI::IRI(XSD_BOOLEAN),
        language: None
    })
));

/// [17] String ::= STRING_LITERAL_QUOTE | STRING_LITERAL_SINGLE_QUOTE
///                 | STRING_LITERAL_LONG_SINGLE_QUOTE | STRING_LITERAL_LONG_QUOTE
named!(string<&str,&str>, alt!(string_literal_long_single_quote
    | string_literal_long_quote | string_literal_quote
    | string_literal_single_quote));

/// [135s] iri ::= IRIREF | PrefixedName
named!(iri<&str,IRI>, alt!(iri_iri|prefixed_name));

/// [136s]  PrefixedName ::= PNAME_LN | PNAME_NS
named!(prefixed_name<&str,IRI>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    pn_local: opt!(pn_local) >>
    (IRI::PrefixedName(
        pn_prefix.unwrap_or(""),
        pn_local.unwrap_or("")
    ))
));

/// [137s]  BlankNode ::= BLANK_NODE_LABEL | ANON
named!(blank_node<&str,BlankNode>, alt!(blank_node_label | anon));

/// [18] IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'
/// #x00=NULL #01-#x1F=control codes #x20=space
named!(iri_ref<&str,&str>, delimited!(
    tag_s!("<"),take_while_s!(is_iri_ref),tag_s!(">")
));

/// [139s] PNAME_NS ::= PN_PREFIX? ':'
named!(pname_ns<&str,&str>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    (pn_prefix.unwrap_or(""))
));

/// [140s] PNAME_LN ::= PNAME_NS PN_LOCAL
/// see prefixed_name

/// [141s] BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?
named!(blank_node_label<&str,BlankNode>, do_parse!(
    tag!("_:") >>
    label: recognize!(tuple!(
        one_if!(is_pn_chars_u_digit),
        take_while_s!(is_pn_chars),
        fold_many0!(tuple!(
            tag!("."),
            take_while_s!(is_pn_chars)
        ),(),|_,_|())
    )) >> (BlankNode::BlankNode(label))
));

fn is_pn_chars_u_digit(c: char) -> bool {
    is_digit(c) || is_pn_chars_u(c)
}

/// [144s] LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*
named!(langtag<&str,RDFLiteralType>, do_parse!(
    tag_s!("@") >>
    langtag: recognize!(tuple!(
        alpha,
        opt!(tuple!(tag_s!("-"), alphanumeric))
    )) >>
    (RDFLiteralType::LangTag(langtag))
));

/// [19] INTEGER ::= [+-]? [0-9]+
named!(integer<&str,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))), digit)),
    (|integer|{
        Literal {
            lexical: integer,
            datatype: IRI::IRI(XSD_INTEGER),
            language: None
        }
    })
));

/// [20] DECIMAL ::= [+-]? [0-9]* '.' [0-9]+
named!(decimal<&str,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))), opt_digit, tag_s!("."), digit)),
    (|decimal|{
        Literal {
            lexical: decimal,
            datatype: IRI::IRI(XSD_DECIMAL),
            language: None
        }
    })
));

/// [21] DOUBLE ::= [+-]? ([0-9]+ '.' [0-9]* EXPONENT | '.' [0-9]+ EXPONENT | [0-9]+ EXPONENT)
named!(double<&str,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))),
    alt!(
        recognize!(tuple!(digit,tag_s!("."), opt_digit, exponent)) |
        recognize!(tuple!(opt!(tag_s!(".")), digit, exponent))
    ))),
    (|double|{
        Literal {
            lexical: double,
            datatype: IRI::IRI(XSD_DOUBLE),
            language: None
        }
    })
));

/// [154s] EXPONENT ::= [eE] [+-]? [0-9]+
named!(exponent<&str,()>, map!(tuple!(
    alt!(tag_s!("E") | tag_s!("e")),opt!(alt!(tag_s!("+") | tag_s!("-"))), digit),
    (|_|())
));

/// [22] STRING_LITERAL_QUOTE ::= '"' ([^#x22#x5C#xA#xD] | ECHAR | UCHAR)* '"'
/// /* #x22=" #x5C=\ #xA=new line #xD=carriage return */
fn string_literal_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 1, start_quote, find_quote)
}
fn start_quote(s: &str) -> bool {
    s.starts_with('"')
}
fn find_quote(s: &str) -> Option<usize> {
    s.find('"')
}

/// [23] STRING_LITERAL_SINGLE_QUOTE ::= "'" ([^#x27#x5C#xA#xD] | ECHAR | UCHAR)* "'"
/// /* #x27=' #x5C=\ #xA=new line #xD=carriage return */
fn string_literal_single_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 1, start_single_quote, find_single_quote)
}
fn start_single_quote(s: &str) -> bool {
    s.starts_with('\'')
}
fn find_single_quote(s: &str) -> Option<usize> {
    s.find('\'')
}

/// [24] STRING_LITERAL_LONG_SINGLE_QUOTE ::= "'''" (("'" | "''")? ([^'\] | ECHAR | UCHAR))* "'''"
fn string_literal_long_single_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 3, start_long_single_quote, find_long_single_quote)
}
fn start_long_single_quote(s: &str) -> bool {
    s.starts_with("'''")
}
fn find_long_single_quote(s: &str) -> Option<usize> {
    s.find("'''")
}

/// [25] STRING_LITERAL_LONG_QUOTE ::= '"""' (('"' | '""')? ([^"\] | ECHAR | UCHAR))* '"""'
fn string_literal_long_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 3, start_long_quote, find_long_quote)
}
fn start_long_quote(s: &str) -> bool {
    s.starts_with("\"\"\"")
}
fn find_long_quote(s: &str) -> Option<usize> {
    s.find("\"\"\"")
}

/// [26] UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX
/// [159s] ECHAR ::= '\' [tbnrf"'\]

/// [161s] WS ::= #x20 | #x9 | #xD | #xA
/// /* #x20=space #x9=character tabulation #xD=carriage return #xA=new line */
fn is_ws(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

/// [162s] ANON ::= '[' WS* ']'
named!(anon<&str,BlankNode>, do_parse!(
    tag_s!("[") >>
    tws >>
    tag_s!("]") >> (BlankNode::Anon)
));

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
    fold_many0!(tuple!(
        tag_s!("."),
        take_while1_s!(is_pn_chars)
    ),(),|_,_|())
)));

/// [168s] PN_LOCAL ::= (PN_CHARS_U | ':' | [0-9] | PLX)
///           ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX))?
named!(pn_local<&str,&str>, recognize!(tuple!(
    alt!(one_if!(is_pn_local_start) | plx),
    fold_many0!(alt!(pn_chars_colon | plx),(),|_,_|()),
    fold_many0!(tuple!(
        tag_s!("."),
        fold_many0!(alt!(pn_chars_colon | plx),(),|_,_|())
    ),(),|_,_|())
)));

named!(pn_chars_colon<&str,&str>, take_while1_s!(is_pn_chars_colon));

fn is_pn_local_start(c: char) -> bool {
    c == ':' || is_digit(c) || is_pn_chars_u(c)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

/// [169s] PLX ::= PERCENT | PN_LOCAL_ESC
named!(plx<&str,&str>, alt!(percent | pn_local_esc));

/// [170s] PERCENT ::= '%' HEX HEX
/// [171s] HEX ::= [0-9] | [A-F] | [a-f]
named!(percent<&str,&str>, recognize!(tuple!(
    tag_s!("%"),
    one_if!(is_hex),
    one_if!(is_hex)
)));

/// [172s] PN_LOCAL_ESC ::= '\' ('_' | '~' | '.' | '-' | '!' | '$' | '&' | "'"
/// | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%')
named!(pn_local_esc<&str,&str>, recognize!(tuple!(
    tag_s!("\\"),
    one_if!(|c| "_~.-!$&'()*+,;=/?#@%".contains(c))
)));

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_hex(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

fn in_range(c: char, lower: u32, upper: u32) -> bool {
    c as u32 >= lower && c as u32 <= upper
}

#[test]
fn test_comment() {
    assert_eq!(comment("#\r\na"), Done(&"\na"[..],()));
    assert_eq!(comment("#\n\ra"), Done(&"\ra"[..],()));
    assert_eq!(comment(""), IResult::Incomplete(Needed::Size(2)));
    assert_eq!(comment("#"), IResult::Incomplete(Needed::Size(1)));
}

#[test]
fn test_prefixed_name() {
    assert_eq!(prefixed_name("a:a"), Done(&""[..],
            IRI::PrefixedName("a","a")));
    assert_eq!(prefixed_name(": "), Done(&" "[..],
            IRI::PrefixedName("","")));
}

named!(alpha<&str,&str>, take_while1_s!(is_alpha));
named!(alphanumeric<&str,&str>, take_while1_s!(is_alphanum));
named!(digit<&str,&str>, take_while1_s!(is_digit));
named!(opt_digit<&str,&str>, take_while_s!(is_digit));

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_iri<&str,IRI>, map!(iri_ref, |v| IRI::IRI(v)));

#[test]
fn test_iri() {
    assert_eq!(iri("<urn:123>"), Done(&""[..],IRI::IRI("urn:123")));
}

#[test]
fn test_string_literal_quote() {
    assert_eq!(string_literal_quote("\"\\\\\""), Done(&""[..], "\\\\"));
}

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(string_literal_single_quote("''"), Done(&""[..], ""));
}

#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(string_literal_long_single_quote("''''''"), Done(&""[..], ""));
}

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(string_literal_long_quote("\"\"\"\\U0001f435\"\"\""), Done(&""[..],
            "\\U0001f435"));
    assert_eq!(string_literal_long_quote("\"\"\"first long literal\"\"\""), Done(&""[..],
            "first long literal"));
}

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Done(&" "[..],
            RDFLiteralType::LangTag("nl")));
    assert_eq!(langtag("@nl-NL "), Done(&" "[..],
            RDFLiteralType::LangTag("nl-NL")));
}

#[test]
fn test_rdfliteral() {
    let r = Literal {
        lexical: "",
        datatype: IRI::IRI(XSD_STRING),
        language: None,
    };
    assert_eq!(rdfliteral("'' "), Done(&" "[..], r));
}
#[cfg(test)]
fn literal_true<'a>() -> Literal<'a> {
    Literal {
        lexical: "true",
        datatype: IRI::IRI(XSD_BOOLEAN),
        language: None,
    }
}
#[cfg(test)]
fn literal_false<'a>() -> Literal<'a> {
    Literal {
        lexical: "false",
        datatype: IRI::IRI(XSD_BOOLEAN),
        language: None,
    }
}
#[cfg(test)]
fn literal_11<'a>() -> Literal<'a> {
    Literal {
        lexical: "11",
        datatype: IRI::IRI(XSD_INTEGER),
        language: None,
    }
}
#[cfg(test)]
fn literal_d11<'a>() -> Literal<'a> {
    Literal {
        lexical: "11.1",
        datatype: IRI::IRI(XSD_DECIMAL),
        language: None,
    }
}

#[test]
fn test_integer() {
    assert_eq!(literal("11 "), Done(&" "[..], literal_11()));
    assert_eq!(literal("+1 "), Done(&" "[..], Literal{
            lexical: "+1",
            datatype: IRI::IRI(XSD_INTEGER),
            language: None}));
    assert_eq!(integer("-1 "), Done(&" "[..], Literal{
     lexical: "-1",
     datatype: IRI::IRI(XSD_INTEGER),
     language: None}));
}

#[test]
fn test_decimal() {
    assert_eq!(literal("11.1 "), Done(&" "[..], literal_d11()));
    assert_eq!(literal("+1.1 "), Done(&" "[..], Literal{
            lexical: "+1.1",
            datatype: IRI::IRI(XSD_DECIMAL),
            language: None}));
    assert_eq!(literal("-1.1 "), Done(&" "[..], Literal{
     lexical: "-1.1",
     datatype: IRI::IRI(XSD_DECIMAL),
     language: None}));
    assert_eq!(literal(".1 "), Done(&" "[..], Literal{
     lexical: ".1",
     datatype: IRI::IRI(XSD_DECIMAL),
     language: None}));
}

#[test]
fn test_boolean() {
    assert_eq!(boolean("true"), Done(&""[..], literal_true()));
    assert_eq!(boolean("false"), Done(&""[..], literal_false()));
}

#[test]
fn test_literal() {
    assert_eq!(literal("true"), Done(&""[..], literal_true()));
    assert_eq!(literal("false"), Done(&""[..], literal_false()));
}

#[test]
fn test_object() {
    assert_eq!(object("_:b1 "), Done(&" "[..], Object::BlankNode(BlankNode::BlankNode("b1"))));
    let long = Object::Literal(Literal {
        lexical: "first long literal",
        datatype: IRI::IRI(XSD_STRING),
        language: None,
    });
    assert_eq!(object("\"\"\"first long literal\"\"\" "), Done(&" "[..], long));
}

#[test]
fn test_blank_node_label() {
    assert_eq!(blank_node_label("_:b1 "), Done(&" "[..], BlankNode::BlankNode("b1")));
}

#[test]
fn test_object_list() {
    let v = vec![
    Object::Literal(literal_true()),
    Object::Literal(literal_11()),
    Object::Literal(literal_false())];
    assert_eq!(object_list("true, 11 , false"), Done(&""[..],v));
}

#[test]
fn test_predicated_objects() {
    let v = vec![Object::Literal(Literal{
            lexical: "1",
            datatype: IRI::IRI(XSD_INTEGER),
            language: None})];
    let po = PredicatedObjects {
        verb: IRI::IRI("urn:123"),
        objects: v,
    };
    assert_eq!(predicated_objects_list("<urn:123> 1 "), Done(&" "[..],vec![po]));
}

#[test]
fn test_triples() {
    let v = vec![Object::Literal(Literal{
            lexical: "1",
            datatype: IRI::IRI(XSD_INTEGER),
            language: None})];
    let i = IRI::IRI("urn:123");
    let s = Subject::IRI(i.clone());
    let po = vec![PredicatedObjects{verb:i.clone(),objects:v}];
    let t = Triples {
        subject: s,
        predicated_objects_list: po,
    };
    assert_eq!(triples("<urn:123> <urn:123> 1 "), Done(&" "[..],t));
}

#[test]
fn test_statement_triples() {
    let i = IRI::PrefixedName("", "");
    let s = Subject::IRI(i.clone());
    let po = vec![PredicatedObjects{verb:i.clone(),objects:vec![Object::IRI(i.clone())]}];
    let t = Triples {
        subject: s,
        predicated_objects_list: po,
    };
    let s = Statement::Triples(t);
    assert_eq!(statement_triples(": : :."), Done(&""[..],s));
}

#[test]
fn test_prefix_id() {
    assert_eq!(prefix_id("@prefix a.b.c: <urn> ."),
        Done(&""[..],Statement::Prefix("a.b.c","urn")));
    assert_eq!(prefix_id("@prefix : <urn> ."),
        Done(&""[..],Statement::Prefix("","urn")));
}

#[test]
fn test_base() {
    assert_eq!(base("@base <urn> ."), Done(&""[..],Statement::Base("urn")));
}

#[test]
fn test_sparql_base() {
    assert_eq!(sparql_base("BASE <urn>"), Done(&""[..],Statement::Base("urn")));
}

#[test]
fn test_sparql_prefix() {
    assert_eq!(sparql_prefix("PREFIX a.b.c: <urn>"),
        Done(&""[..],Statement::Prefix("a.b.c","urn")));
}
