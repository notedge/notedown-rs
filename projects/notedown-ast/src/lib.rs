// lints
#![warn(missing_docs)]
#![allow(clippy::needless_return)]
// documentation
#![doc = include_str!("../Readme.md")]

mod atomics;
pub mod text;
mod traits;

pub use crate::{
    atomics::{
        command::{CommandArguments, CommandBody, CommandNode},
        identifier::{AlignNode, IdentifierNode, LigatureNode, NumberLiteralNode, NumberValueNode, TextLiteralNode},
        punctuation::{CommaNode, PeriodNode},
        whitespace::{HSpaceNode, IgnoreNode, NewlineNode, VSpaceNode, WhitespaceNode},
    },
    text::{TextModeNode, TextModeTerm},
    traits::NotedownNode,
};
