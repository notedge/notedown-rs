use super::*;

impl Debug for TextModeTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TextModeTerm::Heading(v) => Debug::fmt(v, f),
            TextModeTerm::Paragraph(v) => Debug::fmt(v, f),
            TextModeTerm::SpaceBreak(v) => Debug::fmt(v, f),
        }
    }
}
