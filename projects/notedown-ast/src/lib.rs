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
        style::{FontBoldItalicSpan, FontBoldSpan, FontDeleteSpan, FontItalicSpan, FontUnderlineSpan},
        NotedownAST, NotedownTerm, TextEscapeNode,
    },
    traits::{NoteGenerator, NoteOptimizer},
};

#[cfg(feature = "html-ast")]
pub use crate::traits::html::HtmlBuilder;
#[cfg(feature = "html-ast")]
pub use html_ast::HtmlElement;