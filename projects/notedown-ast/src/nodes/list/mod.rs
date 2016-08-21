use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ListView {
    Quote(Box<QuoteList>),
    Ordered(Box<OrderedList>),
    Orderless(Box<OrderlessList>),
    Details(Box<DetailsList>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct QuoteList {
    style: String,
    body: ASTNodes,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderedList {
    head: usize,
    body: ASTNodes,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderlessList {
    body: ASTNodes,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DetailsList {
    summary: ASTNodes,
    body: ASTNodes,
}