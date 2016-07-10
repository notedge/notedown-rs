mod traits;
mod ast;
mod lsp;

use crate::{utils::LSPMetaInfo, ASTKind, CodeHighlight, Command};
pub use self::ast::ASTValue;
pub use self::lsp::LSPValue;

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


