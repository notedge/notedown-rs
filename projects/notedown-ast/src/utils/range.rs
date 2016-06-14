use crate::TextRange;
use std::fmt::{self, Debug, Formatter};

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
