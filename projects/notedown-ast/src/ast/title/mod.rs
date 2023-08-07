use super::*;
mod display;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeadingLevel {
    Part,
    Chapter,
    Section,
    Article,
    Header1,
    Header2,
    Header3,
    Header4,
    Header5,
    Header6,
}



#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeadingSpan {
    pub level: HeadingLevel,
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
