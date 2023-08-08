use notedown_error::{
    ParseResult,
    ParseResult::{Pending, Stop},
    ParseState, Regex,
};
use std::{ops::Range, sync::LazyLock};

pub static IGNORE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
    # whitespace
      \s
    # comments
    | \# [^\r\n]*
)*",
    )
    .unwrap()
});

/// Ignores whitespace and comments.
#[inline]
pub fn ignore<'i>(input: ParseState<'i>) -> ParseResult<&'i str> {
    match input.match_regex(&IGNORE, "IGNORE") {
        Pending(_, a) => input.advance_view(a.len()),
        Stop(_) => input.finish(""),
    }
}

#[inline]
pub fn get_span(input: ParseState, output: ParseState) -> Range<u32> {
    let range = output.away_from(input);
    (range.start as u32)..(range.end as u32)
}

/// Parse the rest of the line, note this does not catch the newline,
#[inline]
pub fn rest_of_line(input: ParseState) -> ParseResult<&str> {
    let offset = match input.residual.find(&['\r', '\n']) {
        Some(s) => s,
        None => input.residual.len(),
    };
    input.advance_view(offset)
}
