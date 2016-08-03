mod elements;
mod literal;
mod value;

pub use indexmap::map::{Keys, Values};
use indexmap::set::IndexSet;
use indexmap::map::IndexMap;
pub use self::{
    elements::*,
    literal::Literal,
    value::{Value, ValueType, Set, Array, Object},
};
use std::collections::BTreeMap;
use num::{BigInt, BigUint};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    mem::transmute,
};

pub type ASTNode = Literal<ASTKind>;
pub type ASTNodes = Vec<Literal<ASTKind>>;

/// Block,
/// - Block:
/// - Span: Text, Styled
/// - Node: Code, Math, Link, Command
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
    ///  - `Paragraph`:
    Paragraph(ASTNodes),
    /// block
    Delimiter(Box<Delimiter>),
    ///
    TableView(Box<TableView>),
    ///
    ListView(Box<ListView>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    CodeNode(Box<CodeNode>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    MathNode(Box<MathNode>),
    /// block: ``` a ```
    /// span: `` `code`  ``
    LinkNode(Box<SmartLink>),
    /// span
    TextSpan(Box<TextNode>),
    /// span
    StyledSpan(Box<StyleNode>),
    /// in
    Command(Box<Command>),
    Value(Box<Value>),
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Value(Box::new(Value::Null))
    }
}

impl ASTKind {
    pub fn statements(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Statements(children), range }
    }
    pub fn paragraph(children: ASTNodes, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Paragraph(children), range }
    }
    pub fn header(children: ASTNodes, level: u8) -> Self {
        let header = Header { level, children };
        Self::Header(Box::new(header))
    }
    pub fn hr(range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: Self::Delimiter(Box::new(Delimiter::HorizontalRule)), range }
    }
}
