use super::*;

/// `*italic*`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontItalicNode {
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

/// `**bold**`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontBoldNode {
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

/// `**bold italic**`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontBoldItalicNode {
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

/// `_underline_`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontUnderlineNode {
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

/// `__delete__`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontDeleteNode {
    pub text: ParagraphNode,
    pub span: Range<u32>,
}
