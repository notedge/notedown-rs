use super::*;

// no punctuation
// no newline
// no math $
// no comma , dot
#[rustfmt::skip]
pub static TEXT: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r#"^(?x)(
    [^,.$\s\\\[]+
)"#).unwrap()});

impl NoteParser for TextNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&TEXT, "IDENTIFIER")?;
        let number = TextNode::new(m.as_str(), get_span(input, state));
        state.finish(number)
    }
}
