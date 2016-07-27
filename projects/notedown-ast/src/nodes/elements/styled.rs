use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextNode {
    Normal(String),
    Raw(String),
    Escaped(char),

    Italic(ASTNodes),
    Bold(ASTNodes),
    Emphasis(ASTNodes),

    Underline(ASTNodes),
    Strikethrough(ASTNodes),
    Undercover(ASTNodes),
}

impl Display for TextNode {
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
            Self::Normal | Self::Raw | Self::Escaped => "",
            Self::Italic => "*",
            Self::Bold => "**",
            Self::Emphasis => "***",
            Self::Underline => "~",
            Self::Strikethrough => "~~",
            Self::Undercover => "~~~",
        }
    }
}

impl TextNode {
    pub fn text(inner: String) -> Self {
        Self::Normal(inner)
    }


    pub fn styled(children: ASTNodes, style: &str) -> Self {
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
