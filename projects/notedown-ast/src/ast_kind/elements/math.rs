use super::*;
use std::mem::transmute;

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
    format: &'static str,
}

impl Default for MathNode {
    fn default() -> Self {
        Self { kind: MathKind::Block, raw: String::new(), format: "LaTeX" }
    }
}

impl MathNode {
    pub fn into_node(self, range: (u32, u32)) -> ASTNode {
        let range = unsafe {
            transmute::<(u32,u32), u64>(range)
        };
        ASTNode { kind: ASTKind::Math(Box::new(self)), range }
    }

    pub fn block(raw: String) -> Self {
        Self { kind: MathKind::Block, raw, ..Self::default() }
    }
    pub fn inline(raw: String) -> Self {
        Self { kind: MathKind::Inline, raw, ..Self::default() }
    }
    pub fn display(raw: String) -> Self {
        Self { kind: MathKind::Display, raw, ..Self::default() }
    }
}
