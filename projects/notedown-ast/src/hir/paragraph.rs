use super::*;
use crate::ast::TextSpaceNode;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParagraphNode {
    pub terms: Vec<ParagraphKind>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ParagraphKind {
    /// Normal ast with no white space
    Plain(Box<TextPlainNode>),
    /// Normal ast with white space
    Style(Box<TextStyleNode>),
    /// White space
    Space(Box<TextSpaceNode>),
    /// Inline code
    Code(Box<CodeNode>),
}

/// `\.`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextEscapeNode {
    pub escape: char,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextPlainNode {
    pub text: String,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyleNode {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub delete: bool,
    pub text: ParagraphNode,
    pub span: Range<u32>,
    pub color: Option<(u8, u8, u8, u8)>,
}

impl ParagraphKind {
    pub fn text<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self::Plain(Box::new(TextPlainNode::new(body, span)))
    }
}

impl TextEscapeNode {
    pub fn new(escape: char, span: Range<u32>) -> Self {
        Self { escape, span }
    }
}

impl TextPlainNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { text: body.to_string(), span }
    }
}
