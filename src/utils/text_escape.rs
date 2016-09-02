use std::{intrinsics::transmute,};
use std::collections::VecDeque;

// Takes in a string with backslash escapes written out with literal backslash characters and
// converts it to a string with the proper escaped characters.
pub fn unescape(text: impl AsRef<str>)-> Option<String> {
    let mut queue : VecDeque<_> = String::from(text.as_ref()).chars().collect();
    let mut out = String::with_capacity(text.as_ref().len());
    while let Some(c) = queue.pop_front() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match queue.pop_front() {
            Some('b') => out.push('\u{0008}'),
            Some('f') => out.push('\u{000C}'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\'') => out.push('\''),
            Some('\"') => out.push('\"'),
            Some('\\') => out.push('\\'),
            _ => return None
        };
    }
    Some(out)
}

pub fn unescape_utf8(text: impl AsRef<str>) {
    unimplemented!()
}

pub fn unescape_only(text: impl AsRef<str>, c: char) {
    unimplemented!()
}

pub fn unescape_hex_chars(text: impl AsRef<str>) -> Option<String> {
    unsafe { unescape_unicode_char(text.as_ref(), 16) }
}
pub fn unescape_dec_chars(text: impl AsRef<str>) -> Option<String> {
    unsafe { unescape_unicode_char(text.as_ref(), 10) }
}

unsafe fn unescape_unicode_char(text: &str, radix: u32) -> Option<String> {
    let mut out = String::with_capacity(text.len());
    for c in text.split_whitespace() {
        match u32::from_str_radix(c, radix) {
            Ok(o) => out.push(transmute::<u32, char>(o)),
            Err(_) => return None,
        }
    }
    Some(out)
}
