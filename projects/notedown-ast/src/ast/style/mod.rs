use crate::ast::paragraph::ParagraphSpan;
use super::*;




/// `*italic*`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontItalicSpan {
    pub text: ParagraphSpan,
    pub span: Range<u32>,
}

/// `**bold**`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontBoldSpan {
    pub text: ParagraphSpan,
    pub span: Range<u32>,
}

/// `**bold italic**`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontBoldItalicSpan {
    pub text: ParagraphSpan,
    pub span: Range<u32>,
}

/// `_underline_`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontUnderlineSpan {
    pub text: ParagraphSpan,
    pub span: Range<u32>,
}

/// `__delete__`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontDeleteSpan {
    pub text: ParagraphSpan,
    pub span: Range<u32>,
}
