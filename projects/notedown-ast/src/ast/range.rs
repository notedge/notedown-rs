use std::fmt::{self, Debug, Formatter};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TextRange {
    // pub index: u64,
    pub start: (u64, u64),
    pub end: (u64, u64),
}

impl Default for TextRange {
    fn default() -> Self {
        Self {
            // index: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Debug for TextRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}) → ({}, {})", self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

impl TextRange {
    pub fn new(a: u64, b: u64, x: u64, y: u64) -> Self {
        Self { start: (a, b), end: (x, y) }
    }
    pub fn as_tuple(&self) -> (u64, u64, u64, u64) {
        (self.start.0, self.start.1, self.end.0, self.end.1)
    }
}
