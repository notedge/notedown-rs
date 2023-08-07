// lints
#![warn(missing_docs)]
#![allow(clippy::needless_return)]
// documentation
#![doc = include_str!("../Readme.md")]

mod atomics;
pub mod hir;
mod link;
pub mod ast;
mod traits;

pub use crate::{
    atomics::{
        command::{CommandArguments, CommandBody, CommandNode},
        identifier::{AlignNode, IdentifierNode, LigatureNode, NumberLiteralNode, NumberValueNode},
        punctuation::{CommaNode, PeriodNode},
        whitespace::{HSpaceNode, IgnoreNode, NewlineNode, ParagraphSpaceNode, VSpaceNode, WhitespaceNode},
    },
    ast::{
        paragraph::{ParagraphNode, ParagraphTerm},
        style::{FontBoldItalicNode, FontBoldNode, FontDeleteNode, FontItalicNode, FontUnderlineNode},
        NotedownAST, NotedownTerm, TextEscapeNode, TextLiteralNode,
    },
    traits::{NoteGenerator, NoteOptimizer},
};
