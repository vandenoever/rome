use super::grammar_helper::*;
use super::grammar_structs::*;
use crate::ontology::iri::rdf;

use nom::{
    alt, char, do_parse, many0, map, named, one_of, opt, recognize, take_while, take_while1, tuple,
    value,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1},
    character::complete::{char, one_of},
    combinator::{map, opt, recognize},
    error::{ErrorKind, ParseError},
    error_position,
    multi::{fold_many0, fold_many1, many0, separated_nonempty_list},
    sequence::{delimited, tuple},
    Err, IResult, InputTake, Needed,
};

/// Take one character if it fits the function
fn one_if<'a, E: ParseError<&'a str>, F: Fn(char) -> bool>(
    f: F,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> {
    move |i: &str| {
        if let Some(c) = i.chars().next() {
            if f(c) {
                Ok(i.take_split(1))
            } else {
                Err(Err::Error(error_position!(i, ErrorKind::OneOf)))
            }
        } else {
            Err(Err::Incomplete(Needed::Size(1)))
        }
    }
}

fn not_eol(c: char) -> bool {
    c != '\n' && c != '\r'
}

fn comment(i: &str) -> IResult<&str, &str> {
    let (i, _) = char('#')(i)?;
    let (i, comment) = take_while(not_eol)(i)?;
    if i.is_empty() {
        Ok((i, comment))
    } else {
        // remove one \n or \r
        Ok((&i[1..], comment))
    }
}

/// whitespace that may contain comments
pub fn tws(i: &str) -> IResult<&str, ()> {
    fold_many0(
        alt((map(one_of(" \t\n\r"), |_| ()), map(comment, |_| ()))),
        (),
        |_, _| (),
    )(i)
}

/// [2] `statement ::= directive | triples '.'`
/// [3] `directive ::= prefixID | base | sparqlPrefix | sparqlBase`
pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((
        statement_triples,
        prefix_id,
        base,
        sparql_prefix,
        sparql_base,
    ))(i)
}

fn statement_triples(i: &str) -> IResult<&str, Statement> {
    let (i, (triples, _, _)) = tuple((triples, tws, char('.')))(i)?;
    Ok((i, Statement::Triples(triples)))
}

/// [4] `prefixID ::= '@prefix' PNAME_NS IRIREF '.'`
fn prefix_id(i: &str) -> IResult<&str, Statement> {
    let (i, (_, _, pname_ns, _, iri_ref, _, _)) =
        tuple((tag("@prefix"), tws, pname_ns, tws, iri_ref, tws, char('.')))(i)?;
    Ok((i, Statement::Prefix(pname_ns, iri_ref)))
}

/// [5] `base ::= '@base' IRIREF '.'`
fn base(i: &str) -> IResult<&str, Statement> {
    let (i, (_, _, iri_ref, _, _)) = tuple((tag("@base"), tws, iri_ref, tws, char('.')))(i)?;
    Ok((i, Statement::Base(iri_ref)))
}

/// [5s] `sparqlBase ::= "BASE" IRIREF`
fn sparql_base(i: &str) -> IResult<&str, Statement> {
    let (i, (_, _, iri_ref)) = tuple((tag_no_case("BASE"), tws, iri_ref))(i)?;
    Ok((i, Statement::Base(iri_ref)))
}

/// [6s] `sparqlPrefix ::= "PREFIX" PNAME_NS IRIREF`
fn sparql_prefix(i: &str) -> IResult<&str, Statement> {
    let (i, (_, _, pname_ns, _, iri_ref)) =
        tuple((tag_no_case("PREFIX"), tws, pname_ns, tws, iri_ref))(i)?;
    Ok((i, Statement::Prefix(pname_ns, iri_ref)))
}

/// [6] `triples ::= subject predicateObjectList | blankNodePropertyList predicateObjectList?`

named!(triples<&str,Triples>, alt!(triples_subject | triples_blank));

