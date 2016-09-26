mod elements;
mod link;
mod list;
mod literal;
mod quote;
mod table;

pub use self::{
    elements::*,
    link::{EmailLink, HyperLink, HyperLinkTarget, ImageLayout, ImageLink, ResourceDescriptor, SmartLink, TagReference, TwoWayLink},
    list::{DetailedList, ListItem, ListPrefixSymbol, ListView, OrderedList, OrderlessList, QuoteList},
    literal::Literal,
    quote::QuoteBlock,
    table::TableView,
};
use crate::{Command, Value};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

/// Maybe have ast position
pub type MaybeRanged = Option<Range<usize>>;
/// Represents an AST object with position
pub type ASTNode = Literal<ASTKind>;
/// Represents a list of AST objects with position
pub type ASTNodes = Vec<Literal<ASTKind>>;

/// - Block:
/// - Span: Text, Styled
/// - Node: Code, Math, Link, Command
///
/// ### Notice
/// If a constructor returns individual elements, then interface accept what is needed (`T`).
///
/// If a constructor returns [`ASTNode`], then the interface implements polymorphic input (`impl Into<T>`).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ASTKind {
    /// Top Scope
    Statements(ASTNodes),
    /// - block only
    Paragraph(ASTNodes),
    /// - block only
    Delimiter(Box<Delimiter>),
    /// - block only
    Header(Box<Header>),
    /// - block only
    TableView(Box<TableView>),
    /// - block only
    ListView(ListView),
    /// - block only
    QuoteNode(Box<QuoteBlock>),
    /// - block + inline
    CodeNode(Box<CodeNode>),
    /// - block + inline
    MathNode(Box<MathNode>),
    /// - block + inline
    LinkNode(SmartLink),
    /// - inline only
    TextSpan(Box<TextKind>),
    /// - inline only
    StyledSpan(Box<StyleNode>),
    /// - context sensitive
    Command(Box<Command>),
    /// - never bared
    Value(Box<Value>),
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Value(Box::new(Value::Null))
    }
}

impl ASTKind {
    #[inline]
    pub fn into_node(self, range: MaybeRanged) -> ASTNode {
        ASTNode { value: self, range }
    }
    #[inline]
    pub fn statements(children: ASTNodes, range: MaybeRanged) -> ASTNode {
        ASTNode { value: Self::Statements(children), range }
    }
    #[inline]
    pub fn paragraph(children: ASTNodes, range: MaybeRanged) -> ASTNode {
        ASTNode { value: Self::Paragraph(children), range }
    }
    #[inline]
    pub fn header(children: ASTNodes, level: u8, range: MaybeRanged) -> ASTNode {
        Header { level, children }.into_node(range)
    }
    #[inline]
    pub fn hr(range: MaybeRanged) -> ASTNode {
        Delimiter::HorizontalRule.into_node(range)
    }
}
