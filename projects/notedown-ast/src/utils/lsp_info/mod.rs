mod range;

pub use self::range::TextRange;

use std::fmt::{self, Debug, Formatter, Display};

#[derive(Copy, Clone,Debug, Eq, PartialEq)]
pub struct LSPMetaInfo {
    pub range: TextRange
}

impl Display for LSPMetaInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}