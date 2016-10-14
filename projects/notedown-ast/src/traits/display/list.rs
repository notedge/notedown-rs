use super::*;
use crate::nodes::{DetailedList, ListView, OrderedList, OrderlessList};

impl Debug for ListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ordered(v) => Debug::fmt(v, f),
            Self::Orderless(v) => Debug::fmt(v, f),
            Self::Details(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ListView {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Ordered(v) => Display::fmt(v, f),
            Self::Orderless(v) => Display::fmt(v, f),
            Self::Details(v) => Display::fmt(v, f),
        }
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

impl Display for DetailedList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
