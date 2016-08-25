use super::*;

mod detailed;
mod simple;

pub use self::{detailed::ListDetailedNode, simple::ListSimpleNode};

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
    Quote(Box<ListSimpleNode>),
    /// ## Ordered List
    /// ```note
    /// 1.1. part1
    /// 1.2. part2
    ///      part2
    /// 1.3. part3
    ///
    /// 1.4. part4
    /// ```
    Ordered(Box<ListSimpleNode>),
    /// ## Orderless List
    /// ```note
    /// - part1
    /// - part2
    ///   part2
    /// - part3
    ///
    /// - part4
    /// ```
    Orderless(Box<ListSimpleNode>),

    Details(Box<ListDetailedNode>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ListPrefixSymbol {
    /// ```note
    /// -
    /// ```
    Hyphen,
    /// ```note
    /// >
    /// ```
    Quote,
    /// ```note
    /// >+ Summary Open
    /// ```
    SummaryOpen,
    /// ```note
    /// >- Summary Open
    /// ```
    SummaryClosed,
    /// Single, serial number from the beginning
    /// ```note
    /// 1.
    /// 2.
    /// 3.
    /// ```
    Arabic,
    /// Complex, multi-layered, not serial number from the beginning
    /// ```note
    /// 4.4.
    /// 4.5.
    /// 4.6.
    /// ```
    ArabicNest { prefix_number: Vec<usize>, number: usize },
    /// ```note
    /// I.
    /// II.
    /// ```
    RomanNumerals,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListItem {
    prefix: Literal<ListPrefixSymbol>,
    rest: ASTNodes,
}

impl Default for ListPrefixSymbol {
    fn default() -> Self {
        Self::Hyphen
    }
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
