mod link;
mod list;
mod value;

use crate::nodes::{ASTKind, Delimiter, Literal, SmartLink, StyleNode, TextNode, Value, ValueType};
use itertools::Itertools;
use std::fmt::{self, Debug, Display, Formatter, Write};

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

impl Display for Delimiter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::HorizontalRule => f.write_str("---"),
        }
    }
}

impl Debug for TextNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal(s) => Debug::fmt(s, f),
            Self::Raw(s) => Debug::fmt(s, f),
            Self::Escaped(c) => {
                write!(f, "TextNode::Escaped({})", c)
            }
            Self::Emoji(c) => f.write_char(*c),
            Self::SoftNewline => f.write_str("TextNode::SoftNewline"),
            Self::HardNewline => f.write_str("TextNode::HardNewline"),
            Self::CheckBox(b) => {
                let w = &mut f.debug_struct("TextNode::CheckBox");
                w.field("complete", b);
                w.finish()
            }
            Self::Empty => f.write_str("TextNode::Empty"),
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
            Self::SoftNewline => f.write_char('\n'),
            Self::HardNewline => f.write_char('\n'),
            Self::CheckBox(b) => match b {
                true => f.write_str("[x]"),
                false => f.write_str("[ ]"),
            },
            Self::Empty => Ok(()),
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
