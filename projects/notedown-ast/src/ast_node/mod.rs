mod ast;
mod lsp;
mod traits;

use crate::{utils::LSPMetaInfo, ASTKind, CodeHighlight, Command};
use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};
use std::mem::transmute;

pub type ASTNodes = Vec<ASTNode>;

#[derive(Clone, Eq)]
pub struct ASTNode {
    pub kind: ASTKind,
    // pub range: Option<(u32, u32)>,
    pub(crate) range: u64,
}

impl Default for ASTNode {
    fn default() -> Self {
        Self { kind: ASTKind::Null, range: 0 }
    }
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("ASTNode");
        w.field("kind", &self.kind);
        if self.range != 0 {
            let s = unsafe {
                transmute::<u64, (u32, u32)>(self.range)
            };
            w.field("range", &format!("{}-{}", s.0, s.1));
        }
        w.finish()
    }
}

impl Hash for ASTNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state)
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl ASTNode {
    pub fn range(&self) -> Option<(u32, u32)> {
        if self.range == 0 {
            return None;
        }
        unsafe {
            let s = transmute::<u64, (u32, u32)>(self.range);
            return Some(s);
        };
    }
}