use crate::{ASTKind, ASTNode, CodeHighlight, Command};

pub type ASTValue = ASTNode<()>;

impl ASTValue {
    pub fn statements(children: Vec<ASTValue>) -> Self {
        ASTNode { kind: ASTKind::statements(children), meta: () }
    }
    pub fn paragraph(children: Vec<ASTValue>) -> Self {
        ASTNode { kind: ASTKind::paragraph(children), meta: () }
    }
    pub fn header(children: Vec<ASTValue>, level: usize) -> Self {
        ASTNode { kind: ASTKind::header(children, level), meta: () }
    }
    pub fn code(code: CodeHighlight) -> Self {
        ASTNode { kind: ASTKind::code(code), meta: () }
    }
    pub fn command(cmd: Command<ASTValue>) -> Self {
        ASTNode { kind: ASTKind::command(cmd), meta: () }
    }

    pub fn hr() -> Self {
        ASTNode { kind: ASTKind::hr(), meta: () }
    }

    pub fn math(text: String, style: &str) -> Self {
        ASTNode { kind: ASTKind::math(text, style), meta: () }
    }
    pub fn style(children: Vec<ASTValue>, style: &str) -> Self {
        ASTNode { kind: ASTKind::style(children, style), meta: () }
    }
    pub fn text(text: String, style: &str) -> Self {
        ASTNode { kind: ASTKind::text(text, style), meta: () }
    }
    pub fn escaped(char: char) -> Self {
        ASTNode { kind: ASTKind::escaped(char), meta: () }
    }
}
