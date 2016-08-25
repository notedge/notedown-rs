use super::*;
use crate::nodes::{ListDetailedNode, ListSimpleNode, ListView};

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

impl Display for ListSimpleNode {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl Display for ListDetailedNode {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
