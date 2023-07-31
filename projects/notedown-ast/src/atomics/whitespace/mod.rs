pub use super::*;
mod display;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IgnoreNode {
    WS(WhitespaceNode),
    NL(NewlineNode),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhitespaceNode {
    width: usize,
    span: Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NewlineNode {
    count: usize,
    span: Range<u32>,
}

impl WhitespaceNode {
    pub fn new(width: usize, span: Range<u32>) -> Self {
        Self { width, span }
    }
}

impl NewlineNode {
    pub fn new(lines: usize, span: Range<u32>) -> Self {
        Self { count: lines, span }
    }
}
