use super::*;
use num::One;

impl SparseArray {
    /// TODO: doc
    #[inline]
    pub fn get(&self, index: &BigUint) -> Option<Value> {
        self.inner.get(index).map(|f| f.value.to_owned())
    }
    /// TODO: doc
    #[inline]
    pub fn get_bool(&self, index: &BigUint) -> Option<bool> {
        self.get(index).and_then(|f| bool::try_from(f.to_owned()).ok())
    }
    /// TODO: doc
    #[inline]
    pub fn get_string(&self, index: &BigUint) -> Option<String> {
        self.get(index).and_then(|f| String::try_from(f.to_owned()).ok())
    }
    /// TODO: doc
    #[inline]
    pub fn last_key_value(&self) -> Option<(&BigUint, &Value)> {
        self.inner.last_key_value().map(|(k, v)| (k, &v.value))
    }
}

impl SparseArray {
    /// TODO: doc
    #[inline]
    pub fn extract(&mut self, index: &BigUint) -> Option<Value> {
        self.inner.remove(index).map(|f| f.value)
    }
}

impl SparseArray {
    /// TODO: doc
    #[inline]
    #[allow(mutable_borrow_reservation_conflict)]
    pub fn push(&mut self, value: Literal<Value>) {
        let last = self.inner.last_key_value().map(|f| f.0);
        match last {
            None => self.insert(BigUint::one(), value),
            Some(s) => self.inner.insert(s + 1u8, value),
        };
    }
    /// TODO: doc
    #[inline]
    pub fn insert(&mut self, index: BigUint, value: Literal<Value>) -> Option<Literal<Value>> {
        self.inner.insert(index, value)
    }
}

impl SparseArray {
    /// Return an iterator over array with default value if not set
    #[inline]
    pub fn iter(&self) -> SparseArrayIter {
        SparseArrayIter { current: BigUint::one(), default: &self.default, inner: &self.inner }
    }
}

/// Wrapper type of [`SparseArray::values`]
pub struct SparseArrayIter<'a> {
    current: BigUint,
    default: &'a Value,
    inner: &'a BTreeMap<BigUint, Literal<Value>>,
}

impl<'a> Iterator for SparseArrayIter<'a> {
    type Item = &'a Value;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1u8;
        match self.inner.get(&self.current) {
            None => Some(&self.default),
            Some(s) => Some(&s.value),
        }
    }
}
