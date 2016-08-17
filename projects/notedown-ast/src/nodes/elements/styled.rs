use super::*;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    Plain = 0,

    Italic = 11,
    Emphasis = 12,
    Strong = 13,
    ItalicBold = 14,

    Underline = 21,
    Undercover = 22,
    Marking = 23,
    Color(u8, u8, u8, u8) = 24,
    // HTMLColor(String) = 25,
    Delete = 31,
    Insert = 32,

    Subscript = 41,
    Superscript = 42,
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
            Self::Marking => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {
                unimplemented!()
            }
            Self::Subscript => "<sub>",
            Self::Superscript => "<sup>",
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
            Self::Marking => {
                unimplemented!()
            }
            Self::Insert => {
                unimplemented!()
            }
            Self::Color(_, _, _, _) => {
                unimplemented!()
            }
            Self::Subscript => "</sub>",
            Self::Superscript => "</sup>",
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

macro_rules! styled_node {
    ($name:tt => $t:tt) => {
        #[inline]
        pub fn $name(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
            StyleNode { kind: StyleKind::$t, children }.into_node(range)
        }
    };
}

impl ASTKind {
    styled_node![strong2 => Strong];

    #[inline]
    pub fn strong(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Strong, children }.into_node(range)
    }
    #[inline]
    pub fn italic(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Italic, children }.into_node(range)
    }
    #[inline]
    pub fn italic_bold(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::ItalicBold, children }.into_node(range)
    }
    #[inline]
    pub fn emphasis(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Emphasis, children }.into_node(range)
    }
    #[inline]
    pub fn marking(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Marking, children }.into_node(range)
    }
    #[inline]
    pub fn underline(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Underline, children }.into_node(range)
    }
    #[inline]
    pub fn undercover(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Undercover, children }.into_node(range)
    }
    #[inline]
    pub fn delete(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Delete, children }.into_node(range)
    }
    #[inline]
    pub fn insert(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Insert, children }.into_node(range)
    }
    #[inline]
    pub fn subscript(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Subscript, children }.into_node(range)
    }
    #[inline]
    pub fn superscript(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        StyleNode { kind: StyleKind::Superscript, children }.into_node(range)
    }
}
