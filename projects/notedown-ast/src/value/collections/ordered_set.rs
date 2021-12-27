use super::*;
use indexmap::set::Iter;

impl OrderedSet {
    /// Returns an iterator over the slice.
    #[inline]
    pub fn iter(&self) -> OrderedSetIter {
        OrderedSetIter { inner: self.inner.iter() }
    }
}

impl<'i> IntoIterator for &'i OrderedSet {
    type Item = &'i Value;
    type IntoIter = OrderedSetIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        OrderedSetIter { inner: self.inner.iter() }
    }
}
/// Wrapper type of [`OrderedSet::iter`]
pub struct OrderedSetIter<'i> {
    inner: Iter<'i, Literal<Value>>,
}

impl<'i> Iterator for OrderedSetIter<'i> {
    type Item = &'i Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value)
    }
}

impl<'i> DoubleEndedIterator for OrderedSetIter<'i> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|f| &f.value)
    }
}

impl<'i> ExactSizeIterator for OrderedSetIter<'i> {}
