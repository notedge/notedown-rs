use super::*;

impl NoteParser for UriNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, scheme) = IdentifierNode::parse(input)?;
        let (state, _) = state.match_char(':')?;
        let (finish, rest) = state.match_str_if(|c| c.is_whitespace(), "URI_BODY")?;
        finish.finish(UriNode {
            scheme,
            body: TextPlainNode { text: rest.to_string(), span: get_span(state, finish) },
            span: get_span(input, finish),
        })
    }
}