named!(triples_subject<&str,Triples>, do_parse!(
    subject: subject >> tws >>
    predicated_objects_list: predicated_objects_list >>
    (Triples{
        subject,
        predicated_objects_list
    })
));

fn triples_blank(str: &str) -> IResult<&str, Triples> {
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
fn predicated_objects_list(mut str: &str) -> IResult<&str, Vec<PredicatedObjects>> {
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

fn predicated_object_sep(i: &str) -> IResult<&str, ()> {
    fold_many1(tuple((tws, char(';'), tws)), (), |_, _| ())(i)
}

named!(predicated_object<&str,PredicatedObjects>, do_parse!(
    verb: verb >>
    tws >>
    objects: object_list >>
    (PredicatedObjects{
        verb,
        objects
    })
));

/// [8] `objectList ::= object (',' object)*`
fn object_list(i: &str) -> IResult<&str, Vec<Object>> {
    separated_nonempty_list(tuple((tws, char(','), tws)), object)(i)
}

/// [9] `verb ::= predicate | 'a'`
named!(verb<&str,IRI>, alt!(iri|a));

named!(a<&str,IRI>, value!(
    IRI::IRI(rdf::TYPE),
    char!('a')
));

/// [10] `subject ::= iri | BlankNode | collection`
named!(subject<&str,Subject>, alt!(
    map!(iri, Subject::IRI) |
    map!(blank_node, Subject::BlankNode) |
    map!(collection, Subject::Collection)
));

/// [11] `predicate ::= iri`

/// [12] `object ::= iri | BlankNode | collection | blankNodePropertyList | literal`
named!(object<&str,Object>, alt!(
    map!(literal, Object::Literal) |
    map!(iri, Object::IRI) |
    map!(blank_node, Object::BlankNode) |
    map!(collection, Object::Collection) |
    map!(blank_node_property_list, Object::BlankNodePropertyList)
));

/// [13] `literal ::= RDFLiteral | NumericLiteral | BooleanLiteral`
named!(literal<&str,Literal>, alt!(rdfliteral | boolean | double | decimal | integer));

/// [14] `blankNodePropertyList ::= '[' predicateObjectList ']'`
named!(blank_node_property_list<&str,Vec<PredicatedObjects> >, do_parse!(
    char!('[') >> tws >>
    pol: predicated_objects_list >> tws >>
    char!(']') >> (pol)
));

/// [15] `collection ::= '(' object* ')'`
named!(collection<&str,Vec<Object> >, do_parse!(
    char!('(') >> tws >>
    objects: many0!(do_parse!(
        object: object >> tws >>
        (object))) >>
    char!(')') >> (objects)
));

/// [16] `NumericLiteral ::= INTEGER | DECIMAL | DOUBLE`

/// [128s]  `RDFLiteral ::= String (LANGTAG | '^^' iri)?`
named!(rdfliteral<&str,Literal>, do_parse!(
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
fn iri_ref_literal(i: &str) -> IResult<&str, RDFLiteralType> {
    let (i, (_, iri)) = tuple((tag("^^"), iri))(i)?;
    Ok((i, RDFLiteralType::DataType(iri)))
}

/// [133s] `BooleanLiteral ::= 'true' | 'false'`
pub fn boolean(i: &str) -> IResult<&str, Literal> {
    let (i, b) = alt((tag("true"), tag("false")))(i)?;
    Ok((
        i,
        Literal {
            lexical: b,
            datatype: Datatype::XSDBoolean,
            language: None,
        },
    ))
}

/// [17] `String ::= STRING_LITERAL_QUOTE | STRING_LITERAL_SINGLE_QUOTE`
///      `           | STRING_LITERAL_LONG_SINGLE_QUOTE | STRING_LITERAL_LONG_QUOTE`
named!(string<&str,&str>, alt!(string_literal_long_single_quote
    | string_literal_long_quote | string_literal_quote
    | string_literal_single_quote));

/// [135s] `iri ::= IRIREF | PrefixedName`
named!(iri<&str,IRI>, alt!(iri_iri|prefixed_name));

/// [136s]  `PrefixedName ::= PNAME_LN | PNAME_NS`
named!(prefixed_name<&str,IRI>, do_parse!(
    pn_prefix: opt!(pn_prefix) >>
    char!(':') >>
    pn_local: opt!(pn_local) >>
    (IRI::PrefixedName(
        pn_prefix.map(|p|p).unwrap_or(""),
        pn_local.map(|p|p).unwrap_or("")
    ))
));

/// [137s]  `BlankNode ::= BLANK_NODE_LABEL | ANON`
named!(blank_node<&str,BlankNode>, alt!(blank_node_label | anon));

/// [18] `IRIREF ::= '<' ([^#x00-#x20<>"{}|^`\] | UCHAR)* '>'`
/// #x00=NULL #01-#x1F=control codes #x20=space
fn iri_ref(i: &str) -> IResult<&str, &str> {
    delimited(char('<'), take_while(is_iri_ref), char('>'))(i)
}

/// [139s] `PNAME_NS ::= PN_PREFIX? ':'`
fn pname_ns(i: &str) -> IResult<&str, &str> {
    let (i, pn_prefix) = opt(pn_prefix)(i)?;
    let (i, _) = char(':')(i)?;
    Ok((i, pn_prefix.unwrap_or("")))
}

/// [140s] `PNAME_LN ::= PNAME_NS PN_LOCAL`
/// see prefixed_name

/// [141s] `BLANK_NODE_LABEL ::= '_:' (PN_CHARS_U | [0-9]) ((PN_CHARS | '.')* PN_CHARS)?`
fn blank_node_label(i: &str) -> IResult<&str, BlankNode> {
    let (i, _) = tag("_:")(i)?;
    let (i, label) = recognize(tuple((one_if(is_pn_chars_u_digit), blank_node_label2)))(i)?;
    Ok((i, BlankNode::BlankNode(label)))
}

fn is_pn_chars_u_digit(c: char) -> bool {
    is_digit(c) || is_pn_chars_u(c)
}

fn is_pn_chars_or_dot(c: char) -> bool {
    c == '.' || is_pn_chars(c)
}

fn blank_node_label2(src: &str) -> IResult<&str, ()> {
    match blank_node_label3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                Ok(((&src[m.len() - 1..]), ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

named!(blank_node_label3<&str,&str>, take_while!(is_pn_chars_or_dot));

/// [144s] `LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`
named!(langtag<&str,RDFLiteralType>, do_parse!(
    char!('@') >>
    langtag: recognize!(tuple!(
        alpha,
        opt!(tuple!(char!('-'), alphanumeric))
    )) >>
    (RDFLiteralType::LangTag(langtag))
));

/// [19] `INTEGER ::= [+-]? [0-9]+`
named!(pub integer<&str,Literal>, map!(recognize!(tuple!(
    opt!(one_of!("+-")), digit)),
    (|integer|{
        Literal {
            lexical: integer,
            datatype: Datatype::XSDInteger,
            language: None
        }
    })
));

/// [20] `DECIMAL ::= [+-]? [0-9]* '.' [0-9]+`
named!(pub decimal<&str,Literal>, map!(recognize!(tuple!(
    opt!(one_of!("+-")), opt_digit, char!('.'), digit)),
    (|decimal|{
        Literal {
            lexical: decimal,
            datatype: Datatype::XSDDecimal,
            language: None
        }
    })
));

/// [21] `DOUBLE ::= [+-]? ([0-9]+ '.' [0-9]* EXPONENT | '.' [0-9]+ EXPONENT | [0-9]+ EXPONENT)`
named!(pub double<&str,Literal>, map!(recognize!(tuple!(
    opt!(one_of!("+-")),
    alt!(
        recognize!(tuple!(digit,char!('.'), opt_digit, exponent)) |
        recognize!(tuple!(opt!(char!('.')), digit, exponent))
    ))),
    (|double|{
        Literal {
            lexical: double,
            datatype: Datatype::XSDDouble,
            language: None
        }
    })
));

/// [154s] `EXPONENT ::= [eE] [+-]? [0-9]+`
named!(exponent<&str,()>, map!(tuple!(
    one_of!("Ee"),opt!(one_of!("+-")), digit),
    (|_|())
));

/// [22] `STRING_LITERAL_QUOTE ::= '"' ([^#x22#x5C#xA#xD] | ECHAR | UCHAR)* '"'`
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

/// [23] `STRING_LITERAL_SINGLE_QUOTE ::= "'" ([^#x27#x5C#xA#xD] | ECHAR | UCHAR)* "'"`
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

/// [24] `STRING_LITERAL_LONG_SINGLE_QUOTE ::= "'''" (("'" | "''")? ([^'\] | ECHAR | UCHAR))* "'''"`
fn string_literal_long_single_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 3, start_long_single_quote, find_long_single_quote)
}
fn start_long_single_quote(s: &str) -> bool {
    s.starts_with("'''")
}
fn find_long_single_quote(s: &str) -> Option<usize> {
    s.find("'''")
}

/// [25] `STRING_LITERAL_LONG_QUOTE ::= '"""' (('"' | '""')? ([^"\] | ECHAR | UCHAR))* '"""'`
fn string_literal_long_quote(str: &str) -> IResult<&str, &str> {
    string_literal(str, 3, start_long_quote, find_long_quote)
}
fn start_long_quote(s: &str) -> bool {
    s.starts_with("\"\"\"")
}
fn find_long_quote(s: &str) -> Option<usize> {
    s.find("\"\"\"")
}

/// [26] `UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX`
/// [159s] `ECHAR ::= '\' [tbnrf"'\]`

/// [161s] `WS ::= #x20 | #x9 | #xD | #xA`
/// /* #x20=space #x9=character tabulation #xD=carriage return #xA=new line */
/// [162s] `ANON ::= '[' WS* ']'`
named!(anon<&str,BlankNode>, do_parse!(
    char!('[') >>
    tws >>
    char!(']') >> (BlankNode::Anon)
));

/// `[163s] PN_CHARS_BASE ::= [A-Z] | [a-z] | [#x00C0-#x00D6] | [#x00D8-#x00F6]`
/// `| [#x00F8-#x02FF] | [#x0370-#x037D] | [#x037F-#x1FFF] | [#x200C-#x200D]`
/// `| [#x2070-#x218F] | [#x2C00-#x2FEF] | [#x3001-#xD7FF] | [#xF900-#xFDCF]`
/// `| [#xFDF0-#xFFFD] | [#x10000-#xEFFFF]`
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

/// [164s] `PN_CHARS_U ::= PN_CHARS_BASE | '_'`
fn is_pn_chars_u(c: char) -> bool {
    c == '_' || is_pn_chars_base(c)
}

/// [166s] `PN_CHARS ::= PN_CHARS_U | '-' | [0-9] | #x00B7 | [#x0300-#x036F] | [#x203F-#x2040]`
fn is_pn_chars(c: char) -> bool {
    is_pn_chars_u(c)
        || c == '-'
        || is_digit(c)
        || c == 0xB7 as char
        || in_range(c, 0x0300, 0x036F)
        || in_range(c, 0x203F, 0x2040)
}

/// [167s] PN_PREFIX ::= PN_CHARS_BASE ((PN_CHARS | '.')* PN_CHARS)?
fn pn_prefix(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        one_if(is_pn_chars_base),
        take_while(is_pn_chars),
        fold_many0(tuple((char('.'), take_while1(is_pn_chars))), (), |_, _| ()),
    )))(i)
}

/// [168s] PN_LOCAL ::= (PN_CHARS_U | ':' | [0-9] | PLX)
///           ((PN_CHARS | '.' | ':' | PLX)* (PN_CHARS | ':' | PLX))?
pub fn pn_local(i: &str) -> IResult<&str, &str> {
    recognize(tuple((alt((one_if(is_pn_local_start), plx)), pn_local2)))(i)
}

fn pn_local2(src: &str) -> IResult<&str, ()> {
    match pn_local3(src) {
        Ok((left, m)) => {
            // if last is a '.', remove that
            if m.ends_with('.') {
                Ok(((&src[m.len() - 1..]), ()))
            } else {
                Ok((left, ()))
            }
        }
        Err(e) => Err(e),
    }
}

fn pn_local3(i: &str) -> IResult<&str, &str> {
    recognize(many0(alt((pn_chars_colon, plx, tag(".")))))(i)
}

fn pn_chars_colon(i: &str) -> IResult<&str, &str> {
    take_while1(is_pn_chars_colon)(i)
}

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
fn percent(i: &str) -> IResult<&str, &str> {
    recognize(tuple((char('%'), one_if(is_hex), one_if(is_hex))))(i)
}

/// [172s] PN_LOCAL_ESC ::= '\' ('_' | '~' | '.' | '-' | '!' | '$' | '&' | "'"
/// | '(' | ')' | '*' | '+' | ',' | ';' | '=' | '/' | '?' | '#' | '@' | '%')
fn pn_local_esc(i: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char('\\'),
        one_if(|c| "_~.-!$&'()*+,;=/?#@%".contains(c)),
    )))(i)
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

