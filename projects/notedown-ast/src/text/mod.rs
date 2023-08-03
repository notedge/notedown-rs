pub mod title;

use crate::{CommaNode, NewlineNode, PeriodNode, WhitespaceNode};
use deriver::From;
use std::ops::Range;

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

#[derive(Clone, Debug, Eq, PartialEq, Hash, From)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextModeTerm {
    Text(Box<TextLiteralNode>),
    WhiteSpace(Box<WhitespaceNode>),
    NewLine(Box<NewlineNode>),
    Comma(Box<CommaNode>),
    Period(Box<PeriodNode>),
}

impl TextLiteralNode {
    pub fn new<S: ToString>(body: S, span: Range<u32>) -> Self {
        Self { text: body.to_string(), span }
    }
}
