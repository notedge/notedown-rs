mod value;

use crate::{nodes::{ASTKind, ASTNode, ValueType}, ListView, TextNode};
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};
use crate::nodes::{Delimiter, Value};

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
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::Delimiter(inner) => Display::fmt(inner, f),
            ASTKind::TableView(inner) => Display::fmt(inner, f),
            ASTKind::ListView(inner) => Display::fmt(inner, f),
            ASTKind::TextSpan(inner) => Display::fmt(inner, f),
            ASTKind::StyledSpan(inner) => Display::fmt(inner, f),
            ASTKind::MathNode(inner) => Display::fmt(inner, f),
            ASTKind::CodeNode(inner) => Display::fmt(inner, f),
            ASTKind::LinkNode(inner) => Display::fmt(inner, f),
            ASTKind::Command(inner) => Display::fmt(inner, f),
            ASTKind::Value(inner) => Display::fmt(inner, f),
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

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for TextNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}