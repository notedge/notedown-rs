use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphNode {
    pub terms: Vec<ParagraphKind>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontStyleNode {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphKind {
    /// Normal ast with white space
    Style(Box<FontStyleNode>)
}
