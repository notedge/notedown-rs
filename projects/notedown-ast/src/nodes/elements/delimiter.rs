use super::*;

/// TODO: doc
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Delimiter {
    /// TODO: doc
    HorizontalRule,
    /// TODO: doc
    HTMLRawBlock(String),
}

impl ASTKind {
    /// Insert raw html text
    pub fn raw_html_inline(msg: impl Into<String>, range: MaybeRanged) -> ASTNode {
        Delimiter::HTMLRawBlock(msg.into()).into_node(range)
    }
    /// Inset raw html block
    pub fn raw_html_block(msg: impl Into<String>, range: MaybeRanged) -> ASTNode {
        TextSpan::HTMLRawInline(msg.into()).into_node(range)
    }
}
