use super::*;

#[repr(u8)]
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TextNode {
    Empty = 0,

    Raw(String) = 1,
    Normal(String) = 2,

    Emoji(char) = 11,
    Escaped(char) = 12,

    SoftNewline = 21,
    HardNewline = 22,

    CheckBox(bool) = 31,
}

impl TextNode {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::TextSpan(box self), range }
    }
    #[inline]
    pub fn new(children: String) -> Self {
        Self::Normal(children)
    }
    #[inline]
    pub fn raw(children: String) -> Self {
        Self::Raw(children)
    }
    pub fn escaped(string: String) -> Option<Self> {
        let mut s = string.chars().peekable();
        match s.next() {
            Some('\\') => {}
            _ => return None,
        }
        s.next().map(Self::Escaped)
    }

    pub fn emoji(_: String) -> Self {
        unimplemented!()
    }
}

impl ASTKind {
    /// aka `<br>`
    #[inline]
    pub fn hard_break(range: Option<OffsetRange>) -> ASTNode {
        TextNode::HardNewline.into_node(range)
    }
    #[inline]
    pub fn soft_break(range: Option<OffsetRange>) -> ASTNode {
        TextNode::SoftNewline.into_node(range)
    }
    #[inline]
    pub fn text(s: impl Into<String>, range: Option<OffsetRange>) -> ASTNode {
        TextNode::Normal(s.into()).into_node(range)
    }
    #[inline]
    pub fn emoji(text: &str, range: Option<OffsetRange>) -> ASTNode {
        let c = match text.chars().next() {
            None => return TextNode::Empty.into_node(range),
            Some(s) => s,
        };
        TextNode::Escaped(c).into_node(range)
    }
    #[inline]
    pub fn escaped(text: &str, range: Option<OffsetRange>) -> ASTNode {
        let c = match text.chars().next() {
            None => '\\',
            Some(s) => s,
        };
        TextNode::Escaped(c).into_node(range)
    }
}
