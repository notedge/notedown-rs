mod style;

use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{
    ast::{FontBoldItalicSpan, FontBoldSpan, FontItalicSpan, HeadingSpan, NotedownTerm, ParagraphSpan, ParagraphTerm},
    hir::{TextPlainNode, TextSpaceNode},
    CommaNode, NewlineSpan, PeriodNode, TextEscapeNode, WhitespaceSpan,
};
use notedown_error::{helpers::paragraph_break, ParseResult, ParseState, StopBecause};

impl NoteParser for NotedownTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose_from(TextSpaceNode::parse).choose_from(HeadingSpan::parse).choose_from(ParagraphSpan::parse).end_choice()
    }
}

impl NoteParser for TextEscapeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (state, any) = state.match_char_any()?;
        state.finish(Self { escape: any, span: get_span(input, state) })
    }
}

impl NoteParser for ParagraphSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(ParagraphTerm::parse)?;
        if terms.is_empty() {
            StopBecause::missing_string("TEXT_MODE_TERM", input.start_offset)?
        }
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for TextSpaceNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = paragraph_break(input)?;
        state.finish(Self { span: get_span(state, state) })
    }
}

impl NoteParser for ParagraphTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_negative(TextSpaceNode::parse, "PARAGRAPH_BREAK")?;
        state
            .begin_choice()
            .choose_from(FontBoldItalicSpan::parse)
            .choose_from(FontBoldSpan::parse)
            .choose_from(FontItalicSpan::parse)
            .choose_from(TextPlainNode::parse)
            .choose_from(CommaNode::parse)
            .choose_from(PeriodNode::parse)
            .choose_from(WhitespaceSpan::parse)
            .choose_from(NewlineSpan::parse)
            .end_choice()
    }
}

impl NoteParser for CommaNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char(',')?;
        state.finish(Self { span: get_span(input, state) })
    }
}

impl NoteParser for PeriodNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('.')?;
        state.finish(Self { span: get_span(input, state) })
    }
}

impl NoteParser for HeadingSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mark) = input.match_str_if(|c| c == '=', "TITLE_MARK")?;
        let (state, _) = WhitespaceSpan::parse(state)?;
        let (state, text) = ParagraphSpan::parse(state)?;
        state.finish(Self { level: mark.len(), text, span: get_span(input, state) })
    }
}
