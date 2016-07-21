mod elements;
mod value;
mod literal;

pub use self::elements::*;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};
use crate::nodes::value::Value;
use std::mem::transmute;
pub use self::literal::ASTNodes;
pub use self::literal::{ASTNode, Literal};
use num::{BigUint, BigInt};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(ASTNodes),
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
    Value(Box<Value>),
    Command(Box<Command>),
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Value(Box::new(Value::Null))
    }
}

impl ASTKind {
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode {
            value: self,
            range: ASTNode::range_from(range),
        }
    }

    pub fn statements(children: ASTNodes) -> Self {
        Self::Statements(children)
    }
    pub fn paragraph(children: ASTNodes) -> Self {
        Self::Paragraph(children)
    }
    pub fn header(children: ASTNodes, level: usize) -> Self {
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
    pub fn styled(children: ASTNodes, style: &str) -> Self {
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
