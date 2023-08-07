use notedown_ast::{FontBoldItalicNode, FontBoldNode, FontItalicNode};
use super::*;


impl NoteParser for FontBoldItalicNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("***")?;
        let (state, text) = ParagraphNode::parse(state)?;
        let (state, _) = state.match_str("***")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for FontBoldNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("**")?;
        let (state, text) = ParagraphNode::parse(state)?;
        let (state, _) = state.match_str("**")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for FontItalicNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("*")?;
        let (state, text) = ParagraphNode::parse(state)?;
        let (state, _) = state.match_str("*")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}