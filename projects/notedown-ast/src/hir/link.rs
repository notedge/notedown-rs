use super::*;

/// `\w+:(\/?\/?)[^\s]+`
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UriNode {
    pub scheme: IdentifierNode,
    pub body: TextPlainNode,
    pub span: Range<u32>,
}
