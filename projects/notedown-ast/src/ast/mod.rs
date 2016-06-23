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
pub enum AST {
    Node {
        kind: ASTKind,
        //      children: Box<[AST]>,
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
    Statements,
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph,
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

    Italic,
    Bold,
    Emphasis,

    Underline,
    Strikethrough,
    Undercover,

    MathInline(Box<String>),
    MathDisplay(Box<String>),

    Link(Box<SmartLink>),
    Escaped(char),
    //
    Command(Box<Command>),
    String(Box<String>),
    Number(Box<String>),
    Boolean(bool),
    Array,
    Object,
}

impl Default for AST {
    fn default() -> Self {
        Self::Leaf { kind: ASTKind::None, r: Default::default() }
    }
}

impl AST {
    pub fn kind(&self) -> ASTKind {
        match self {
            Self::Node { kind, .. } | Self::Leaf { kind, .. } => kind.to_owned(),
        }
    }

    pub fn children(&self) -> Vec<AST> {
        match self {
            Self::Node { children, .. } => children.to_vec(),
            Self::Leaf { .. } => vec![],
        }
    }
    pub fn range(&self) -> TextRange {
        match self {
            Self::Node { r, .. } | Self::Leaf { r, .. } => r.clone().unwrap_or_default().as_ref().clone(),
        }
    }
}

impl AST {
    pub fn statements(children: Vec<AST>, r: TextRange) -> Self {
        Self::Node { kind: ASTKind::Statements, children, r: Some(Box::new(r)) }
    }
}
