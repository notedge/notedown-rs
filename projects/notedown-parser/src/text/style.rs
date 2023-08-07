use super::*;
use notedown_ast::ast::{CodeInlineSpan, FontBoldItalicSpan, FontBoldSpan, FontDeleteSpan, FontItalicSpan, FontUnderlineSpan};

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

impl NoteParser for FontDeleteSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("__")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("__")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for FontUnderlineSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_str("_")?;
        let (state, text) = ParagraphSpan::parse(state)?;
        let (state, _) = state.match_str("_")?;
        state.finish(Self { text, span: get_span(input, state) })
    }
}

impl NoteParser for CodeInlineSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut head = 0;
        for i in input.residual.chars() {
            if i == '`' { head += 1 } else { break }
        }
        if head == 0 {
            StopBecause::missing_character('`', input.start_offset)?
        }
        let state = input.advance(head);
        match state.residual.find(&"`".repeat(head)) {
            Some(tail) => {
                let (state, text) = state.advance_view(tail)?;
                let finish = state.advance(head);
                finish.finish(Self { level: head, code: text.to_string(), span: get_span(input, finish) })
            }
            None => StopBecause::missing_character('`', state.end_offset())?,
        }
    }
}
