mod identifier;
mod ignore;
mod number;
mod text;

use crate::{helpers::get_span, traits::NoteParser};
use notedown_ast::{
    ast::{IgnoreNode, NewlineSpan, TextSpaceNode},
    hir::{IdentifierNode, TextPlainNode},
    CommandNode, LigatureNode, NumberLiteralNode, NumberValueNode,
};
use notedown_error::{ParseResult, ParseState, Regex, StopBecause};
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
