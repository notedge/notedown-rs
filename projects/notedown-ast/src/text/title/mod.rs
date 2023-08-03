use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeadingNode {
    pub level: usize,
    pub text: ParagraphNode,
    pub span: Range<u32>,
}

/// need in start of line
///
/// ```note
/// 
/// ===
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HorizontalRuleNode {
    pub marks: usize,
    pub span: Range<u32>,
}
