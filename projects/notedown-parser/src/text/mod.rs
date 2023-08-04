use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{
    text::{title::HeadingNode, NotedownAST, NotedownTerm},
    CommaNode, NewlineNode, ParagraphNode, ParagraphSpaceNode, ParagraphTerm, PeriodNode, TextEscapeNode, TextLiteralNode, WhitespaceNode,
};
use pex::{helpers::paragraph_break, ParseResult, ParseState, StopBecause};

impl NoteParser for NotedownAST {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(NotedownTerm::parse)?;
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for TextEscapeNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (state, any) = state.match_char_any()?;
        state.finish(Self { escape: any, span: get_span(input, state) })
    }
}

impl NoteParser for NotedownTerm {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .choose_from(ParagraphSpaceNode::parse)
            .choose_from(HeadingNode::parse)
            .choose_from(ParagraphNode::parse)
            .end_choice()
    }
}

impl NoteParser for ParagraphNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, terms) = input.match_repeats(ParagraphTerm::parse)?;
        if terms.is_empty() {
            StopBecause::missing_string("TEXT_MODE_TERM", input.start_offset)?
        }
        state.finish(Self { terms, span: get_span(input, state) })
    }
}

impl NoteParser for ParagraphSpaceNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = paragraph_break(input)?;
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
