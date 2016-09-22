use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Delimiter {
    HorizontalRule,
    HTMLRawBlock(String),
}

impl Delimiter {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::Delimiter(box self), range }
    }
}

impl ASTKind {
    pub fn raw_html_inline(msg: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        Delimiter::HTMLRawBlock(msg.into()).into_node(range)
    }
    pub fn raw_html_block(msg: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        TextKind::HTMLRawInline(msg.into()).into_node(range)
    }
}
