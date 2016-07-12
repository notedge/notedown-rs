mod elements;
mod traits;
mod styled;
mod math;

pub use self::elements::*;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};
pub use self::styled::StyledNode;
pub use self::math::MathNode;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
    TableView(Box<TableView<T>>),
    ListView(Box<ListView<T>>),
    /// - `Code`:
    // inlined
    Normal(String),
    Raw(String),
    /// `` `code` ``
    Code(String),

    Styled(StyledNode<T>),

    Math(MathNode),

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
        unimplemented!()
    }

    pub fn styled(children: Vec<T>, style: &str) -> Self {
        Self::Styled(StyledNode::new(children, style))
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
