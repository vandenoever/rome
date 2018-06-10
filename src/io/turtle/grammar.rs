#![cfg_attr(feature = "cargo-clippy", allow(redundant_closure_call))]
use super::grammar_helper::*;
use super::grammar_structs::*;
use constants;
use nom::types::CompleteStr;
use nom::{Err, ErrorKind, IResult, Needed};

/// Take one character if it fits the function
macro_rules! one_if (
  ($i:expr, $f:expr) => (
    {
      if let Some(c) =  $i.chars().next() {
        if $f(c) {
          Ok((CompleteStr(&$i[1..]), CompleteStr(&$i[..1])))
        } else {
          Err(::nom::Err::Error(error_position!($i, ErrorKind::OneOf)))
        }
      } else {
        Err(::nom::Err::Incomplete::<_, _>(Needed::Size(1)))
      }
    }
  );
);

fn not_eol(c: char) -> bool {
    c != '\n' && c != '\r'
}

named!(comment<CompleteStr,&str>, do_parse!(
    char!('#') >>
    comment: take_while!(not_eol) >>
    alt!(tag!("\n") | tag!("\r") | eof!()) >>
    (&comment)
));

/// whitespace that may contain comments
pub fn tws(mut str: CompleteStr) -> IResult<CompleteStr, ()> {
    let mut last_len = str.len();
    loop {
        if let Ok((left, _)) = comment(str) {
            str = left;
        }
        if let Ok((left, _)) = take_while_s!(str, is_ws) {
            str = left;
        }
        if str.len() == last_len {
            return Ok((str, ()));
        }
        last_len = str.len();
    }
}

/// [2] `statement ::= directive | triples '.'`
/// [3] `directive ::= prefixID | base | sparqlPrefix | sparqlBase`
named!(pub statement<CompleteStr,Statement>, alt!(statement_triples
		| prefix_id | base | sparql_prefix | sparql_base));

named!(statement_triples<CompleteStr,Statement>, do_parse!(
    triples: triples >> tws >>
    tag_s!(".") >>
    (Statement::Triples(triples))
));

/// [4] `prefixID ::= '@prefix' PNAME_NS IRIREF '.'`
named!(prefix_id<CompleteStr,Statement>, do_parse!(
    tag_s!("@prefix") >> tws >>
    pname_ns: pname_ns >> tws >>
    iri_ref: iri_ref >> tws >>
    tag_s!(".") >>
    (Statement::Prefix(pname_ns.0, iri_ref.0))
));

/// [5] `base ::= '@base' IRIREF '.'`
named!(base<CompleteStr,Statement>, do_parse!(
    tag_s!("@base") >> tws >>
    iri_ref: iri_ref >> tws >>
    tag_s!(".") >>
    (Statement::Base(iri_ref.0))
));

/// [5s] `sparqlBase ::= "BASE" IRIREF`
named!(sparql_base<CompleteStr,Statement>, do_parse!(
    tag_no_case_s!("BASE") >> tws >>
    iri_ref: iri_ref >>
    (Statement::Base(iri_ref.0))
));

/// [6s] `sparqlPrefix ::= "PREFIX" PNAME_NS IRIREF`
named!(sparql_prefix<CompleteStr,Statement>, do_parse!(
    tag_no_case_s!("PREFIX") >> tws >>
    pname_ns: pname_ns >> tws >>
    iri_ref: iri_ref >>
    (Statement::Prefix(pname_ns.0, iri_ref.0))
));

/// [6] `triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?`
named!(triples<CompleteStr,Triples>, alt!(triples_subject | triples_blank));

named!(triples_subject<CompleteStr,Triples>, do_parse!(
    subject: subject >> tws >>
    predicated_objects_list: predicated_objects_list >>
    (Triples{
        subject,
        predicated_objects_list
    })
));

fn triples_blank(str: CompleteStr) -> IResult<CompleteStr, Triples> {
    match blank_node_property_list(str) {
        Ok((mut left, mut blank)) => {
            if let Ok((l, _)) = tws(left) {
                match predicated_objects_list(l) {
                    Ok((l, mut pol)) => {
                        left = l;
                        blank.append(&mut pol);
                    }
                    Err(Err::Incomplete(i)) => return Err(Err::Incomplete(i)),
                    _ => {}
                }
            }
            let t = Triples {
                subject: Subject::BlankNode(BlankNode::Anon),
                predicated_objects_list: blank,
            };
            Ok((left, t))
        }
        Err(e) => Err(e),
    }
}

