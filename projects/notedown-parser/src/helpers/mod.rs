use std::sync::LazyLock;
use pex::{ParseResult, ParseState};
use pex::ParseResult::{Pending, Stop};

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