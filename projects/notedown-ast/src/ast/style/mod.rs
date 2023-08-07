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

impl FontItalicSpan {
    pub fn as_hir(&self) -> TextStyleNode {
        TextStyleNode { italic: true, bold: false, underline: false, text: self.text.as_hir(), span: self.span.clone(), color: None }
    }
}

impl FontBoldSpan {
    pub fn as_hir(&self) -> TextStyleNode {
        TextStyleNode { italic: false, bold: true, underline: false, text: self.text.as_hir(), span: self.span.clone(), color: None }
    }
}

impl FontBoldItalicSpan {
    pub fn as_hir(&self) -> TextStyleNode {
        TextStyleNode { italic: true, bold: true, underline: false, text: self.text.as_hir(), span: self.span.clone(), color: None }
    }
}
