use super::*;

impl NoteParser for IgnoreNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose_from(WhitespaceSpan::parse).choose_from(NewlineSpan::parse).end_choice()
    }
}

impl NoteParser for WhitespaceSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut offset = 0;
        let mut width = 0;
        for c in input.residual.chars() {
            match c {
                '\t' => width += 4,
                ' ' => width += 1,
                _ => break,
            }
            offset += c.len_utf8();
        }
        if offset == 0 {
            return StopBecause::missing_character(' ', input.start_offset)?;
        }
        let state = input.advance(offset);
        state.finish(WhitespaceSpan::new(width, get_span(input, state)))
    }
}

impl NoteParser for NewlineSpan {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input
            .begin_choice()
            .choose(|c| c.match_str("\r\n")) // CRLF
            .choose(|c| c.match_str("\n")) // LF
            .choose(|c| c.match_str("\r")) // CR
            .end_choice()?;
        state.finish(NewlineSpan { span: get_span(input, state) })
    }
}
