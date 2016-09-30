use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct QuoteBlock {
    style: Option<String>,
    head: ASTNodes,
    body: ASTNodes,
    quote: Option<String>,
}

impl QuoteBlock {
    #[inline]
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        ASTNode { value: ASTKind::QuoteNode(box self), range }
    }
    #[inline]
    pub fn quote() -> Self {
        Self { style: None, head: vec![], body: vec![], quote: None }
    }
    #[inline]
    pub fn quote_style(style: String) -> Self {
        Self { style: Some(style), head: vec![], body: vec![], quote: None }
    }
}

impl ASTNode {
    pub fn quote() {}
}
