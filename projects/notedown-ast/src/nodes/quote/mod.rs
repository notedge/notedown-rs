use super::*;

pub struct QuoteBlock {
    style: Option<String>,
    head: ASTNodes,
    body: ASTNodes,
    quote: Option<String>,
}

impl QuoteBlock {
    pub fn quote() {}
    pub fn warn() {}
    pub fn info() {}
}
