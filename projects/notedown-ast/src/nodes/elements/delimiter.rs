use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Delimiter {
    HorizontalRule,
}

impl Delimiter {
    #[inline]
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: ASTKind::Delimiter(Box::new(self)), range }
    }
}
