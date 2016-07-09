mod traits;

use crate::{utils::LSPMetaInfo, ASTKind, CodeBlock, Command};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode<M> {
    pub kind: ASTKind<ASTNode<M>>,
    pub meta: M,
}

impl<M: Default> Default for ASTNode<M> {
    fn default() -> Self {
        Self { kind: ASTKind::None, meta: Default::default() }
    }
}

impl<M: Default> ASTNode<M> {
    pub fn statements(children: Vec<ASTNode<M>>) -> Self {
        ASTNode { kind: ASTKind::statements(children), meta: M::default() }
    }
    pub fn paragraph(children: Vec<ASTNode<M>>) -> Self {
        ASTNode { kind: ASTKind::paragraph(children), meta: M::default() }
    }
    pub fn header(children: Vec<ASTNode<M>>, level: usize) -> Self {
        ASTNode { kind: ASTKind::header(children, level), meta: M::default() }
    }
    pub fn code(code: CodeBlock) -> Self {
        ASTNode { kind: ASTKind::code(code), meta: M::default() }
    }
    pub fn command(cmd: Command<ASTNode<M>>) -> Self {
        ASTNode { kind: ASTKind::command(cmd), meta: M::default() }
    }

    pub fn hr() -> Self {
        ASTNode { kind: ASTKind::hr(), meta: M::default() }
    }

    pub fn math(text: String, style: &str) -> Self {
        ASTNode { kind: ASTKind::math(text, style), meta: M::default() }
    }
    pub fn style(children: Vec<ASTNode<M>>, style: &str) -> Self {
        ASTNode { kind: ASTKind::style(children, style), meta: M::default() }
    }
    pub fn text(text: String, style: &str) -> Self {
        ASTNode { kind: ASTKind::text(text, style), meta: M::default() }
    }
    pub fn escaped(char: char) -> Self {
        ASTNode { kind: ASTKind::escaped(char), meta: M::default() }
    }
}

impl ASTNode<LSPMetaInfo> {
    pub fn set_range(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.meta.set_range(x1, y1, x2, y2)
    }
}
