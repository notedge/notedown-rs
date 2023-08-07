use notedown_ast::{FontBoldItalicSpan, FontBoldSpan, FontItalicSpan};
use super::*;


impl NoteParser for FontBoldItalicSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("***")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("***")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for FontBoldSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("**")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("**")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for FontItalicSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("*")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("*")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}