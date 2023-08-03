use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{text::title::HeadingNode, CommaNode, NewlineNode, PeriodNode, TextLiteralNode, TextModeNode, TextModeTerm, WhitespaceNode};
use pex::{ParseResult, ParseState};

impl NoteParser for TextModeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(TextModeTerm::parse)?;
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for TextModeTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose_from(TextLiteralNode::parse)
            .choose_from(CommaNode::parse)
            .choose_from(PeriodNode::parse)
            .choose_from(WhitespaceNode::parse)
            .choose_from(NewlineNode::parse)
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

impl NoteParser for HeadingNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, mark) = input.match_str_if(|c| c == '=', "TITLE_MARK")?;
        let (state, _) = WhitespaceNode::parse(state)?;
        let (state, text) = TextModeNode::parse(state)?;
        state.finish(Self { level: mark.len(), text, span: get_span(input, state) })
    }
}
