use super::*;

// 0
// 123.4
#[rustfmt::skip]
pub static LITERAL: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r"^(?x)(
    [0-9]+(\.[0-9]+)?
)").unwrap()});

impl NoteParser for NumberLiteralNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&LITERAL, "IDENTIFIER")?;
        let number = NumberLiteralNode::new(m.as_str(), get_span(input, state));
        state.finish(number)
    }
}

// 0
// 123.4
#[rustfmt::skip]
pub static VALUE: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r"^(?x)(
    0 
|   [1-9][0-9]*(\.[0-9]+)?
)").unwrap()});

impl NoteParser for NumberValueNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&LITERAL, "IDENTIFIER")?;
        let (state, unit) = state.match_optional(IdentifierNode::parse)?;
        let number = NumberValueNode { value: m.as_str().to_string(), unit, span: get_span(input, state) };
        state.finish(number)
    }
}
