mod elements;
mod link;
mod list;
mod literal;
mod table;
mod value;

pub use self::{
    elements::*,
    link::{EmailLink, HyperLink, HyperLinkTarget, ImageLayout, ImageLink, SmartLink, TagReference, TwoWayLink},
    list::ListView,
    literal::Literal,
    table::TableView,
    value::{Value, ValueType},
};

use indexmap::{map::IndexMap, set::IndexSet};
use num::{BigInt, BigUint};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

pub type OffsetRange = Range<usize>;
pub type ASTNode = Literal<ASTKind>;
pub type ASTNodes = Vec<Literal<ASTKind>>;
pub type Set = IndexSet<Literal<Value>>;
pub type Array = BTreeMap<BigUint, Literal<Value>>;
pub type Object = IndexMap<String, Literal<Value>>;

/// Block,
/// - Block:
/// - Span: Text, Styled
/// - Node: Code, Math, Link, Command
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    ///  - `Paragraph`:
    Paragraph(ASTNodes),
    /// block
    Delimiter(Box<Delimiter>),
    // Blocks
    /// - `Header`: TEXT, level
    Header(Box<Header>),
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
    LinkNode(SmartLink),
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
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: self, range }
    }
    #[inline]
    pub fn statements(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: Self::Statements(children), range }
    }
    #[inline]
    pub fn paragraph(children: ASTNodes, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: Self::Paragraph(children), range }
    }
    #[inline]
    pub fn header(children: ASTNodes, level: u8, range: Option<OffsetRange>) -> ASTNode {
        Header { level, children }.into_node(range)
    }
    #[inline]
    pub fn hr(range: Option<OffsetRange>) -> ASTNode {
        Delimiter::HorizontalRule.into_node(range)
    }
}
