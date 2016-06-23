use crate::{ast::ASTKind, AST};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentAware {
    None,
    Text,
    Function,
    Math,
    Code,
}

impl AST {
    pub fn content_aware(&self, line: u32, column: u32) -> ContentAware {
        let children = self.children();
        match self.kind() {
            ASTKind::None => ContentAware::None,
            ASTKind::Statements => content_aware_vec(&children, line, column),
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::HorizontalRule { .. } => unimplemented!(),
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::CodeBlock { .. } => unimplemented!(),
            ASTKind::MathBlock { .. } => ContentAware::Math,
            ASTKind::TableView { .. } => unimplemented!(),
            ASTKind::ListView { .. } => unimplemented!(),
            ASTKind::Normal { .. } => unimplemented!(),
            ASTKind::Raw { .. } => unimplemented!(),
            ASTKind::Code { .. } => ContentAware::Code,
            ASTKind::Italic { .. } => unimplemented!(),
            ASTKind::Bold { .. } => unimplemented!(),
            ASTKind::Emphasis { .. } => unimplemented!(),
            ASTKind::Underline { .. } => unimplemented!(),
            ASTKind::Strikethrough { .. } => unimplemented!(),
            ASTKind::Undercover { .. } => unimplemented!(),
            ASTKind::MathInline { .. } => ContentAware::Math,
            ASTKind::MathDisplay { .. } => ContentAware::Math,
            ASTKind::Link { .. } => unimplemented!(),
            ASTKind::Escaped { .. } => unimplemented!(),
            ASTKind::Command { .. } => unimplemented!(),
            ASTKind::String { .. } => unimplemented!(),
            ASTKind::Number { .. } => unimplemented!(),
            ASTKind::Boolean { .. } => unimplemented!(),
            ASTKind::Array { .. } => unimplemented!(),
            ASTKind::Object => unimplemented!(),
        }
    }
}

fn content_aware_vec(v: &[AST], line: u32, column: u32) -> ContentAware {
    for item in v {
        let e = item.content_aware(line, column);
        if e != ContentAware::None {
            return e;
        }
    }
    return ContentAware::None;
}
