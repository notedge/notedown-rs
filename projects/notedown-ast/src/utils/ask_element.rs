use crate::nodes::{ASTKind, ASTNode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentAware {
    None,
    Text,
    Function,
    Math,
    Code,
}

impl ASTNode {
    pub fn content_aware(&self, line: u32, column: u32) -> ContentAware {
        match &self.value {
            ASTKind::Statements(children) => unimplemented!(),
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::HorizontalRule { .. } => unimplemented!(),
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::CodeBlock { .. } => unimplemented!(),
            ASTKind::TableView { .. } => unimplemented!(),
            ASTKind::ListView { .. } => unimplemented!(),
            ASTKind::Normal { .. } => unimplemented!(),
            ASTKind::Raw { .. } => unimplemented!(),
            ASTKind::Code { .. } => ContentAware::Code,
            ASTKind::Styled(_) => unimplemented!(),
            ASTKind::Math(_) => ContentAware::Math,
            ASTKind::Link { .. } => unimplemented!(),
            ASTKind::Escaped { .. } => unimplemented!(),
            ASTKind::Command { .. } => unimplemented!(),
            ASTKind::Value { .. } => unimplemented!(),
        }
    }
}

fn content_aware_vec(v: &[ASTNode], line: u32, column: u32) -> ContentAware {
    for item in v {
        let e = item.content_aware(line, column);
        if e != ContentAware::None {
            return e;
        }
    }
    return ContentAware::None;
}
