use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    Normal = 0,

    Italic,
    Bold,
    Emphasis,

    Underline,
    Strikethrough,
    Undercover,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyleNode {
    pub kind: StyleKind,
    pub children: ASTNodes,
}

impl Display for StyleNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.surround(f)
    }
}

impl From<&str> for StyleKind {
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

impl StyleKind {
    pub fn surround(&self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Italic => "*",
            Self::Bold => "**",
            Self::Emphasis => "***",
            Self::Underline => "~",
            Self::Strikethrough => "~~",
            Self::Undercover => "~~~",
        }
    }
}

impl StyleNode {
    pub fn new(children: ASTNodes, style: &str) -> Self {
        Self { kind: StyleKind::from(style), children }
    }
    pub fn surround(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = self.kind.surround();
        f.write_str(s)?;
        for child in &self.children {
            write!(f, "{}", child.value)?;
        }
        f.write_str(s)?;
        Ok(())
    }
}
