use std::ops::Range;
use std::sync::LazyLock;
use pex::{BracketPattern, ParseResult, ParseState, Regex};
use crate::helpers::{get_span, ignore};
use crate::traits::ThisParser;


pub struct NotedownKind {}

pub struct NotedownNode {
    pub kind: NotedownKind,
    pub span: Range<u32>,
    pub children: Vec<NotedownNode>,
}

pub struct WhitespaceNode {}

pub struct NewlineNode {}


pub struct NumberNode {
    number: String,
    span: Range<u32>,
}

#[derive(Debug)]
pub struct IdentifierNode {
    name: String,
    span: Range<u32>,
}

///CommandNode
///
/// ```note
/// \cmd () { }
/// ```
#[derive(Debug)]
pub struct CommandNode {
    name: String,
    span: Range<u32>,
}
///CommandNode
///
/// ```note
/// ()
/// ```
#[derive(Debug)]
pub struct CommandArguments {}

///CommandNode
///
/// ```note
/// { }
/// ```
#[derive(Debug)]
pub struct CommandBody {

}

impl IdentifierNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self {
            name: body.to_string(),
            span,
        }
    }
}

impl CommandNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self {
            name: body.to_string(),
            span,
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

impl ThisParser for CommandNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, _) = input.match_char('\\')?;
        let (state, id) = state.match_fn(IdentifierNode::parse)?;
        state.finish(CommandNode::new(id.name, get_span(input, state)))
    }
}

impl ThisParser for CommandArguments {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let pat = BracketPattern::new("(", ")");
        pat.consume(input.skip(ignore), ignore, GenericArgumentTerm::parse)
    }
}



#[test]
fn test() {
    let id = CommandNode::parse_text("\\abc");
    println!("{:#?}", id);
}