/// [7] `predicateObjectList ::= verb objectList (';' (verb objectList)?)*`
fn predicated_objects_list(mut str: CompleteStr) -> IResult<CompleteStr, Vec<PredicatedObjects>> {
    let mut v = Vec::new();
    if let Ok((left, _)) = tws(str) {
        str = left;
    }
    match predicated_object(str) {
        Ok((left, po)) => {
            v.push(po);
            str = left;
        }
        Err(e) => return Err(e),
    }
    loop {
        match predicated_object_sep(str) {
            Ok((left, _)) => {
                str = left;
            }
            _ => return Ok((str, v)),
        }
        match predicated_object(str) {
            Ok((left, po)) => {
                v.push(po);
                str = left;
            }
            _ => return Ok((str, v)),
        }
    }
}

named!(predicated_object_sep<CompleteStr,()>,
    fold_many1!(tuple!(tws, tag_s!(";"), tws),(),|_,_|())
);

named!(predicated_object<CompleteStr,PredicatedObjects>, do_parse!(
    verb: verb >>
    tws >>
    objects: object_list >>
    (PredicatedObjects{
        verb,
        objects
    })
));

/// [8] `objectList ::= object (',' object)*`
named!(object_list<CompleteStr,Vec<Object> >, separated_nonempty_list!(
    tuple!(tws, tag_s!(","), tws),
    object
));

/// [9] `verb ::= predicate | 'a'`
named!(verb<CompleteStr,IRI>, alt!(iri|a));

named!(a<CompleteStr,IRI>, value!(
    IRI::IRI(constants::RDF_TYPE),
    tag_s!("a")
));

/// [10] `subject ::= iri | BlankNode | collection`
named!(subject<CompleteStr,Subject>, alt!(
    map!(iri, Subject::IRI) |
    map!(blank_node, Subject::BlankNode) |
    map!(collection, Subject::Collection)
));

/// [11] `predicate ::= iri`

/// [12] `object ::= iri | BlankNode | collection | blankNodePropertyList | literal`
named!(object<CompleteStr,Object>, alt!(
    map!(literal, Object::Literal) |
    map!(iri, Object::IRI) |
    map!(blank_node, Object::BlankNode) |
    map!(collection, Object::Collection) |
    map!(blank_node_property_list, Object::BlankNodePropertyList)
));

/// [13] `literal ::= RDFLiteral | NumericLiteral | BooleanLiteral`
named!(literal<CompleteStr,Literal>, alt!(rdfliteral | boolean | double | decimal | integer));

/// [14] `blankNodePropertyList ::= '[' predicateObjectList ']'`
named!(blank_node_property_list<CompleteStr,Vec<PredicatedObjects> >, do_parse!(
    tag_s!("[") >> tws >>
    pol: predicated_objects_list >> tws >>
    tag_s!("]") >> (pol)
));

/// [15] `collection ::= '(' object* ')'`
named!(collection<CompleteStr,Vec<Object> >, do_parse!(
    tag_s!("(") >> tws >>
    objects: many0!(do_parse!(
        object: object >> tws >>
        (object))) >>
    tag_s!(")") >> (objects)
));

/// [16] `NumericLiteral ::= INTEGER | DECIMAL | DOUBLE`

/// [128s]  `RDFLiteral ::= String (LANGTAG | '^^' iri)?`
named!(rdfliteral<CompleteStr,Literal>, do_parse!(
    string: string >>
    datatype: opt!(alt!(langtag | iri_ref_literal)) >>
    ({
        match datatype {
            Some(RDFLiteralType::LangTag(langtag)) => {
                Literal {
                    lexical: string,
                    datatype: Datatype::RDFLangString,
                    language: Some(langtag)
                }
            },
            Some(RDFLiteralType::DataType(datatype)) => {
                Literal {
                    lexical: string,
                    datatype: Datatype::IRI(datatype),
                    language: None
                }
            },
            None => {
                Literal {
                    lexical: string,
                    datatype: Datatype::XSDString,
                    language: None
                }
            }
        }
    })
));

