use super::*;

mod detailed;
mod ordered;
mod orderless;
mod prefix;
mod quote;

pub use self::{detailed::DetailedList, ordered::OrderedList, orderless::OrderlessList, prefix::ListPrefixSymbol, quote::QuoteList};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    /// ## Quote List
    /// ```note
    /// > part1
    /// > part2
    ///   part2
    /// > part3
    ///
    /// > part4
    /// ```
    Quote(Box<QuoteList>),
    /// ## Ordered List
    /// ```note
    /// 1.1. part1
    /// 1.2. part2
    ///      part2
    /// 1.3. part3
    ///
    /// 1.4. part4
    /// ```
    Ordered(Box<OrderedList>),
    /// ## Orderless List
    /// ```note
    /// - part1
    /// - part2
    ///   part2
    /// - part3
    ///
    /// - part4
    /// ```
    Orderless(Box<OrderlessList>),

    Details(Box<DetailedList>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListItem {
    pub prefix: Literal<ListPrefixSymbol>,
    pub rest: ASTNodes,
}

impl From<ListView> for ASTNode {
    fn from(node: ListView) -> Self {
        Self { value: ASTKind::ListView(node), range: None }
    }
}

impl From<ASTNodes> for ListItem {
    fn from(node: ASTNodes) -> Self {
        Self { prefix: Default::default(), rest: node }
    }
}
