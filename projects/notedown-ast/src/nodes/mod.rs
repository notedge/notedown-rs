mod elements;
mod literal;
mod value;

// used for ide hint
#[cfg(debug_assertions)]
mod remap {
    pub use std::collections::btree_map::{Keys, Values};

    pub type Set<V> = std::collections::BTreeSet<V>;
    pub type Map<K, V> = std::collections::BTreeMap<K, V>;
}

#[cfg(not(debug_assertions))]
mod remap {
    pub use indexmap::map::{Keys, Values};

    pub type Set<V> = indexmap::IndexSet<V>;
    pub type Map<K, V> = indexmap::IndexMap<K, V>;
}

use self::remap::{Map, Set};
pub use self::{
    elements::*,
    literal::Literal,
    value::{Value, ValueType},
};
use num::{BigInt, BigUint};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    mem::transmute,
};

pub type ASTNode = Literal<ASTKind>;
pub type ASTNodes = Vec<Literal<ASTKind>>;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    Delimiter(Box<Delimiter>),
    HorizontalRule,
    ///  - `Paragraph`:
    Paragraph(ASTNodes),
    CodeBlock(Box<CodeNode>),
    /// - `Math`:
    TableView(Box<TableView>),
    ListView(Box<ListView>),


    TextSpan(Box<TextNode>),
    MathNode(Box<MathNode>),
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
        ASTNode { value: self, range: ASTNode::range_from(range) }
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
    pub fn code(code: CodeNode) -> Self {
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
        Self::MathNode(Box::new(node))
    }
    pub fn styled(children: ASTNodes, style: &str) -> Self {
        Self::TextSpan(Box::new(TextNode::styled(children, style)))
    }
    pub fn text(text: String, style: &str) -> Self {
        Self::TextSpan(Box::new(TextNode::styled(children, style)))
    }
    pub fn escaped(char: char) -> Self {
        Self::Escaped(char)
    }
}
