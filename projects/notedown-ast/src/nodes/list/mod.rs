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
    /// -
    /// -
    /// ```
    Hyphen,
    /// ```note
    /// >+ Summary Open
    /// ```
    SummaryOpen,
    /// ```note
    /// >- Summary Open
    /// ```
    SummaryClosed,
    /// ```note
    /// 1.
    /// 2.
    /// 3.1.
    /// 3.2.
    /// ```
    ArabicNumerals { prefix_number: Vec<usize>, number: usize },
    /// ```note
    /// I.
    /// II.
    /// ```
    RomanNumerals,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListItem {
    prefix: ListPrefixSymbol,
    rest: ASTNodes,
}

impl Default for ListPrefixSymbol {
    fn default() -> Self {
        Self::Hyphen
    }
}
