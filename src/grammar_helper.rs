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
    match unescape(&hay[..offset]) {
        Some(result) => IResult::Done(&hay[offset + ql..], result),
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

fn acc(acc: Option<u32>, c: char) -> Option<u32> {
    acc.and_then(|acc| c.to_digit(16).and_then(|c| Some(16 * acc + c)))
}

fn hex_to_char(chars: &mut Chars, n: usize) -> Option<char> {
    chars.by_ref()
        .take(n)
        .fold(Some(0), acc)
        .and_then(char::from_u32)
}

fn unescape(s: &str) -> Option<String> {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            result.push(ch)
        } else {
            let r = match chars.next() {
                Some('u') => hex_to_char(&mut chars, 4),
                Some('U') => hex_to_char(&mut chars, 8),
                Some('b') => Some('\x08'),
                Some('f') => Some('\x0c'),
                Some('n') => Some('\n'),
                Some('r') => Some('\r'),
                Some('t') => Some('\t'),
                ch => ch,
            };
            match r {
                Some(v) => result.push(v),
                None => return None,
            }
        }
    }
    Some(result)
}