named!(iri_ref_literal<CompleteStr,RDFLiteralType>, do_parse!(
    tag_s!("^^") >>
    iri: iri >>
    (RDFLiteralType::DataType(iri))
));

/// [133s] `BooleanLiteral ::= 'true' | 'false'`
named!(pub boolean<CompleteStr,Literal>, do_parse!(
    b: alt!(tag_s!("true") | tag_s!("false")) >>
    (Literal {
        lexical: b.0,
        datatype: Datatype::XSDBoolean,
        language: None
    })
));

/// [17] `String ::= STRING_LITERAL_QUOTE | STRING_LITERAL_SINGLE_QUOTE`
///      `           | STRING_LITERAL_LONG_SINGLE_QUOTE | STRING_LITERAL_LONG_QUOTE`
named!(string<CompleteStr,&str>, alt!(string_literal_long_single_quote
    | string_literal_long_quote | string_literal_quote
    | string_literal_single_quote));

/// [135s] `iri ::= IRIREF | PrefixedName`
named!(iri<CompleteStr,IRI>, alt!(iri_iri|prefixed_name));

/// [136s]  `PrefixedName ::= PNAME_LN | PNAME_NS`
named!(prefixed_name<CompleteStr,IRI>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    pn_local: opt!(pn_local) >>
    (IRI::PrefixedName(
        pn_prefix.map(|p|p.0).unwrap_or(""),
        pn_local.map(|p|p.0).unwrap_or("")
    ))
));

/// [137s]  `BlankNode ::= BLANK_NODE_LABEL | ANON`
named!(blank_node<CompleteStr,BlankNode>, alt!(blank_node_label | anon));

/// [18] `IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'`
/// #x00=NULL #01-#x1F=control codes #x20=space
named!(iri_ref<CompleteStr,CompleteStr>, delimited!(
    tag_s!("<"),take_while_s!(is_iri_ref),tag_s!(">")
));

/// [139s] `PNAME_NS ::= PN_PREFIX? ':'`
named!(pname_ns<CompleteStr,CompleteStr>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    tag_s!(":") >>
    (pn_prefix.unwrap_or(CompleteStr("")))
));

/// [140s] `PNAME_LN ::= PNAME_NS PN_LOCAL`
/// see prefixed_name

/// [141s] `BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?`
named!(blank_node_label<CompleteStr,BlankNode>, do_parse!(
    tag!("_:") >>
    label: recognize!(tuple!(
        one_if!(is_pn_chars_u_digit),
        blank_node_label2
    )) >> (BlankNode::BlankNode(label.0))
));

fn is_pn_chars_u_digit(c: char) -> bool {
    is_digit(c) || is_pn_chars_u(c)
}

fn is_pn_chars_or_dot(c: char) -> bool {
    c == '.' || is_pn_chars(c)
}

fn blank_node_label2(src: CompleteStr) -> IResult<CompleteStr, ()> {
    match blank_node_label3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                Ok((CompleteStr(&src[m.len() - 1..]), ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

named!(blank_node_label3<CompleteStr,CompleteStr>, take_while_s!(is_pn_chars_or_dot));

/// [144s] `LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`
named!(langtag<CompleteStr,RDFLiteralType>, do_parse!(
    tag_s!("@") >>
    langtag: recognize!(tuple!(
        alpha,
        opt!(tuple!(tag_s!("-"), alphanumeric))
    )) >>
    (RDFLiteralType::LangTag(langtag.0))
));

/// [19] `INTEGER ::= [+-]? [0-9]+`
named!(pub integer<CompleteStr,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))), digit)),
    (|integer|{
        Literal {
            lexical: integer.0,
            datatype: Datatype::XSDInteger,
            language: None
        }
    })
));

/// [20] `DECIMAL ::= [+-]? [0-9]* '.' [0-9]+`
named!(pub decimal<CompleteStr,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))), opt_digit, tag_s!("."), digit)),
    (|decimal|{
        Literal {
            lexical: decimal.0,
            datatype: Datatype::XSDDecimal,
            language: None
        }
    })
));

