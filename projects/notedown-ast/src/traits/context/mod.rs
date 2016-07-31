use crate::nodes::{ASTKind, ASTNode};
use crate::traits::ContextAware;
use crate::ASTNodes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextKind {
    None,
    Text,
    Function,
    Math,
    Code,
}

impl ContextAware for ASTNodes {
    fn context_aware(&self, offset: u32) -> ContextKind {
        for item in self {
            let e = item.value.context_aware(offset);
            if e != ContextKind::None {
                return e;
            }
        }
        return ContextKind::None;
    }
}

impl ContextAware for ASTNode {
    fn context_aware(&self, offset: u32) -> ContextKind {
        self.value.context_aware(offset)
    }
}

impl ContextAware for ASTKind {
    fn context_aware(&self, offset: u32) -> ContextKind {
        match self {
            ASTKind::Statements(children) => children.context_aware(offset),
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::Paragraph(children) => children.context_aware(offset),
            ASTKind::TableView { .. } => unimplemented!(),
            ASTKind::ListView { .. } => unimplemented!(),
            ASTKind::TextSpan(_) => unimplemented!(),
            ASTKind::MathNode(_) => ContextKind::Math,
            ASTKind::Command { .. } => unimplemented!(),
            ASTKind::Value { .. } => unimplemented!(),
            ASTKind::Delimiter(_) => unimplemented!(),
            ASTKind::CodeNode(_) => unimplemented!(),
            ASTKind::LinkNode(_) => unimplemented!(),
            ASTKind::StyledSpan(_) => unimplemented!(),
        }
    }
}
