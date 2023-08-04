use super::*;

impl Display for HeadingNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl NotedownNode for HeadingNode {
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
