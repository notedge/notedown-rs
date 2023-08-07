use super::*;

impl Debug for ParagraphTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParagraphTerm::Text(v) => Debug::fmt(v, f),
            ParagraphTerm::WhiteSpace(v) => Debug::fmt(v, f),
            ParagraphTerm::NewLine(v) => Debug::fmt(v, f),
            ParagraphTerm::Comma(v) => Debug::fmt(v, f),
            ParagraphTerm::Period(v) => Debug::fmt(v, f),
            ParagraphTerm::Escape(v) => Debug::fmt(v, f),
            ParagraphTerm::Italic(v) => Debug::fmt(v, f),
            ParagraphTerm::Bold(v) => Debug::fmt(v, f),
            ParagraphTerm::BoldItalic(v) => Debug::fmt(v, f),
        }
    }
}
