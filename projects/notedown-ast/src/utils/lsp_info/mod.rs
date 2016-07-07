
pub use lsp_types::{Range, Url, Position};

use std::fmt::{self, Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LSPMetaInfo {
    pub range: Range,
    pub url: Option<Rc<Url>>
}

impl Default for LSPMetaInfo {
    fn default() -> Self {
        Self {
            range: Default::default(),
            url: None
        }
    }
}

impl Display for LSPMetaInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl LSPMetaInfo {
    pub fn set_range(&mut self, x1: u32, y1: u32, x2: u32, y2: u32) {
        self.range = Range {
            start: Position {
                line: x1,
                character: y1
            },
            end: Position {
                line: x2,
                character: y2
            }
        }
    }
    pub fn set_url(&mut self) {

    }
}