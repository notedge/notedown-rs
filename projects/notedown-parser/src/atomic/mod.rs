use std::ops::Range;
use std::sync::LazyLock;
use pex::{ParseResult, ParseState, Regex};
use crate::traits::ThisParser;


pub struct NotedownKind {}

pub struct NotedownNode {
    pub kind: NotedownKind,
    pub span: Range<u32>,
    pub rest: Vec<NotedownIgnore>,
    pub children: Vec<NotedownNode>,
}

pub struct WhitespaceNode {

}

pub struct NewlineNode {

}

pub struct IdentifierNode {
    name: String,
    span: Range<u32>,
    rest: Vec<NotedownIgnore>,
}

impl IdentifierNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self {
            name: body.to_string(),
            span,
            rest: vec![],
        }
    }
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
        let id = IdentifierNode::new(m.as_str(), start..end);
        state.finish(id)
    }
}

fn test() {
    IdentifierNode::
}