use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    Plain = 0,

    Italic = 11,
    Bold = 12,
    Emphasis = 13,

    Underline = 21,
    Strikethrough = 22,
    Undercover = 23,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyleNode {
    pub kind: StyleKind,
    pub children: ASTNodes,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TextNode {
    Normal(String),
    Raw(String),
    Escaped(char),
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
            _ => Self::Plain,
        }
    }
}

impl StyleKind {
    pub fn surround(&self) -> &'static str {
        match self {
            Self::Plain => "",
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
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: ASTKind::StyledSpan(Box::new(self)), range }
    }
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

impl TextNode {
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: ASTKind::TextSpan(Box::new(self)), range }
    }
    pub fn new(children: String) -> Self {
        Self::Normal(children)
    }
    pub fn raw(children: String) -> Self {
        Self::Raw(children)
    }
    pub fn escaped(string: String) -> Option<Self> {
        let mut s = string.chars().peekable();
        match s.next() {
            Some('\\') => {}
            _ => return None,
        }
        match s.next() {
            Some(c) => Some(Self::Escaped(c)),
            None => None,
        }
    }
    pub fn escaped_char(char: char) -> Self {
        Self::Escaped(char)
    }
}
