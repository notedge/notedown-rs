use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphNode {
    pub terms: Vec<ParagraphKind>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphKind {
    Plain(Box<TextPlainNode>),
    /// Normal ast with white space
    Style(Box<TextStyleNode>),

    Space(Box<TextSpaceNode>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPlainNode {
    pub text: String,
    pub span: Range<u32>,
}

/// A period of whitespace longer than two newlines, terminated by a newline
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextSpaceNode {
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyleNode {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub text: ParagraphNode,
    pub span: Range<u32>,
    pub color: Option<(u8, u8, u8, u8)>,
}
