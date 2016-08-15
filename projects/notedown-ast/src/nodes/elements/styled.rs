use super::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    Plain = 0,

    Italic = 11,
    Strong = 12,
    Emphasis = 13,

    Underline = 21,
    Undercover = 22,
    Highlight = 23,
    Color(u8, u8, u8, u8) = 24,
    // HTMLColor(String) = 25,

    Delete = 31,
    Insert = 32,

    Sub = 41,
    Super = 42,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyleNode {
    pub kind: StyleKind,
    pub children: ASTNodes,
}

impl From<&str> for StyleKind {
    fn from(style: &str) -> Self {
        match style {
            "*" | "i" | "italic" => Self::Italic,
            "**" | "b" | "bold" => Self::Strong,
            "***" | "em" => Self::Emphasis,
            "~" | "u" | "underline" => Self::Underline,
            "~~" | "s" => Self::Delete,
            "~~~" => Self::Undercover,
            _ => Self::Plain,
        }
    }
}

impl StyleKind {
    pub fn surround_in(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Italic => "*",
            Self::Strong => "**",
            Self::Emphasis => "***",
            Self::Underline => "~",
            Self::Delete => "~~",
            Self::Undercover => "~~~",
            Self::Highlight => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            StyleKind::Color(_, _, _, _) => {unimplemented!()}
            StyleKind::Sub => {unimplemented!()}
            StyleKind::Super => {unimplemented!()}
        }
    }
    pub fn surround_out(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Italic => "*",
            Self::Strong => "**",
            Self::Emphasis => "***",
            Self::Underline => "~",
            Self::Delete => "~~",
            Self::Undercover => "~~~",
            Self::Highlight => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {unimplemented!()}
            Self::Sub => {unimplemented!()}
            Self::Super => {unimplemented!()}
        }
    }
}

impl StyleNode {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::StyledSpan(box self), range }
    }
    #[inline]
    pub fn new(children: ASTNodes, style: &str) -> Self {
        Self { kind: StyleKind::from(style), children }
    }
}