/// [21] `DOUBLE ::= [+-]? ([0-9]+ '.' [0-9]* EXPONENT | '.' [0-9]+ EXPONENT | [0-9]+ EXPONENT)`
named!(pub double<CompleteStr,Literal>, map!(recognize!(tuple!(
    opt!(alt!(tag_s!("+") | tag_s!("-"))),
    alt!(
        recognize!(tuple!(digit,tag_s!("."), opt_digit, exponent)) |
        recognize!(tuple!(opt!(tag_s!(".")), digit, exponent))
    ))),
    (|double|{
        Literal {
            lexical: double.0,
            datatype: Datatype::XSDDouble,
            language: None
        }
    })
));

/// [154s] `EXPONENT ::= [eE] [+-]? [0-9]+`
named!(exponent<CompleteStr,()>, map!(tuple!(
    alt!(tag_s!("E") | tag_s!("e")),opt!(alt!(tag_s!("+") | tag_s!("-"))), digit),
    (|_|())
));

/// [22] `STRING_LITERAL_QUOTE ::= '"' ([^#x22#x5C#xA#xD] | ECHAR | UCHAR)* '"'`
/// /* #x22=" #x5C=\ #xA=new line #xD=carriage return */
fn string_literal_quote(str: CompleteStr) -> IResult<CompleteStr, &str> {
    string_literal(str, 1, start_quote, find_quote)
}
fn start_quote(s: CompleteStr) -> bool {
    s.starts_with('"')
}
fn find_quote(s: CompleteStr) -> Option<usize> {
    s.find('"')
}

/// [23] `STRING_LITERAL_SINGLE_QUOTE ::= "'" ([^#x27#x5C#xA#xD] | ECHAR | UCHAR)* "'"`
/// /* #x27=' #x5C=\ #xA=new line #xD=carriage return */
fn string_literal_single_quote(str: CompleteStr) -> IResult<CompleteStr, &str> {
    string_literal(str, 1, start_single_quote, find_single_quote)
}
fn start_single_quote(s: CompleteStr) -> bool {
    s.starts_with('\'')
}
fn find_single_quote(s: CompleteStr) -> Option<usize> {
    s.find('\'')
}

/// [24] `STRING_LITERAL_LONG_SINGLE_QUOTE ::= "'''" (("'" | "''")? ([^'\] | ECHAR | UCHAR))* "'''"`
fn string_literal_long_single_quote(str: CompleteStr) -> IResult<CompleteStr, &str> {
    string_literal(str, 3, start_long_single_quote, find_long_single_quote)
}
fn start_long_single_quote(s: CompleteStr) -> bool {
    s.starts_with("'''")
}
fn find_long_single_quote(s: CompleteStr) -> Option<usize> {
    s.find("'''")
}

/// [25] `STRING_LITERAL_LONG_QUOTE ::= '"""' (('"' | '""')? ([^"\] | ECHAR | UCHAR))* '"""'`
fn string_literal_long_quote(str: CompleteStr) -> IResult<CompleteStr, &str> {
    string_literal(str, 3, start_long_quote, find_long_quote)
}
fn start_long_quote(s: CompleteStr) -> bool {
    s.starts_with("\"\"\"")
}
fn find_long_quote(s: CompleteStr) -> Option<usize> {
    s.find("\"\"\"")
}

/// [26] `UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX`
/// [159s] `ECHAR ::= '\' [tbnrf"'\]`

/// [161s] `WS ::= #x20 | #x9 | #xD | #xA`
/// /* #x20=space #x9=character tabulation #xD=carriage return #xA=new line */
fn is_ws(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

/// [162s] `ANON ::= '[' WS* ']'`
named!(anon<CompleteStr,BlankNode>, do_parse!(
    tag_s!("[") >>
    tws >>
    tag_s!("]") >> (BlankNode::Anon)
));

/// `[163s] PN_CHARS_BASE ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6]`
/// `| [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D]`
/// `| [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF]`
/// `| [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]`
fn is_pn_chars_base(c: char) -> bool {
    is_alpha(c) || in_range(c, 0xC0, 0x00D6) || in_range(c, 0x00D8, 0x00F6)
        || in_range(c, 0x00F8, 0x02FF) || in_range(c, 0x0370, 0x037D)
        || in_range(c, 0x037F, 0x1FFF) || in_range(c, 0x200C, 0x200D)
        || in_range(c, 0x2070, 0x218F) || in_range(c, 0x2C00, 0x2FEF)
        || in_range(c, 0x3001, 0xD7FF) || in_range(c, 0xF900, 0xFDCF)
        || in_range(c, 0xFDF0, 0xFFFD) || in_range(c, 0x10000, 0xEFFFF)
}

/// [164s] `PN_CHARS_U ::= PN_CHARS_BASE | '_'`
fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

/// [166s] `PN_CHARS ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]`
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c) || c == '-' || is_digit(c) || c == 0xB7 as char || in_range(c, 0x0300, 0x036F)
        || in_range(c, 0x203F, 0x2040)
}

