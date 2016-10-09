use super::*;
use crate::nodes::{DetailedList, ListView};

impl Debug for ListView {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("ListView");
        w.finish()
    }
}

impl Display for ListView {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        todo!()
    }
}

impl Display for DetailedList {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
