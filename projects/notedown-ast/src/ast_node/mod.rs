mod ast;
mod lsp;
mod traits;

use crate::{utils::LSPMetaInfo, ASTKind, CodeHighlight, Command};
use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

#[derive(Clone, Eq)]
pub struct ASTNode<T> {
    pub kind: ASTKind<T>,
    pub range: Option<(u32, u32)>,
}

impl<T: Default> Default for ASTNode<T> {
    fn default() -> Self {
        Self { kind: ASTKind::Null, range: None }
    }
}

impl<T: Debug> Debug for ASTNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("ASTNode");
        w.field("kind", &self.kind);
        if let Some(s) = self.range {
            w.field("range", &format!("{}-{}", s.0, s.1));
        }
        w.finish()
    }
}

impl<T: Hash> Hash for ASTNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state)
    }
}

impl<T: PartialEq> PartialEq for ASTNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}
