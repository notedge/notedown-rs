#![doc = include_str!("readme.md")]
pub(crate) mod as_element;
pub(crate) mod elements;
pub(crate) mod link;
pub(crate) mod list;
pub(crate) mod literal;
pub(crate) mod quote;
pub(crate) mod table;

pub use self::{
    elements::{code_block::*, delimiter::*, header::*, math::*, styled::*, text::*},
    link::*,
    list::*,
    literal::*,
    quote::*,
    table::*,
};
use crate::{
    command::CommandArguments,
    traits::{IntoNotedown, Slugify},
    Command, Value,
};
use diagnostic_quick::error_3rd::NodeLocation;
use num::{Signed, Zero};
use std::{
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::RangeInclusive,
};

/// Represents an AST object with position
pub type NotedownNode = NodeLocation<NotedownKind>;
/// Represents a list of AST objects with position
pub type NotedownNodes = Vec<NotedownNode>;

/// ## ASTKing
/// Typed info of the Node
///
/// - Block: Statements, Paragraph
/// - Span: Text, Styled
/// - Node: Code, Math, Link, Command
///
/// ### Notice
/// If a constructor returns individual elements, then interface accept what is
/// needed (`T`).
///
/// If a constructor returns [`NotedownNode`], then the interface implements
/// polymorphic input (`impl Into<T>`).
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum NotedownKind {
    /// Top Scope
    Statements(NotedownNodes),
    /// - block only
    Paragraph(NotedownNodes),
    /// - block only
    Delimiter(Box<Delimiter>),
    /// - block only
    Header(Box<Header>),
    /// - block only
    TableView(TableView),
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
    TextSpan(Box<TextSpan>),
    /// - inline only
    StyledSpan(Box<StyleNode>),
    /// - context sensitive
    Command(Box<Command>),
    /// - never bared
    Value(Box<Value>),
}

impl Default for NotedownKind {
    fn default() -> Self {
        Self::Value(Box::new(Value::Null))
    }
}

impl NotedownKind {
    /// Constructor of [`NotedownKind::Statements`]
    #[inline]
    pub fn statements(children: NotedownNodes, range: MaybeRanged) -> NotedownNode {
        NotedownNode { value: Self::Statements(children), range }
    }
    /// Constructor of [`NotedownKind::Paragraph`]
    #[inline]
    pub fn paragraph(children: NotedownNodes, range: MaybeRanged) -> NotedownNode {
        NotedownNode { value: Self::Paragraph(children), range }
    }
    /// Constructor of [`Delimiter::HorizontalRule`]
    #[inline]
    pub fn hr(range: MaybeRanged) -> NotedownNode {
        Delimiter::HorizontalRule.into_node(range)
    }
}