fn is_hex(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

fn in_range(c: char, lower: u32, upper: u32) -> bool {
    c as u32 >= lower && c as u32 <= upper
}

#[test]
fn test_comment() {
    assert_eq!(comment("#\r\na"), Ok((("\na"), (""))));
    assert_eq!(comment("#\n\ra"), Ok((("\ra"), (""))));
    assert_eq!(comment(""), Err(Err::Error(("", ErrorKind::Char))));
    assert_eq!(comment("#"), Ok(("", "")));
    assert_eq!(comment("#abc"), Ok(("", "abc")));
    assert_eq!(comment("#\n\n"), Ok(("\n", "")));
}

#[test]
fn test_prefixed_name() {
    assert_eq!(
        prefixed_name("a:a "),
        Ok((" ", IRI::PrefixedName("a", "a")))
    );
    assert_eq!(prefixed_name(": "), Ok((" ", IRI::PrefixedName("", ""))));
}

named!(alpha<&str,&str>, take_while1!(is_alpha));
named!(alphanumeric<&str,&str>, take_while1!(is_alphanum));
named!(digit<&str,&str>, take_while1!(is_digit));
named!(opt_digit<&str,&str>, take_while!(is_digit));

#[inline]
fn is_iri_ref(chr: char) -> bool {
    chr > ' ' && "<>\"{}|^`".find(chr) == None
}

named!(iri_iri<&str,IRI>, map!(iri_ref, |v| IRI::IRI(v)));

#[test]
fn test_iri() {
    assert_eq!(iri("<urn:123>"), Ok(("", IRI::IRI("urn:123"))));
}

#[test]
fn test_string_literal_quote() {
    assert_eq!(string_literal_quote("\"\\\\\""), Ok(("", "\\\\")));
}

#[test]
fn test_string_literal_single_quote() {
    assert_eq!(string_literal_single_quote("''"), Ok(("", "")));
}

#[test]
fn test_string_literal_long_single_quote() {
    assert_eq!(string_literal_long_single_quote("''''''"), Ok(("", "")));
}

#[test]
fn test_string_literal_long_quote() {
    assert_eq!(
        string_literal_long_quote("\"\"\"\\U0001f435\"\"\""),
        Ok(("", "\\U0001f435"))
    );
    assert_eq!(
        string_literal_long_quote("\"\"\"first long literal\"\"\""),
        Ok(("", "first long literal"))
    );
}

#[test]
fn test_langtag() {
    assert_eq!(langtag("@nl "), Ok((" ", RDFLiteralType::LangTag("nl"))));
    assert_eq!(
        langtag("@nl-NL "),
        Ok((" ", RDFLiteralType::LangTag("nl-NL")))
    );
}

#[test]
fn test_rdfliteral() {
    let r = Literal {
        lexical: "",
        datatype: Datatype::XSDString,
        language: None,
    };
    assert_eq!(rdfliteral("'' "), Ok((" ", r)));
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
    assert_eq!(literal("11 "), Ok((" ", literal_11())));
    assert_eq!(
        literal("+1 "),
        Ok((
            " ",
            Literal {
                lexical: "+1",
                datatype: Datatype::XSDInteger,
                language: None
            }
        ))
    );
    assert_eq!(
        integer("-1 "),
        Ok((
            " ",
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
    assert_eq!(literal("11.1 "), Ok((" ", literal_d11())));
    assert_eq!(
        literal("+1.1 "),
        Ok((
            " ",
            Literal {
                lexical: "+1.1",
                datatype: Datatype::XSDDecimal,
                language: None
            }
        ))
    );
    assert_eq!(
        literal("-1.1 "),
        Ok((
            " ",
            Literal {
                lexical: "-1.1",
                datatype: Datatype::XSDDecimal,
                language: None
            }
        ))
    );
    assert_eq!(
        literal(".1 "),
        Ok((
            " ",
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
    assert_eq!(boolean("true"), Ok(("", literal_true())));
    assert_eq!(boolean("false"), Ok(("", literal_false())));
}

#[test]
fn test_literal() {
    assert_eq!(literal("true"), Ok(("", literal_true())));
    assert_eq!(literal("false"), Ok(("", literal_false())));
}

#[test]
fn test_object() {
    assert_eq!(
        object("_:b1 "),
        Ok((" ", Object::BlankNode(BlankNode::BlankNode("b1"))))
    );
    let long = Object::Literal(Literal {
        lexical: "first long literal",
        datatype: Datatype::XSDString,
        language: None,
    });
    assert_eq!(object("\"\"\"first long literal\"\"\" "), Ok((" ", long)));
}

#[test]
fn test_blank_node_label() {
    assert_eq!(
        blank_node_label("_:b1 "),
        Ok((" ", BlankNode::BlankNode("b1")))
    );
    assert_eq!(
        blank_node_label("_:b1. "),
        Ok((". ", BlankNode::BlankNode("b1")))
    );
}

#[test]
fn test_object_list() {
    let v = vec![
        Object::Literal(literal_true()),
        Object::Literal(literal_11()),
        Object::Literal(literal_false()),
    ];
    assert_eq!(object_list("true, 11 , false "), Ok((" ", v)));
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
    assert_eq!(predicated_objects_list("<urn:123> 1 "), Ok((" ", vec![po])));
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
    assert_eq!(triples("<urn:123> <urn:123> 1 "), Ok((" ", t)));
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
    assert_eq!(statement_triples(": : :."), Ok(("", s)));
}

#[test]
fn test_prefix_id() {
    assert_eq!(
        prefix_id("@prefix a.b.c: <urn> ."),
        Ok(("", Statement::Prefix("a.b.c", "urn")))
    );
    assert_eq!(
        prefix_id("@prefix : <urn> ."),
        Ok(("", Statement::Prefix("", "urn")))
    );
}

#[test]
fn test_base() {
    assert_eq!(base("@base <urn> ."), Ok(("", Statement::Base("urn"))));
}

#[test]
fn test_sparql_base() {
    assert_eq!(sparql_base("BASE <urn>"), Ok(("", Statement::Base("urn"))));
}

#[test]
fn test_sparql_prefix() {
    assert_eq!(
        sparql_prefix("PREFIX a.b.c: <urn>"),
        Ok(("", Statement::Prefix("a.b.c", "urn")))
    );
}

#[test]
fn test_pn_local() {
    assert_eq!(pn_local("c"), Ok(("", "c")));
    // dot does not belong in the pn_local
    assert_eq!(pn_local("c. "), Ok((". ", "c")));
}
