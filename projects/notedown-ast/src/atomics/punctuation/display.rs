use super::*;
use std::fmt::Debug;

impl Debug for IgnoreNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WS(s) => Debug::fmt(s, f),
            Self::NL(s) => Debug::fmt(s, f),
        }
    }
}

impl NotedownNode for WhitespaceNode {
    fn get_span(&self) -> Range<u32> {
        self.span.clone()
    }
}

impl From<WhitespaceNode> for IgnoreNode {
    fn from(value: WhitespaceNode) -> Self {
        Self::WS(value)
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

impl From<NewlineNode> for IgnoreNode {
    fn from(value: NewlineNode) -> Self {
        Self::NL(value)
    }
}
