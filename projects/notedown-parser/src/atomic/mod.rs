use crate::{helpers::get_span, traits::ThisParser};
use notedown_ast::{CommandNode, IdentifierNode, NewlineNode, WhitespaceNode};
use pex::{ParseResult, ParseState, Regex};
use std::sync::LazyLock;

pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?x)(
      [∞∅]
    | (?P<regular>(?:\p{XID_Start}|_)\p{XID_Continue}*)
    | `(?P<escaped>(?:\\.|[^`])*)`
)",
    )
    .unwrap()
});

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
        let state = input.advance(offset);
        state.finish(NewlineNode::new(lines, get_span(input, state)))
    }
}

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str());
        state.finish(id)
    }
}

impl ThisParser for CommandNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (cmd, id) = state.match_fn(IdentifierNode::parse)?;
        cmd.finish(CommandNode::new(id.name, get_span(input, cmd)))
    }
}

// impl ThisParser for CommandArguments {
//     fn parse(input: ParseState) -> ParseResult<Self> {
//         let pat = BracketPattern::new("(", ")");
//         pat.consume(input.skip(ignore), ignore, GenericArgumentTerm::parse)
//     }
// }

#[test]
fn test() {
    let id = WhitespaceNode::parse_text("  ").unwrap();
    println!("{}", id);
    println!("{:#?}", id);
}
