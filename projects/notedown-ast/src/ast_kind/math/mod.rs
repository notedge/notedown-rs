use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MathKind {
    Block,
    Inline,
    Display,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MathNode {
    kind: MathKind,
    raw: String,
}

impl Display for MathNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}



impl MathNode {
    pub fn block(raw: String) -> Self {
        Self {
            kind: MathKind::Block,
            raw,
        }
    }
    pub fn inline(raw: String) -> Self {
        Self {
            kind: MathKind::Inline,
            raw,
        }
    }
    pub fn display(raw: String) -> Self {
        Self {
            kind: MathKind::Display,
            raw,
        }
    }
}


