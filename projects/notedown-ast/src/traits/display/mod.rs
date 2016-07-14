use crate::{
    ast_kind::{ASTKind, MathNode, StyledNode},
    utils::join_ast_list,
    ASTNode,
};
use std::fmt::{self, Debug, Display, Formatter};

impl<M: Debug + Display> Display for ASTNode<M> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ASTKind::Null => write!(f, ""),
            ASTKind::Statements(children) => {
                let s: Vec<_> = children.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{}", s.join("\n\n"))
            }
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::HorizontalRule { .. } => unimplemented!(),
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::CodeBlock(inner) => Display::fmt(inner, f),
            ASTKind::TableView { .. } => unimplemented!(),
            ASTKind::ListView(inner) => Display::fmt(inner, f),
            ASTKind::Normal(inner) => write!(f, "{}", inner),
            ASTKind::Styled(inner) => Display::fmt(inner, f),
            ASTKind::Math(inner) => Display::fmt(inner, f),
            ASTKind::Raw { .. } => unimplemented!(),
            ASTKind::Code { .. } => unimplemented!(),
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

impl<T> Display for StyledNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Display for MathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