/// [167s] PN_PREFIX ::= PN_CHARS_BASE ((PN_CHARS | '.')* PN_CHARS)?
named!(pn_prefix<CompleteStr,CompleteStr>, recognize!(tuple!(
    one_if!(is_pn_chars_base),
    take_while_s!(is_pn_chars),
    fold_many0!(tuple!(
        tag_s!("."),
        take_while1_s!(is_pn_chars)
    ),(),|_,_|())
)));

/// [168s] PN_LOCAL ::= (PN_CHARS_U | ':' | [0-9] | PLX)
///           ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX))?
named!(pub pn_local<CompleteStr,CompleteStr>, recognize!(tuple!(
    alt!(one_if!(is_pn_local_start) | plx),
    pn_local2
)));

fn pn_local2(src: CompleteStr) -> IResult<CompleteStr, ()> {
    match pn_local3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                Ok((CompleteStr(&src[m.len() - 1..]), ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

named!(pn_local3<CompleteStr,CompleteStr>,
    recognize!(many0!(alt!(pn_chars_colon | plx | tag_s!(".")))));

named!(pn_chars_colon<CompleteStr,CompleteStr>, take_while1_s!(is_pn_chars_colon));

fn is_pn_local_start(c: char) -> bool {
    c == ':' || is_digit(c) || is_pn_chars_u(c)
}

fn is_pn_chars_colon(c: char) -> bool {
    c == ':' || is_pn_chars(c)
}

/// [169s] PLX ::= PERCENT | PN_LOCAL_ESC
named!(plx<CompleteStr,CompleteStr>, alt!(percent | pn_local_esc));

/// [170s] PERCENT ::= '%' HEX HEX
/// [171s] HEX ::= [0-9] | [A-F] | [a-f]
named!(percent<CompleteStr,CompleteStr>, recognize!(tuple!(
    tag_s!("%"),
    one_if!(is_hex),
    one_if!(is_hex)
)));

/// [172s] PN_LOCAL_ESC ::= '\' ('_' | '~' | '.' | '-' | '!' | '$' | '&' | "'"
/// | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%')
named!(pn_local_esc<CompleteStr,CompleteStr>, recognize!(tuple!(
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
    use nom::Context;
    assert_eq!(comment(CompleteStr("#\r\na")), Ok((CompleteStr("\na"), "")));
    assert_eq!(comment(CompleteStr("#\n\ra")), Ok((CompleteStr("\ra"), "")));
    assert_eq!(
        comment(CompleteStr("")),
        Err(Err::Error(Context::Code(CompleteStr(""), ErrorKind::Eof)))
    );
    assert_eq!(comment(CompleteStr("#")), Ok((CompleteStr(""), "")));
    assert_eq!(comment(CompleteStr("#abc")), Ok((CompleteStr(""), "abc")));
    assert_eq!(comment(CompleteStr("#\n\n")), Ok((CompleteStr("\n"), "")));
}

#[test]
fn test_prefixed_name() {
    assert_eq!(
        prefixed_name(CompleteStr("a:a ")),
        Ok((CompleteStr(&" "[..]), IRI::PrefixedName("a", "a")))
    );
    assert_eq!(
        prefixed_name(CompleteStr(": ")),
        Ok((CompleteStr(&" "[..]), IRI::PrefixedName("", "")))
    );
}

named!(alpha<CompleteStr,CompleteStr>, take_while1_s!(is_alpha));
named!(alphanumeric<CompleteStr,CompleteStr>, take_while1_s!(is_alphanum));
named!(digit<CompleteStr,CompleteStr>, take_while1_s!(is_digit));
named!(opt_digit<CompleteStr,CompleteStr>, take_while_s!(is_digit));

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_iri<CompleteStr,IRI>, map!(iri_ref, |v| IRI::IRI(v.0)));

#[test]
fn test_iri() {
    assert_eq!(
        iri(CompleteStr("<urn:123>")),
        Ok((CompleteStr(&""[..]), IRI::IRI("urn:123")))
    );
}

#[test]
fn test_string_literal_quote() {
    assert_eq!(
        string_literal_quote(CompleteStr("\"\\\\\"")),
        Ok((CompleteStr(&""[..]), "\\\\"))
    );
}

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(
        string_literal_single_quote(CompleteStr("''")),
        Ok((CompleteStr(&""[..]), ""))
    );
}

#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(
        string_literal_long_single_quote(CompleteStr("''''''")),
        Ok((CompleteStr(&""[..]), ""))
    );
}

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(
        string_literal_long_quote(CompleteStr("\"\"\"\\U0001f435\"\"\"")),
        Ok((CompleteStr(&""[..]), "\\U0001f435"))
    );
    assert_eq!(
        string_literal_long_quote(CompleteStr("\"\"\"first long literal\"\"\"")),
        Ok((CompleteStr(&""[..]), "first long literal"))
    );
}

