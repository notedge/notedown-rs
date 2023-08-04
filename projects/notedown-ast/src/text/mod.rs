mod display;
pub mod paragraph;
pub mod style;
pub mod title;

use crate::{text::title::HeadingNode, CommaNode, NewlineNode, NotedownNode, ParagraphNode, ParagraphSpaceNode, PeriodNode, WhitespaceNode};
use deriver::From;
use std::{
    fmt::{Debug, Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLiteralNode {
    pub text: String,
    pub span: Range<u32>,
}

/// The root node of all notedown terms
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NotedownAST {
    pub terms: Vec<NotedownTerm>,
    pub span: Range<u32>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NotedownTerm {
    Heading(Box<HeadingNode>),
    Paragraph(Box<ParagraphNode>),
    SpaceBreak(Box<ParagraphSpaceNode>),
}

/// `\.`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextEscapeNode {
    pub escape: char,
    pub span: Range<u32>,
}

impl TextLiteralNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { text: body.to_string(), span }
    }
}
