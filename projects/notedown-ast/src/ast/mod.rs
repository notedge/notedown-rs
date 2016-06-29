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
    pub  range: Option<Box<TextRange>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind<T> {
    /// - `None`: It doesn't look like anything to me
    None,
    /// Top
    Statements(Vec<T>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(Vec<T>),
    CodeBlock(Box<CodeBlock>),
    /// - `Math`:
    MathBlock(Box<String>),
    TableView(Box<TableView>),
    ListView(Box<ListView>),
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
    Command(Box<Command>),
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

impl ASTNode {
    pub fn children(&self) -> Vec<ASTNode> {
        match &self.kind {
            ASTKind::Statements(v) => {v.to_owned()}
            _ => unimplemented!()
        }
    }
}

impl ASTNode {
    pub fn statements(children: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::Statements(children) , range: box_range(r) }
    }
    pub fn paragraph(children: Vec<ASTNode>, r: TextRange) -> Self {
        Self { kind: ASTKind::Paragraph(children) , range: box_range(r) }
    }
    pub fn header(children: Vec<ASTNode>, level: usize, r: TextRange) -> Self {
        let header = Header { level, children };
        Self { kind: ASTKind::Header(Box::new(header)), range: box_range(r) }
    }
    pub fn code(code: CodeBlock, r: TextRange) -> ASTNode {
        Self { kind: ASTKind::CodeBlock(Box::new(code)), range: box_range(r) }
    }
    pub fn command(cmd: Command, r: TextRange) -> ASTNode {
        Self { kind: ASTKind::Command(Box::new(cmd)), range: box_range(r) }
    }

    pub fn hr(r: TextRange) -> ASTNode {
        Self { kind: ASTKind::HorizontalRule, range: box_range(r) }
    }

    pub fn math(text: String, style: &str, r: TextRange) -> Self {
        let kind = match style {
            "inline" => ASTKind::MathInline(Box::new(text)),
            "display" => ASTKind::MathDisplay(Box::new(text)),
            _ => ASTKind::MathBlock(Box::new(text)),
        };
        Self { kind, range: box_range(r) }
    }
    pub fn style(children: Vec<ASTNode>, style: &str, r: TextRange) -> Self {
        let kind = match style {
            "*" | "i" | "italic" => ASTKind::Italic(children),
            "**" | "b" | "bold" => ASTKind::Bold(children),
            "***" | "em" => ASTKind::Emphasis(children),
            "~" | "u" | "underline" => ASTKind::Underline(children),
            "~~" | "s" => ASTKind::Strikethrough(children),
            "~~~" => ASTKind::Undercover(children),
            _ => unreachable!(),
        };
        Self { kind,  range: box_range(r) }
    }
    pub fn text(text: String, style: &str, r: TextRange) -> Self {
        let kind = match style {
            "raw" => ASTKind::Raw(Box::new(text)),
            _ => ASTKind::Normal(Box::new(text)),
        };
        Self { kind, range: box_range(r) }
    }
    pub fn escaped(char: char, r: TextRange) -> Self {
        Self { kind: ASTKind::Escaped(char), range: box_range(r) }
    }
}

fn box_range(r: TextRange) -> Option<Box<TextRange>> {
    match r.sum() {
        0 => None,
        _ => box_range(r),
    }
}
