use crate::error::{Error, Result};
use nom::types::CompleteStr;
use nom::ErrorKind;
use nom::Needed;
use nom::{Context, Err, IResult};
use std::char;
use std::str::Chars;

pub fn string_literal(
    str: CompleteStr,
    ql: usize,
    starts_with: fn(CompleteStr) -> bool,
    find: fn(CompleteStr) -> Option<usize>,
) -> IResult<CompleteStr, &str> {
    if !starts_with(str) {
        return Err(Err::Error(Context::Code(str, ErrorKind::Custom(0))));
    }
    let hay = CompleteStr(&str[ql..]);
    if starts_with(hay) {
        return Ok((CompleteStr(&hay[ql..]), ""));
    }
    let mut offset = 0;
    loop {
        let left = CompleteStr(&hay[offset..]);
        if let Some(i) = find(left) {
            offset += i;
            if !escaped(hay.as_bytes(), offset) {
                break;
            }
            offset += 1;
        } else {
            return Err(Err::Incomplete(Needed::Unknown));
        }
    }
    Ok((CompleteStr(&hay[offset + ql..]), &hay[..offset]))
}

fn escaped(hay: &[u8], offset: usize) -> bool {
    let mut p = offset;
    while p != 0 && hay[p - 1] == b'\\' {
        p -= 1;
    }
    (offset - p) % 2 == 1
}

fn acc(acc: Option<(u32, u8)>, c: char) -> Option<(u32, u8)> {
    acc.and_then(|(acc, n)| c.to_digit(16).and_then(|c| Some(((acc << 4) + c, n + 1))))
}

fn hex_to_char(chars: &mut Chars, n: u8) -> Option<char> {
    chars
        .by_ref()
        .take(n as usize)
        .fold(Some((0, 0)), acc)
        .and_then(|(ch, count)| if count == n { char::from_u32(ch) } else { None })
}

/// [26] UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX
/// [159s] ECHAR ::= '\' [tbnrf"'\]
pub fn unescape(s: &str, result: &mut String) -> Result<()> {
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let r = match chars.next() {
                Some('u') => hex_to_char(&mut chars, 4),
                Some('U') => hex_to_char(&mut chars, 8),
                Some('t') => Some('\t'),
                Some('b') => Some('\x08'),
                Some('n') => Some('\n'),
                Some('r') => Some('\r'),
                Some('f') => Some('\x0c'),
                Some('\'') => Some('\''),
                Some('"') => Some('"'),
                Some('\\') => Some('\\'),
                _ => return Err(Error::Custom("Invalid escape sequence")),
            };
            match r {
                Some(v) => result.push(v),
                None => return Err(Error::Custom("Unclosed escape sequence")),
            }
        } else {
            result.push(ch)
        }
    }
    Ok(())
}

pub fn unescape_iri(s: &str, result: &mut String) -> Result<()> {
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            let r = match chars.next() {
                Some('u') => hex_to_char(&mut chars, 4),
                Some('U') => hex_to_char(&mut chars, 8),
                _ => return Err(Error::Custom("Invalid escape sequence")),
            };
            match r {
                Some(v) if v <= ' ' || "<>\"{}|^`\\".contains(v) => {
                    return Err(Error::Custom("Invalid character in IRI."))
                }
                Some(v) => result.push(v),
                None => return Err(Error::Custom("Incomplete escape sequence")),
            }
        } else {
            result.push(ch)
        }
    }
    Ok(())
}

pub fn pn_local_unescape(s: &str, result: &mut String) -> Result<()> {
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // simply remove \
            match chars.next() {
                Some(v) => result.push(v),
                None => return Err(Error::Custom("Error escaping local")),
            }
        } else {
            result.push(ch)
        }
    }
    Ok(())
}