#[test]
fn test_langtag() {
    assert_eq!(
        langtag(CompleteStr("@nl ")),
        Ok((CompleteStr(&" "[..]), RDFLiteralType::LangTag("nl")))
    );
    assert_eq!(
        langtag(CompleteStr("@nl-NL ")),
        Ok((CompleteStr(&" "[..]), RDFLiteralType::LangTag("nl-NL")))
    );
}

#[test]
fn test_rdfliteral() {
    let r = Literal {
        lexical: "",
        datatype: Datatype::XSDString,
        language: None,
    };
    assert_eq!(
        rdfliteral(CompleteStr("'' ")),
        Ok((CompleteStr(&" "[..]), r))
    );
}
#[cfg(test)]
fn literal_true<'a>() -> Literal<'a> {
    Literal {
        lexical: "true",
        datatype: Datatype::XSDBoolean,
        language: None,
    }
}
#[cfg(test)]
fn literal_false<'a>() -> Literal<'a> {
    Literal {
        lexical: "false",
        datatype: Datatype::XSDBoolean,
        language: None,
    }
}
#[cfg(test)]
fn literal_11<'a>() -> Literal<'a> {
    Literal {
        lexical: "11",
        datatype: Datatype::XSDInteger,
        language: None,
    }
}
#[cfg(test)]
fn literal_d11<'a>() -> Literal<'a> {
    Literal {
        lexical: "11.1",
        datatype: Datatype::XSDDecimal,
        language: None,
    }
}

#[test]
fn test_integer() {
    assert_eq!(
        literal(CompleteStr("11 ")),
        Ok((CompleteStr(&" "[..]), literal_11()))
    );
    assert_eq!(
        literal(CompleteStr("+1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Literal {
                lexical: "+1",
                datatype: Datatype::XSDInteger,
                language: None
            }
        ))
    );
    assert_eq!(
        integer(CompleteStr("-1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Literal {
                lexical: "-1",
                datatype: Datatype::XSDInteger,
                language: None
            }
        ))
    );
}

#[test]
fn test_decimal() {
    assert_eq!(
        literal(CompleteStr("11.1 ")),
        Ok((CompleteStr(&" "[..]), literal_d11()))
    );
    assert_eq!(
        literal(CompleteStr("+1.1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Literal {
                lexical: "+1.1",
                datatype: Datatype::XSDDecimal,
                language: None
            }
        ))
    );
    assert_eq!(
        literal(CompleteStr("-1.1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Literal {
                lexical: "-1.1",
                datatype: Datatype::XSDDecimal,
                language: None
            }
        ))
    );
    assert_eq!(
        literal(CompleteStr(".1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Literal {
                lexical: ".1",
                datatype: Datatype::XSDDecimal,
                language: None
            }
        ))
    );
}

#[test]
fn test_boolean() {
    assert_eq!(
        boolean(CompleteStr("true")),
        Ok((CompleteStr(&""[..]), literal_true()))
    );
    assert_eq!(
        boolean(CompleteStr("false")),
        Ok((CompleteStr(&""[..]), literal_false()))
    );
}

#[test]
fn test_literal() {
    assert_eq!(
        literal(CompleteStr("true")),
        Ok((CompleteStr(&""[..]), literal_true()))
    );
    assert_eq!(
        literal(CompleteStr("false")),
        Ok((CompleteStr(&""[..]), literal_false()))
    );
}

