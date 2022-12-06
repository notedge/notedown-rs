use super::*;

/// Delimiter of two block
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Delimiter {
    /// Aka. `<hr>`
    HorizontalRule,
    /// Raw HTML block that can be insert directly
    HTMLRawBlock(String),
}

impl NotedownKind {
    /// Insert raw html text
    pub fn raw_html_inline(msg: impl Into<String>, span: &Span, file: &FileID) -> NotedownNode {
        Delimiter::HTMLRawBlock(msg.into()).into_node(span, file)
    }
    /// Inset raw html block
    pub fn raw_html_block(msg: impl Into<String>, span: &Span, file: &FileID) -> NotedownNode {
        TextSpan::HTMLRawInline(msg.into()).into_node(span, file)
    }
}
