use super::*;

mod detailed;
mod item;
mod ordered;
mod orderless;
mod prefix;

pub use self::{detailed::DetailedList, item::ListItem, ordered::OrderedList, orderless::OrderlessList, prefix::ListPrefixSymbol};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    Ordered(Box<OrderedList>),
    Orderless(Box<OrderlessList>),
}

impl ListView {
    /// ## Ordered List
    /// ```note
    /// 1. part1
    /// 2. part2
    ///    part2
    /// 3. part3
    ///
    /// 4. part4
    /// ```
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>) -> Self {
        let list = OrderedList { first_order: 0, children };
        Self::Ordered(box list)
    }
    /// ## Orderless List
    /// ```note
    /// - part1
    /// - part2
    ///   part2
    /// - part3
    ///
    /// - part4
    /// ```
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>) -> Self {
        let list = OrderlessList { children };
        Self::Orderless(box list)
    }
}

impl ASTKind {
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>, r: MaybeRanged) -> ASTNode {
        ListView::ordered_list(children).into_node(r)
    }
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>, r: MaybeRanged) -> ASTNode {
        ListView::orderless_list(children).into_node(r)
    }
}
