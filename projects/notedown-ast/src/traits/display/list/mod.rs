use super::*;
use crate::nodes::{DetailedList, ListView};

impl Debug for ListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ordered(v) => Debug::fmt(v, f),
            Self::Orderless(v) => Debug::fmt(v, f),
        }
    }
}

impl Display for ListView {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl Debug for OrderedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("List");
        todo!();
        w.finish()
    }
}

impl Debug for OrderlessList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("List");
        todo!();
        w.finish()
    }
}

impl Display for DetailedList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
