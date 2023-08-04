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
    /// Normal text with white space
    Text(Box<TextLiteralNode>),
    WhiteSpace(Box<WhitespaceNode>),
    NewLine(Box<NewlineNode>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
    Escape(Box<TextEscapeNode>),
}
