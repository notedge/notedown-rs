use super::*;

impl NoteParser for UriNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, scheme) = IdentifierNode::parse(input)?;
        let (state, _) = state.match_char(':')?;
        let (finish, rest) = state.skip(double_slash).match_str_until(|c| c.is_whitespace(), "URI_BODY")?;
        finish.finish(UriNode {
            scheme,
            body: TextPlainNode { text: rest.to_string(), span: get_span(state, finish) },
            span: get_span(input, finish),
        })
    }
}

fn double_slash(input: ParseState) -> ParseResult<&str> {
    input.match_str("//")
}