#[test]
fn test_object() {
    assert_eq!(
        object(CompleteStr("_:b1 ")),
        Ok((
            CompleteStr(&" "[..]),
            Object::BlankNode(BlankNode::BlankNode("b1"))
        ))
    );
    let long = Object::Literal(Literal {
        lexical: "first long literal",
        datatype: Datatype::XSDString,
        language: None,
    });
    assert_eq!(
        object(CompleteStr("\"\"\"first long literal\"\"\" ")),
        Ok((CompleteStr(&" "[..]), long))
    );
}

#[test]
fn test_blank_node_label() {
    assert_eq!(
        blank_node_label(CompleteStr("_:b1 ")),
        Ok((CompleteStr(&" "[..]), BlankNode::BlankNode("b1")))
    );
    assert_eq!(
        blank_node_label(CompleteStr("_:b1. ")),
        Ok((CompleteStr(&". "[..]), BlankNode::BlankNode("b1")))
    );
}

#[test]
fn test_object_list() {
    let v = vec![
        Object::Literal(literal_true()),
        Object::Literal(literal_11()),
        Object::Literal(literal_false()),
    ];
    assert_eq!(
        object_list(CompleteStr("true, 11 , false ")),
        Ok((CompleteStr(&" "[..]), v))
    );
}

#[test]
fn test_predicated_objects() {
    let v = vec![Object::Literal(Literal {
        lexical: "1",
        datatype: Datatype::XSDInteger,
        language: None,
    })];
    let po = PredicatedObjects {
        verb: IRI::IRI("urn:123"),
        objects: v,
    };
    assert_eq!(
        predicated_objects_list(CompleteStr("<urn:123> 1 ")),
        Ok((CompleteStr(&" "[..]), vec![po]))
    );
}

#[test]
fn test_triples() {
    let v = vec![Object::Literal(Literal {
        lexical: "1",
        datatype: Datatype::XSDInteger,
        language: None,
    })];
    let i = IRI::IRI("urn:123");
    let s = Subject::IRI(i.clone());
    let po = vec![PredicatedObjects {
        verb: i.clone(),
        objects: v,
    }];
    let t = Triples {
        subject: s,
        predicated_objects_list: po,
    };
    assert_eq!(
        triples(CompleteStr("<urn:123> <urn:123> 1 ")),
        Ok((CompleteStr(&" "[..]), t))
    );
}

#[test]
fn test_statement_triples() {
    let i = IRI::PrefixedName("", "");
    let s = Subject::IRI(i.clone());
    let po = vec![PredicatedObjects {
        verb: i.clone(),
        objects: vec![Object::IRI(i.clone())],
    }];
    let t = Triples {
        subject: s,
        predicated_objects_list: po,
    };
    let s = Statement::Triples(t);
    assert_eq!(
        statement_triples(CompleteStr(": : :.")),
        Ok((CompleteStr(&""[..]), s))
    );
}

#[test]
fn test_prefix_id() {
    assert_eq!(
        prefix_id(CompleteStr("@prefix a.b.c: <urn> .")),
        Ok((CompleteStr(&""[..]), Statement::Prefix("a.b.c", "urn")))
    );
    assert_eq!(
        prefix_id(CompleteStr("@prefix : <urn> .")),
        Ok((CompleteStr(&""[..]), Statement::Prefix("", "urn")))
    );
}

#[test]
fn test_base() {
    assert_eq!(
        base(CompleteStr("@base <urn> .")),
        Ok((CompleteStr(&""[..]), Statement::Base("urn")))
    );
}

#[test]
fn test_sparql_base() {
    assert_eq!(
        sparql_base(CompleteStr("BASE <urn>")),
        Ok((CompleteStr(&""[..]), Statement::Base("urn")))
    );
}

#[test]
fn test_sparql_prefix() {
    assert_eq!(
        sparql_prefix(CompleteStr("PREFIX a.b.c: <urn>")),
        Ok((CompleteStr(&""[..]), Statement::Prefix("a.b.c", "urn")))
    );
}

#[test]
fn test_pn_local() {
    // dot does not belong in the pn_local
    assert_eq!(
        pn_local(CompleteStr("c. ")),
        Ok((CompleteStr(&". "[..]), CompleteStr("c")))
    );
}
