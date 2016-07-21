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
pub struct StyledNode {
    pub kind: StyledKind,
    pub children: ASTNodes,
}

impl Display for StyledNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.surround(f)
    }
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

impl StyledKind {
    pub fn surround(&self) -> &'static str {
        match self {
            Self::Normal => { "" }
            Self::Italic => { "*" }
            Self::Bold => { "**" }
            Self::Emphasis => { "***" }
            Self::Underline => { "~" }
            Self::Strikethrough => { "~~" }
            Self::Undercover => { "~~~" }
        }
    }
}

impl StyledNode {
    pub fn new(children: ASTNodes, style: &str) -> Self {
        Self { kind: StyledKind::from(style), children }
    }
    pub fn surround(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s  =self.kind.surround();
        f.write_str(s)?;
        for child in &self.children {
            write!(f, "{}", child.value)?;
        }
        f.write_str(s)?;
        Ok(())
    }
}
