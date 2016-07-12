mod elements;
mod traits;

pub use self::elements::*;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Eq, PartialEq,Hash)]
pub enum ASTKind<T> {
    /// Top Scope
    Statements(Vec<T>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header<T>>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(Vec<T>),
    CodeBlock(Box<CodeHighlight>),
    /// - `Math`:
    MathBlock(String),
    TableView(Box<TableView<T>>),
    ListView(Box<ListView<T>>),
    /// - `Code`:
    // inlined
    Normal(String),
    Raw(String),
    /// `` `code` ``
    Code(String),

    Italic(Vec<T>),
    Bold(Vec<T>),
    Emphasis(Vec<T>),

    Underline(Vec<T>),
    Strikethrough(Vec<T>),
    Undercover(Vec<T>),

    MathInline(String),
    MathDisplay(String),

    Escaped(char),
    Link(Box<SmartLink<T>>),
    //
    /// - `None`: It doesn't look like anything to me
    Null,
    String(String),
    Number(String),
    Boolean(bool),
    Array(Vec<T>),
    Object,
    Command(Box<Command<T>>),
}

impl<T> Default for ASTKind<T> {
    fn default() -> Self {
        Self::Null
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
    pub fn code(code: CodeHighlight) -> Self {
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
            "inline" => Self::MathInline(text),
            "display" => Self::MathDisplay(text),
            _ => Self::MathBlock(text),
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
            "raw" => Self::Raw(text),
            _ => Self::Normal(text),
        }
    }
    pub fn escaped(char: char) -> Self {
        Self::Escaped(char)
    }
}
