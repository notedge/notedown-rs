mod ast;
mod lsp;
mod traits;

pub use self::{ast::ASTValue, lsp::LSPValue};
use crate::{utils::LSPMetaInfo, ASTKind, CodeHighlight, Command};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode<M> {
    pub kind: ASTKind<ASTNode<M>>,
    pub meta: M,
}

impl<M: Default> Default for ASTNode<M> {
    fn default() -> Self {
        Self { kind: ASTKind::Null, meta: Default::default() }
    }
}
