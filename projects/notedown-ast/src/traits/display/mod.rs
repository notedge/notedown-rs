use crate::{
    ast_kind::{ASTKind, MathNode, StyledNode},
    ASTNode, ListView,
};
use std::fmt::{self, Debug, Display, Formatter};

impl Display for ASTNode {
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

impl Display for StyledNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Display for MathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Display for ListView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::QuoteList { style, body } => {
                writeln!(f, "QuoteList")?;
                writeln!(f, "{:?}", style)?;
                writeln!(f, "{:?}", body)?;
            }
            Self::OrderedList { .. } => {
                writeln!(f, "OrderedList")?;
            }
            Self::OrderlessList { .. } => {
                writeln!(f, "OrderlessList")?;
            }
            Self::Details { .. } => {
                writeln!(f, "Details")?;
            }
        }
        Ok(())
    }
}
