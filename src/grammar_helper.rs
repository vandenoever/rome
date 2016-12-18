use nom::IResult;
use nom::ErrorKind;
use nom::Needed;
use std::char;
use std::str::Chars;

pub fn string_literal(str: &str,
                      ql: usize,
                      starts_with: fn(&str) -> bool,
                      find: fn(&str) -> Option<usize>)
                      -> IResult<&str, String> {
    if !starts_with(str) {
        return IResult::Error(ErrorKind::Custom(0));
    }
    let hay = &str[ql..];
    if starts_with(hay) {
        return IResult::Done(&hay[ql..], String::new());
    }
    let mut offset = 0;
    loop {
        let left = &hay[offset..];
        if let Some(i) = find(left) {
            offset += i;
            if !escaped(hay.as_bytes(), offset) {
                break;
            }
            offset += 1;
        } else {
            return IResult::Incomplete(Needed::Unknown);
        }
    }
    nom_unescape(&hay[offset + ql..], &hay[..offset])
}

fn nom_unescape<'a>(left: &'a str, data: &str) -> IResult<&'a str, String> {
    match unescape(data) {
        Some(result) => IResult::Done(left, result),
        None => return IResult::Error(ErrorKind::Custom(0)),
    }
}

fn escaped(hay: &[u8], offset: usize) -> bool {
    let mut p = offset;
    while p != 0 && hay[p - 1] == '\\' as u8 {
        p -= 1;
    }
    (offset - p) % 2 == 1
}

fn acc(acc: Option<(u32, u8)>, c: char) -> Option<(u32, u8)> {
    acc.and_then(|(acc, n)| c.to_digit(16).and_then(|c| Some(((acc << 4) + c, n + 1))))
}

fn hex_to_char(chars: &mut Chars, n: u8) -> Option<char> {
    chars.by_ref()
        .take(n as usize)
        .fold(Some((0, 0)), acc)
        .and_then(|(ch, count)| if count == n { char::from_u32(ch) } else { None })
}

/// [26] UCHAR ::= '\u' HEX HEX HEX HEX | '\U' HEX HEX HEX HEX HEX HEX HEX HEX
/// [159s] ECHAR ::= '\' [tbnrf"'\]
pub fn unescape(s: &str) -> Option<String> {
    let mut result = String::with_capacity(s.len());
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
                ch => ch, // ', " and \ are simply copied
            };
            match r {
                Some(v) => result.push(v),
                None => return None,
            }
        } else {
            result.push(ch)
        }
    }
    Some(result)
}

pub fn pn_local_unescape(s: &str) -> Option<String> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // simply remove \
            match chars.next() {
                Some(v) => result.push(v),
                None => return None,
            }
        } else if ch == '%' {
            match hex_to_char(&mut chars, 2) {
                Some(v) => result.push(v),
                None => return None,
            }
        } else {
            result.push(ch)
        }
    }
    Some(result)
}
