mod detailed;
mod item;
mod ordered;
mod orderless;
mod prefix;

pub use self::{detailed::DetailedList, item::ListItem, ordered::OrderedList, orderless::OrderlessList, prefix::ListPrefixSymbol};

use super::*;
use crate::{NoteError, Result};

/// List like nodes
/// Basically can be classified as ordered and orderless
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    /// Ordered list
    Ordered(Box<OrderedList>),
    /// Orderless list
    Orderless(Box<OrderlessList>),
}

impl ListView {
    /// Returns the children nodes of this list
    #[inline]
    pub fn children(&self) -> &Vec<ListItem> {
        match self {
            Self::Ordered(v) => &v.children,
            Self::Orderless(v) => &v.children,
        }
    }
    /// Returns the mutable children nodes of this list
    #[inline]
    pub fn children_mut(&mut self) -> &mut Vec<ListItem> {
        match self {
            Self::Ordered(v) => &mut v.children,
            Self::Orderless(v) => &mut v.children,
        }
    }
    /// Returns the first prefix of this list
    #[inline]
    pub fn first_prefix(&self) -> Result<&ListPrefixSymbol> {
        match self.children().first() {
            None => Err(NoteError::runtime_error("Not a valid list")),
            Some(s) => Ok(&s.prefix.value),
        }
    }
    /// Check if this list is ordered
    #[inline]
    pub fn is_ordered(&self) -> bool {
        matches!(self, Self::Ordered(_))
    }
}

impl ListView {
    /// Constructor of [`OrderedList`]
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>) -> Self {
        let list = OrderedList { first_order: 0, children };
        Self::Ordered(box list)
    }
    /// Constructor of [`OrderlessList`]
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>) -> Self {
        let list = OrderlessList { children };
        Self::Orderless(box list)
    }
}

impl ASTKind {
    /// Constructor of [`OrderedList`]
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>, r: MaybeRanged) -> ASTNode {
        ListView::ordered_list(children).into_node(r)
    }
    /// Constructor of [`OrderlessList`]
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>, r: MaybeRanged) -> ASTNode {
        ListView::orderless_list(children).into_node(r)
    }
}
