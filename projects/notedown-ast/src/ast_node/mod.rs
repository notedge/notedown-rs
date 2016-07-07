mod traits;

use crate::ASTKind;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode<M> {
    pub kind: ASTKind<ASTNode<M>>,
    pub meta: M,
}

impl<M: Default> Default for ASTNode<M> {
    fn default() -> Self {
        Self { kind: ASTKind::None, meta: Default::default() }
    }
}