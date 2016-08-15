mod link;
mod value;

use crate::nodes::{ASTKind, Delimiter, ListView, Literal, SmartLink, TextNode, Value, ValueType, StyleNode};
use itertools::Itertools;
use std::fmt::{self, Display, Formatter, Write};
use std::fmt::Debug;

impl<T: Display> Display for Literal<T> {
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
        match self {
            Self::HorizontalRule => f.write_str("---"),
        }
    }
}

impl Display for TextNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal(_) => {
                unimplemented!()
            }
            Self::Raw(_) => {
                unimplemented!()
            }
            Self::Escaped(c) => {
                f.write_char('\\')?;
                f.write_char(*c)
            }
            Self::Emoji(c) => f.write_char(*c),
            TextNode::SoftNewline => f.write_char('\n'),
            TextNode::HardNewline => f.write_char('\n'),
        }
    }
}

impl Display for StyleNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.surround_in())?;
        for child in &self.children {
            write!(f, "{}", child.value)?;
        }
        f.write_str(self.kind.surround_out())?;
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => f.write_str("null"),
            Self::Boolean(v) => f.write_str(&v.to_string()),
            Self::Integer(v) => f.write_str(&v.to_string()),
            Self::Decimal(v) => f.write_str(&v.to_string()),
            Self::String(_) => {
                unimplemented!()
            }
            Self::Set(_) => {
                unimplemented!()
            }
            Self::Array(_) => {
                unimplemented!()
            }
            Self::Object(_) => {
                unimplemented!()
            }
        }
    }
}
