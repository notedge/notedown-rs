use super::*;

impl Debug for NotedownTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotedownTerm::Heading(v) => Debug::fmt(v, f),
            NotedownTerm::Paragraph(v) => Debug::fmt(v, f),
            NotedownTerm::SpaceBreak(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for NotedownAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for NotedownTerm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
