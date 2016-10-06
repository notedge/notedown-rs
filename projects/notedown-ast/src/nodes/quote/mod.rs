use super::*;

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
