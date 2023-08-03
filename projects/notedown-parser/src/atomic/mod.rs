mod identifier;
mod ignore;
mod number;
mod text;

use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{
    CommandNode, IdentifierNode, IgnoreNode, LigatureNode, NewlineNode, NumberLiteralNode, NumberValueNode, TextNode, WhitespaceNode,
};
use pex::{ParseResult, ParseState, Regex};
use std::sync::LazyLock;

impl NoteParser for CommandNode {
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
    let id = IgnoreNode::parse(ParseState::new("\n\r\n "));
    // println!("{}", id);
    println!("{:#?}", id);
}

#[test]
fn test2() {
    let id = TextNode::parse(ParseState::new("a_b, good"));
    // println!("{}", id);
    println!("{:#?}", id);
}
