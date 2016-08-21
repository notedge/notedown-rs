use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    Quote(Box<QuoteList>),
    Ordered(Box<OrderedList>),
    Orderless(Box<OrderlessList>),
    Details(Box<DetailsList>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ListPrefixSymbol {
    /// Varies according to global settings
    Default = 0,
    /// ```note
    /// 1.
    /// 2.
    /// ```
    ArabicNumerals,
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

/// ## Quote List
/// ```note
/// > part1
/// > part2
///   part2
/// > part3
///
/// > part4
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct QuoteList {
    style: String,
    children: Vec<Literal<ListItem>>,
}

/// ## Orderless List
/// ```note
/// 1.1. part1
/// 1.2. part2
///      part2
/// 1.3. part3
///
/// 1.4. part4
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderedList {
    prefix: ListPrefixSymbol,
    head: Vec<usize>,
    body: ASTNodes,
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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderlessList {
    body: ASTNodes,
}

/// ## Details List
/// ```note
/// >- summary part
/// >- sum
/// >  part2
/// >  part3
/// ```
///
/// ```html
/// <details open>
/// <summary>Want to ruin the surprise?</summary>
/// <br>
/// Well, you asked for it!
/// </details>
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DetailsList {
    is_open: bool,
    summary: ASTNodes,
    body: ASTNodes,
}

impl Default for ListPrefixSymbol {
    fn default() -> Self {
        Self::Default
    }
}
