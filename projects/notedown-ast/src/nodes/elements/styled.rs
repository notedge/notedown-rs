use super::*;

///
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StyleKind {
    /// Transparent type, do nothing
    Plain = 0,
    ///
    Emphasis = 11,
    ///
    Strong = 13,
    ///
    ItalicBold = 14,
    ///
    Underline = 21,
    ///
    Undercover = 22,
    ///
    Marking = 23,
    ///
    Color(u8, u8, u8, u8) = 24,
    ///
    Delete = 31,
    ///
    Insert = 32,
    ///
    Subscript = 41,
    ///
    Superscript = 42,
}
///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct StyleNode {
    ///
    pub kind: StyleKind,
    ///
    pub children: ASTNodes,
}

impl From<&str> for StyleKind {
    fn from(style: &str) -> Self {
        match style {
            "*" | "i" | "italic" | "em" => Self::Emphasis,
            "**" | "b" | "bold" => Self::Strong,
            "***" => Self::ItalicBold,
            "~" | "u" | "underline" => Self::Underline,
            "~~" | "s" => Self::Delete,
            "~~~" => Self::Undercover,
            _ => Self::Plain,
        }
    }
}

impl StyleKind {
    ///
    pub fn surround_in(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Emphasis => "*",
            Self::Strong => "**",
            Self::ItalicBold => "***",
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
    ///
    pub fn surround_out(&self) -> &'static str {
        match self {
            Self::Plain => "",
            Self::Emphasis => "*",
            Self::Strong => "**",
            Self::ItalicBold => "***",
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
    /// Constructor of [`StyleNode`]
    #[inline]
    pub fn new(children: ASTNodes, style: &str) -> Self {
        Self { kind: StyleKind::from(style), children }
    }
}

macro_rules! styled_node {
    (@StyleNode => $name:tt => $t:tt) => {
        /// Constructor of [`StyleNode`]
        #[inline]
        pub fn $name(children: ASTNodes) -> Self {
            Self { kind: StyleKind::$t, children }
        }
    };
    (@ASTKind => $name:tt => $t:tt) => {
        /// Constructor of [`StyleNode`]
        #[inline]
        pub fn $name(children: ASTNodes, range: MaybeRanged) -> ASTNode {
            StyleNode::$name(children).into_node(range)
        }
    };
    ($($name:tt => $t:tt),+ $(,)?) => (
        impl StyleNode { $(styled_node!(@StyleNode => $name=>$t);)+ }
        impl ASTKind {$(styled_node!(@ASTKind => $name=>$t);)+ }
    );
}

styled_node![
    bold        => Strong,
    strong      => Strong,
    italic      => Emphasis,
    emphasis    => Emphasis,
    italic_bold => ItalicBold,
    marking     => Marking,
    underline   => Underline,
    undercover  => Undercover,
    delete      => Delete,
    insert      => Insert,
    subscript   => Subscript,
    superscript => Superscript,
];
