use super::*;

/// `*italic*`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MathSpan {
    pub display: bool,
    pub text: Vec<ParagraphTerm>,
    pub span: Range<u32>,
}
