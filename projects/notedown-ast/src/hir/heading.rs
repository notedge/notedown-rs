use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeadingNode {
    pub level: HeadingLevel,
    pub id: String,
    pub span: Range<u32>,
}

impl HeadingSpan {
    pub fn as_hir(&self) -> HeadingNode {
        HeadingNode {
            level: HeadingLevel::from(self.level),
            id: "".to_string(),
            span: self.span.clone(),
        }
    }
}