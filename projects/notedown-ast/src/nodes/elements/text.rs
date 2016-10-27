use super::*;
use crate::{NoteError, Result};

///
#[repr(u8)]
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum TextSpan {
    ///
    Empty,
    ///
    Normal(String),
    ///
    Raw(String),
    ///
    HTMLRawInline(String),
    /// The Unicode codepoint sequence of this emoji.
    /// The actual/rendered emoji.
    Emoji(&'static str),
    ///
    Escaped(char),
    ///
    SoftNewline,
    ///
    HardNewline,
    ///
    CheckBox(bool),
}

impl Default for TextSpan {
    fn default() -> Self {
        Self::Empty
    }
}

impl ASTKind {
    /// Aka. `<br>`
    #[inline]
    pub fn hard_break(range: MaybeRanged) -> ASTNode {
        TextSpan::HardNewline.into_node(range)
    }
    /// Aka. `\n`
    #[inline]
    pub fn soft_break(range: MaybeRanged) -> ASTNode {
        TextSpan::SoftNewline.into_node(range)
    }
    ///
    #[inline]
    pub fn text(s: impl Into<String>, range: MaybeRanged) -> ASTNode {
        TextSpan::Normal(s.into()).into_node(range)
    }
    ///
    #[inline]
    pub fn text_raw(s: impl Into<String>, range: MaybeRanged) -> ASTNode {
        TextSpan::Raw(s.into()).into_node(range)
    }
    /// Constructor of [`TextSpan::Escaped`]
    #[inline]
    pub fn escaped(text: &str, range: MaybeRanged) -> Result<ASTNode> {
        let c = match text.chars().next() {
            None => '\\',
            Some(s) => s,
        };
        Ok(TextSpan::Escaped(c).into_node(range))
    }
    /// Constructor of [`TextSpan::Escaped`]
    #[inline]
    pub fn escaped_char(c: char, range: MaybeRanged) -> ASTNode {
        TextSpan::Escaped(c).into_node(range)
    }
    /// Constructor of [`TextSpan::Normal`]
    #[inline]
    pub fn escaped_html(_: char, _: MaybeRanged) -> Result<ASTNode> {
        todo!()
    }
    /// Constructor of [`TextSpan::Emoji`]
    #[inline]
    pub fn emoji(text: &str, range: MaybeRanged) -> Result<ASTNode> {
        match text_utils::parse_emoji(text) {
            None => {
                let msg = format!("{} can not be parsed as emoji", text);
                let mut error = NoteError::syntax_error(msg);
                error.range = range;
                Err(error)
            }
            Some(s) => Ok(TextSpan::Emoji(s.grapheme).into_node(range)),
        }
    }
}
