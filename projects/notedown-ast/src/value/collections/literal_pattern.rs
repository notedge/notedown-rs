use super::*;
use std::{
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    slice::Iter,
};

impl Debug for LiteralPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter().cloned().map(|s| s.value)).finish()
    }
}

impl Display for LiteralPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in &self.inner {
            write!(f, "[{}]", i)?;
        }
        Ok(())
    }
}

impl Hash for LiteralPattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in &self.inner {
            i.value.hash(state)
        }
    }
}

impl LiteralPattern {
    /// Returns the number of elements in the vector, also referred to as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Returns true if the vector contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl LiteralPattern {
    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> LiteralPatternIter {
        LiteralPatternIter { inner: self.inner.iter() }
    }
}

/// Wrapper type of [`LiteralPattern::iter`]
pub struct LiteralPatternIter<'i> {
    inner: Iter<'i, Literal<String>>,
}

impl<'i> Iterator for LiteralPatternIter<'i> {
    type Item = &'i String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value)
    }
}
