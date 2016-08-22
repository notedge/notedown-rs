use super::*;
use crate::nodes::{ListDetailedNode, ListView, OrderedList, OrderlessList, QuoteList};

impl Debug for ListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Quote(v) => Debug::fmt(v, f),
            Self::Ordered(v) => Debug::fmt(v, f),
            Self::Orderless(v) => Debug::fmt(v, f),
            Self::Details(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ListView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Quote(v) => Display::fmt(v, f),
            Self::Ordered(v) => Display::fmt(v, f),
            Self::Orderless(v) => Display::fmt(v, f),
            Self::Details(v) => Display::fmt(v, f),
        }
    }
}

impl Display for QuoteList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Display for OrderedList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Display for OrderlessList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Display for ListDetailedNode {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
