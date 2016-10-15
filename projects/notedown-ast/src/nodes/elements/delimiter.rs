use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Delimiter {
    HorizontalRule,
    HTMLRawBlock(String),
}

impl ASTKind {
    pub fn raw_html_inline(msg: impl Into<String>, range: MaybeRanged) -> ASTNode {
        Delimiter::HTMLRawBlock(msg.into()).into_node(range)
    }
    pub fn raw_html_block(msg: impl Into<String>, range: MaybeRanged) -> ASTNode {
        TextKind::HTMLRawInline(msg.into()).into_node(range)
    }
}
