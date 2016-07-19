mod elements;

pub use self::elements::*;
use crate::{ASTNode, ASTNodes};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(Vec<ASTNode>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(Vec<ASTNode>),
    CodeBlock(Box<CodeHighlight>),
    /// - `Math`:
    TableView(Box<TableView>),
    ListView(Box<ListView>),
    /// - `Code`:
    // inlined
    Normal(String),
    Raw(String),
    /// `` `code` ``
    Code(String),

    Styled(Box<StyledNode>),

    Math(Box<MathNode>),

    Escaped(char),
    Link(Box<SmartLink<String>>),
    //
    /// - `None`: It doesn't look like anything to me
    Null,
    String(String),
    Number(String),
    Boolean(bool),
    Array(Vec<ASTNode>),
    Object,
    Command(Box<Command>),
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Null
    }
}

impl ASTKind {
    pub fn statements(children: Vec<ASTNode>) -> Self {
        Self::Statements(children)
    }
    pub fn paragraph(children: Vec<ASTNode>) -> Self {
        Self::Paragraph(children)
    }
    pub fn header(children: Vec<ASTNode>, level: usize) -> Self {
        let header = Header { level, children };
        Self::Header(Box::new(header))
    }
    pub fn code(code: CodeHighlight) -> Self {
        Self::CodeBlock(Box::new(code))
    }
    pub fn command(cmd: Command) -> Self {
        Self::Command(Box::new(cmd))
    }

    pub fn hr() -> ASTKind {
        Self::HorizontalRule
    }

    pub fn math(text: String, style: &str) -> Self {
        let node = match style {
            "$" => MathNode::inline(text),
            "$$" => MathNode::display(text),
            _ => MathNode::block(text),
        };
        Self::Math(Box::new(node))
    }
    pub fn styled(children: Vec<ASTNode>, style: &str) -> Self {
        Self::Styled(Box::new(StyledNode::new(children, style)))
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
