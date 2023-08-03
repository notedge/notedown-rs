pub mod title;

use crate::{text::title::HeadingNode, CommaNode, NewlineNode, ParagraphSpaceNode, PeriodNode, WhitespaceNode};
use deriver::From;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLiteralNode {
    pub text: String,
    pub span: Range<u32>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextModeNode {
    pub terms: Vec<TextModeTerm>,
    pub span: Range<u32>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextModeTerm {
    Heading(Box<HeadingNode>),
    Paragraph(Box<ParagraphNode>),
    SpaceBreak(Box<ParagraphSpaceNode>),
}

impl Debug for TextModeTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModeTerm::Heading(v) => Debug::fmt(v, f),
            TextModeTerm::Paragraph(v) => Debug::fmt(v, f),
            TextModeTerm::SpaceBreak(v) => Debug::fmt(v, f),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphNode {
    pub terms: Vec<ParagraphTerm>,
    pub span: Range<u32>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphTerm {
    Text(Box<TextLiteralNode>),
    WhiteSpace(Box<WhitespaceNode>),
    NewLine(Box<NewlineNode>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
}

impl Debug for ParagraphTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphTerm::Text(v) => Debug::fmt(v, f),
            ParagraphTerm::WhiteSpace(v) => Debug::fmt(v, f),
            ParagraphTerm::NewLine(v) => Debug::fmt(v, f),
            ParagraphTerm::Comma(v) => Debug::fmt(v, f),
            ParagraphTerm::Period(v) => Debug::fmt(v, f),
        }
    }
}

impl TextLiteralNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { text: body.to_string(), span }
    }
}
