use crate::{
    helpers::{get_span, ignore},
    traits::ThisParser,
};
use notedown_ast::{CommandArguments, CommandNode, IdentifierNode};
use pex::{BracketPattern, ParseResult, ParseState, Regex};
use std::{ops::Range, sync::LazyLock};

pub struct NotedownKind {}

pub struct NotedownNode {
    pub kind: NotedownKind,
    pub span: Range<u32>,
    pub children: Vec<NotedownNode>,
}

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

impl ThisParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let start = input.start_offset as u32;
        let end = (input.start_offset + m.range().end) as u32;
        let id = IdentifierNode::new(m.as_str());
        state.finish(id)
    }
}

impl ThisParser for CommandNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (state, id) = state.match_fn(IdentifierNode::parse)?;
        state.finish(CommandNode::new(id.name, get_span(input, state)))
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
    let id = CommandNode::parse_text("\\abc ").unwrap();
    println!("{}", id);
    println!("{:#?}", id);
}
