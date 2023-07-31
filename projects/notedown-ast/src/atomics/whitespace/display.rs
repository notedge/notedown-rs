use super::*;

impl NotedownNode for WhitespaceNode {
    fn get_span(&self) -> Range<u32> {
        self.span.clone()
    }
}

impl Display for WhitespaceNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.width {
            f.write_char(' ')?
        }
        Ok(())
    }
}

impl Display for NewlineNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.count {
            f.write_char('\n')?
        }
        Ok(())
    }
}
