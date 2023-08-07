
use super::*;

// no whitespace
// no newline
// no math $
// no comma , dot
#[rustfmt::skip]
pub static TEXT: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r#"^(?x)(
    [^,.*_$\s\\\[]+
)"#).unwrap()});

impl NoteParser for TextPlainNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&TEXT, "IDENTIFIER")?;
        let number = TextPlainNode::new(m.as_str(), get_span(input, state));
        state.finish(number)
    }
}
