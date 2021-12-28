use super::*;
use crate::traits::IntoASTNode;

/// ## Quote List
/// ```note
/// > part1
/// > part2
///   part2
/// > part3
///
/// > part4
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct QuoteBlock {
    /// Quote style name
    pub style: Option<String>,
    /// Head part
    pub head: ASTNodes,
    /// Body part
    pub body: ASTNodes,
    /// Last quote
    pub quote: Option<String>,
}

impl QuoteBlock {
    /// Constructor of [`QuoteBlock`]
    #[inline]
    pub fn quote(body: ASTNodes) -> Self {
        Self { style: None, head: vec![], body, quote: None }
    }
    /// Constructor of [`QuoteBlock`]
    #[inline]
    pub fn quote_styled(body: ASTNodes, style: String) -> Self {
        Self { style: Some(style), head: vec![], body, quote: None }
    }
}

impl ASTKind {
    /// Constructor of [`QuoteBlock`]
    #[inline]
    pub fn quote(body: ASTNodes, r: MaybeRanged) -> ASTNode {
        QuoteBlock::quote(body).into_node(r)
    }
    /// Constructor of [`QuoteBlock`]
    #[inline]
    pub fn quote_style(body: ASTNodes, style: impl Into<String>, r: MaybeRanged) -> ASTNode {
        QuoteBlock::quote_styled(body, style.into()).into_node(r)
    }
}
