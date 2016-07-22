use crate::{nodes::{ASTKind, ASTNode}, ListView};
use std::fmt::{self, Display, Formatter};

impl Display for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl Display for ASTKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            ASTKind::Statements(children) => {
                let s: Vec<_> = children.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{}", s.join("\n\n"))
            }
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::HorizontalRule { .. } => unimplemented!(),
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::Code(inner) => Display::fmt(inner, f),
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
            ASTKind::Value { .. } => unimplemented!(),
        }
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
