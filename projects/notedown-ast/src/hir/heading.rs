use super::*;


#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HeadingNode {
    pub level: HeadingLevel,
    pub text: ParagraphNode,
}
