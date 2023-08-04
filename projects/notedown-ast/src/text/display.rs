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

impl NotedownNode for NotedownAST {
    fn write_html(&self, f: &mut impl Write) -> std::fmt::Result {
        todo!()
    }

    fn write_tex(&self, f: &mut impl Write) -> std::fmt::Result {
        todo!()
    }

    fn get_span(&self) -> Range<u32> {
        todo!()
    }
}

impl NotedownNode for NotedownTerm {
    fn write_html(&self, f: &mut impl Write) -> std::fmt::Result {
        match self {
            NotedownTerm::Heading(v) => v,
            NotedownTerm::Paragraph(v) => {}
            NotedownTerm::SpaceBreak(v) => {}
        }
    }

    fn write_tex(&self, f: &mut impl Write) -> std::fmt::Result {
        todo!()
    }

    fn get_span(&self) -> Range<u32> {
        todo!()
    }
}
