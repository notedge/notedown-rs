use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListItem {
    pub prefix: Literal<ListPrefixSymbol>,
    pub rest: ASTNodes,
}

impl From<ASTNodes> for ListItem {
    fn from(node: ASTNodes) -> Self {
        Self { prefix: Default::default(), rest: node }
    }
}

impl ListItem {}
