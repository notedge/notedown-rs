use super::*;
use num::Zero;
use std::collections::btree_map::{Iter, Keys};

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
    pub fn iter(&self) -> Iter<'_, BigUint, Literal<Value>> {
        self.inner.iter()
    }
    /// TODO: doc
    #[inline]
    pub fn keys(&self) -> Keys<'_, BigUint, Literal<Value>> {
        self.inner.keys()
    }
    /// TODO: doc
    #[inline]
    pub fn values(&self) -> SparseArrayValues {
        SparseArrayValues { current: BigUint::zero(), default: &self.default, inner: &self.inner }
    }
}
/// Wrapper type of [`SparseArray::values`]
pub struct SparseArrayValues<'a> {
    current: BigUint,
    default: &'a Value,
    inner: &'a BTreeMap<BigUint, Literal<Value>>,
}

impl<'a> Iterator for SparseArrayValues<'a> {
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
