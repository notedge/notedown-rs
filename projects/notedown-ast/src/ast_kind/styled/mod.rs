use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyledKind {
    Normal = 0,

    Italic,
    Bold,
    Emphasis,

    Underline,
    Strikethrough,
    Undercover,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyledNode<T> {
    kind: StyledKind,
    children: Vec<T>,
}

impl From<&str> for StyledKind {
    fn from(style: &str) -> Self {
        match style {
            "*" | "i" | "italic" => Self::Italic,
            "**" | "b" | "bold" => Self::Bold,
            "***" | "em" => Self::Emphasis,
            "~" | "u" | "underline" => Self::Underline,
            "~~" | "s" => Self::Strikethrough,
            "~~~" => Self::Undercover,
            _ => Self::Normal,
        }
    }
}

impl<T> StyledNode<T> {
    pub fn new(children: Vec<T>, style: &str) -> Self {
        Self { kind: StyledKind::from(style), children }
    }
}
