use crate::{FontBoldItalicNode, FontBoldNode, FontItalicNode};
use super::*;
mod display;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphNode {
    pub terms: Vec<ParagraphTerm>,
    pub span: Range<u32>,
}

#[derive(Clone, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphTerm {
    /// Normal ast with white space
    Text(Box<TextLiteralNode>),
    WhiteSpace(Box<WhitespaceNode>),
    /// `*italic*`
    Italic(Box<FontItalicNode>),
    /// `**bold**`
    Bold(Box<FontBoldNode>),
    /// `**bold italic**`
    BoldItalic(Box<FontBoldItalicNode>),
    NewLine(Box<NewlineNode>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
    Escape(Box<TextEscapeNode>),
}
