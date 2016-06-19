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
pub struct AST {
    kind: ASTKind,
    r: Option<Box<TextRange>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    /// - `None`: It doesn't look like anything to me
    None,
    Statements(Vec<AST>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(Vec<AST>),
    CodeBlock(Box<CodeBlock>),
    /// - `Math`:
    MathBlock(String),
    TableView(Box<TableView>),
    ListView(Box<ListView>),
    /// - `Code`:
    // inlined
    Normal(String),
    Raw(String),
    /// `` `code` ``
    Code(String),
    Italic(Vec<AST>),
    Bold(Vec<AST>),
    Emphasis(Vec<AST>),
    Underline(Vec<AST>),
    Strikethrough (Vec<AST>),
    Undercover(Vec<AST>),
    MathInline(String),
    MathDisplay (String),
    Link (Box<SmartLink>),
    Escaped(char),
    //
    Command (Box<Command>),
    String (String),
    Number (String),
    Boolean (bool),
    Array (Vec<AST>),
    Object
}

impl Default for AST {
    fn default() -> Self {
        Self {
            kind: ASTKind::None,
            r: None
        }
    }
}

impl AST {
    pub fn to_vec(&self) -> Vec<AST> {
        use ASTKind::*;
        match &self.kind {
            Statements(v)| Paragraph(v) => {v.to_owned()},
            _ => vec![]
        }
    }
}
