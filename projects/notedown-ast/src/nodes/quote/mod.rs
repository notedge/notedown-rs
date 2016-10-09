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
    pub style: Option<String>,
    pub head: ASTNodes,
    pub body: ASTNodes,
    pub quote: Option<String>,
}

impl QuoteBlock {
    #[inline]
    pub fn quote(body: ASTNodes) -> Self {
        Self { style: None, head: vec![], body, quote: None }
    }
    #[inline]
    pub fn quote_styled(body: ASTNodes, style: String) -> Self {
        Self { style: Some(style), head: vec![], body, quote: None }
    }
}

impl ASTKind {
    #[inline]
    pub fn quote(body: ASTNodes, r: MaybeRanged) -> ASTNode {
        QuoteBlock::quote(body).into_node(r)
    }
    #[inline]
    pub fn quote_style(body: ASTNodes, style: impl Into<String>, r: MaybeRanged) -> ASTNode {
        QuoteBlock::quote_styled(body, style.into()).into_node(r)
    }
}
