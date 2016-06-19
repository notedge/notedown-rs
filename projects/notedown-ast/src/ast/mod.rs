mod command;
mod link;
mod literal;
mod range;
mod code_block;
mod list;
mod table;
mod header;

pub use command::CommandKind;
pub use link::SmartLink;
pub use range::TextRange;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
pub use crate::ast::code_block::CodeBlock;
pub use crate::ast::list::ListView;
pub use crate::ast::table::TableView;
pub use crate::ast::command::Command;
pub use crate::ast::header::Header;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AST {
    Node {
        kind: ASTKind,
        children: Vec<AST>,
        r: Option<Box<TextRange>>,
    },
    Leaf {
        kind: ASTKind,
        r: Option<Box<TextRange>>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    /// - `None`: It doesn't look like anything to me
    None,
    Statements(Vec<AST>),
    // Blocks
    /// - `Header`: TEXT, level
    Header {
        inner: Box<Header>,
    },
    HorizontalRule {
    },
    ///  - `Paragraph`:
    Paragraph {
        children: Vec<AST>,
    },
    CodeBlock {
        inner: Box<CodeBlock>,
    },
    /// - `Math`:
    MathBlock {
        inner: String,
    },
    TableView {
        inner: Box<TableView>,
    },
    ListView {
        inner: Box<ListView>,
    },
    /// - `Code`:
    // inlined
    Normal {
        inner: String,
    },
    Raw {
        inner: String,
    },
    /// `` `code` ``
    Code {
        inner: String,
    },
    Italic {
        children: Vec<AST>,
    },
    Bold {
        children: Vec<AST>,
    },
    Emphasis {
        children: Vec<AST>,
    },
    Underline {
        children: Vec<AST>,
    },
    Strikethrough {
        children: Vec<AST>,
    },
    Undercover {
        children: Vec<AST>,
    },
    MathInline {
        inner: String,
    },
    MathDisplay {
        inner: String,
    },
    Link {
        inner: Box<SmartLink>,
    },
    Escaped {
        inner: char,
    },
    //
    Command {
        inner: Box<Command>,
    },
    String {
        inner: String,
    },
    Integer {
        inner: String,
    },
    Decimal {
        inner: String,
    },
    Boolean {
        inner: bool,
    },
    Array {
        inner: Vec<AST>,
    },
}

impl Default for AST {
    fn default() -> Self {
        Self::Leaf { kind: ASTKind::None, r: None }
    }
}

impl AST {
    pub fn to_vec(&self) -> Vec<AST> {
        match self {
            AST::Node { children, .. } => {
                children.to_owned()
            }
            AST::Leaf { .. } => vec![],
        }
    }
}
