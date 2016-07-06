use super::*;

type RangeSize = u64;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TextRange {
    // pub index: u64,
    pub start: (RangeSize, RangeSize),
    pub end: (RangeSize, RangeSize),
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
    pub fn new(a: impl Into<RangeSize>, b: impl Into<RangeSize>, x: impl Into<RangeSize>, y: impl Into<RangeSize>) -> Self {
        Self { start: (a.into(), b.into()), end: (x.into(), y.into()) }
    }
    pub fn sum(&self) -> RangeSize {
        self.start.0 + self.start.1 + self.end.0 + self.end.1
    }
    pub fn as_tuple(&self) -> (RangeSize, RangeSize, RangeSize, RangeSize) {
        (self.start.0, self.start.1, self.end.0, self.end.1)
    }
    pub fn boxed(self) -> Option<Box<TextRange>> {
        match self.sum() {
            0 => None,
            _ => Some(Box::new(self)),
        }
    }
}
