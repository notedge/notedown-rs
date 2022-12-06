use super::*;

///
#[repr(u8)]
#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TextSpan {
    ///
    Empty = 0,
    ///
    Normal(String),
    ///
    Raw(String),
    ///
    HTMLRawInline(String),
    /// The Unicode codepoint sequence of this emoji.
    /// The actual/rendered emoji.
    Emoji(String),
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
