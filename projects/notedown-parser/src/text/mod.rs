use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{
    text::{title::HeadingNode, TextModeNode, TextModeTerm},
    CommaNode, NewlineNode, ParagraphNode, ParagraphSpaceNode, ParagraphTerm, PeriodNode, TextLiteralNode, WhitespaceNode,
};
use pex::{helpers::paragraph_break, ParseResult, ParseState, StopBecause};

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
            .choose_from(ParagraphSpaceNode::parse)
            .choose_from(ParagraphNode::parse)
            .choose_from(HeadingNode::parse)
            .end_choice()
    }
}

impl NoteParser for ParagraphNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(ParagraphTerm::parse)?;
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for ParagraphSpaceNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, input) = paragraph_break(input)?;
        state.finish(Self { span: get_span(state, state) })
    }
}

impl NoteParser for ParagraphTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_negative(ParagraphSpaceNode::parse, "PARAGRAPH_BREAK")?;
        state
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
        let (state, text) = ParagraphNode::parse(state)?;
        state.finish(Self { level: mark.len(), text, span: get_span(input, state) })
    }
}
