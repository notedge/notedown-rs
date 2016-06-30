mod code_block;
mod command;
mod header;
mod link;
mod list;
mod literal;
mod range;
mod table;

pub use crate::ast::{code_block::CodeBlock, command::Command, header::Header, list::ListView, table::TableView};
pub use command::CommandKind;
pub use link::SmartLink;
pub use range::TextRange;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ASTNode {
    pub kind: ASTKind<ASTNode>,
    pub range: TextRange,
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

impl Default for ASTNode {
    fn default() -> Self {
        Self { kind: ASTKind::None, range: Default::default() }
    }
}

impl<T> ASTKind<T> {
    pub fn statements(children: Vec<T>) -> Self {
        ASTKind::Statements(children)
    }
    pub fn paragraph(children: Vec<T>) -> Self {
        ASTKind::Paragraph(children)
    }
    pub fn header(children: Vec<T>, level: usize) -> Self {
        let header = Header { level, children };
        ASTKind::Header(Box::new(header))
    }
    pub fn code(code: CodeBlock) -> Self {
        ASTKind::CodeBlock(Box::new(code))
    }
    pub fn command(cmd: Command<T>) -> Self {
        ASTKind::Command(Box::new(cmd))
    }

    pub fn hr() -> ASTKind<T> {
        ASTKind::HorizontalRule
    }

    pub fn math(text: String, style: &str) -> Self {
        match style {
            "inline" => ASTKind::MathInline(Box::new(text)),
            "display" => ASTKind::MathDisplay(Box::new(text)),
            _ => ASTKind::MathBlock(Box::new(text)),
        }
    }
    pub fn style(children: Vec<T>, style: &str) -> Self {
        match style {
            "*" | "i" | "italic" => ASTKind::Italic(children),
            "**" | "b" | "bold" => ASTKind::Bold(children),
            "***" | "em" => ASTKind::Emphasis(children),
            "~" | "u" | "underline" => ASTKind::Underline(children),
            "~~" | "s" => ASTKind::Strikethrough(children),
            "~~~" => ASTKind::Undercover(children),
            _ => unreachable!(),
        }
    }
    pub fn text(text: String, style: &str) -> Self {
        match style {
            "raw" => ASTKind::Raw(Box::new(text)),
            _ => ASTKind::Normal(Box::new(text)),
        }
    }
    pub fn escaped(char: char) -> Self {
        ASTKind::Escaped(char)
    }
}
