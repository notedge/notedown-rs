use super::*;
use pex::StopBecause;

impl ThisParser for IgnoreNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.begin_choice().choose_from(WhitespaceNode::parse).choose_from(NewlineNode::parse).end_choice()
    }
}

impl ThisParser for WhitespaceNode {
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
        state.finish(WhitespaceNode::new(width, get_span(input, state)))
    }
}

impl ThisParser for NewlineNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let mut offset = 0;
        let mut lines = 0;
        let mut chars = input.residual.chars();
        while let Some(c) = chars.next() {
            match c {
                '\n' => lines += 1,
                '\r' => {
                    match chars.next() {
                        Some('\n') => lines += 1,
                        Some('\r') => lines += 2,
                        _ => break,
                    }
                    offset += 1;
                }
                _ => break,
            }
            offset += 1;
        }
        if offset == 0 {
            return StopBecause::missing_character('\n', input.start_offset)?;
        }
        let state = input.advance(offset);
        state.finish(NewlineNode::new(lines, get_span(input, state)))
    }
}
