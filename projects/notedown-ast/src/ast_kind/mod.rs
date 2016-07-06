mod code_block;
mod command;
mod header;
mod link;
mod list;
mod literal;
mod table;

use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};


pub use command::CommandKind;
pub use link::SmartLink;

pub use crate::ast_kind::{code_block::CodeBlock, command::Command, header::Header, list::ListView, table::TableView};




#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode<M> {
    pub kind: ASTKind<ASTNode<M>>,
    pub meta: M,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind<T> {
    /// - `None`: It doesn't look like anything to me
    None,
    /// Top
    Statements(Vec<T>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header<T>>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(Vec<T>),
    CodeBlock(Box<CodeBlock>),
    /// - `Math`:
    MathBlock(Box<String>),
    TableView(Box<TableView<T>>),
    ListView(Box<ListView<T>>),
    /// - `Code`:
    // inlined
    Normal(Box<String>),
    Raw(Box<String>),
    /// `` `code` ``
    Code(Box<String>),

    Italic(Vec<T>),
    Bold(Vec<T>),
    Emphasis(Vec<T>),

    Underline(Vec<T>),
    Strikethrough(Vec<T>),
    Undercover(Vec<T>),

    MathInline(Box<String>),
    MathDisplay(Box<String>),

    Escaped(char),
    Link(Box<SmartLink>),
    //
    Command(Box<Command<T>>),
    String(Box<String>),
    Number(Box<String>),
    Boolean(bool),
    Array,
    Object,
}




impl<T> Default for ASTKind<T> {
    fn default() -> Self {
        Self::None
    }
}

impl<M: Default> Default for ASTNode<M> {
    fn default() -> Self {
        Self { kind: ASTKind::None, meta: Default::default() }
    }
}

impl<T> ASTKind<T> {
    pub fn statements(children: Vec<T>) -> Self {
        Self::Statements(children)
    }
    pub fn paragraph(children: Vec<T>) -> Self {
        Self::Paragraph(children)
    }
    pub fn header(children: Vec<T>, level: usize) -> Self {
        let header = Header { level, children };
        Self::Header(Box::new(header))
    }
    pub fn code(code: CodeBlock) -> Self {
        Self::CodeBlock(Box::new(code))
    }
    pub fn command(cmd: Command<T>) -> Self {
        Self::Command(Box::new(cmd))
    }

    pub fn hr() -> ASTKind<T> {
        Self::HorizontalRule
    }

    pub fn math(text: String, style: &str) -> Self {
        match style {
            "inline" => Self::MathInline(Box::new(text)),
            "display" => Self::MathDisplay(Box::new(text)),
            _ => Self::MathBlock(Box::new(text)),
        }
    }
    pub fn style(children: Vec<T>, style: &str) -> Self {
        match style {
            "*" | "i" | "italic" => Self::Italic(children),
            "**" | "b" | "bold" => Self::Bold(children),
            "***" | "em" => Self::Emphasis(children),
            "~" | "u" | "underline" => Self::Underline(children),
            "~~" | "s" => Self::Strikethrough(children),
            "~~~" => Self::Undercover(children),
            _ => unreachable!(),
        }
    }
    pub fn text(text: String, style: &str) -> Self {
        match style {
            "raw" => Self::Raw(Box::new(text)),
            _ => Self::Normal(Box::new(text)),
        }
    }
    pub fn escaped(char: char) -> Self {
        Self::Escaped(char)
    }
}
