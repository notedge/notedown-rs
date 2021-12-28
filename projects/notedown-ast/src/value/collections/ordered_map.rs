use super::*;
use indexmap::map::{Iter, Keys, Values};

impl OrderedMap {
    /// Get value from Ordered Map
    #[inline]
    pub fn get(&self, key: &str) -> Option<Value> {
        self.inner.get(key).map(|f| f.value.value.to_owned())
    }
    /// Get value from Ordered Map
    #[inline]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|f| bool::try_from(f).ok())
    }
    /// Get value from Ordered Map
    #[inline]
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|f| String::try_from(f).ok())
    }
}

impl OrderedMap {
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract(&mut self, key: &str) -> Option<Value> {
        self.inner.remove(key).map(|f| f.value.value)
    }
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract_bool(&mut self, key: &str) -> Option<bool> {
        self.extract(key).and_then(|f| bool::try_from(f).ok())
    }
    /// Extract value from Ordered Map
    #[inline]
    pub fn extract_string(&mut self, key: &str) -> Option<String> {
        self.extract(key).and_then(|f| String::try_from(f).ok())
    }
}

impl OrderedMap {
    /// Returns true if the array contains no elements
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Insert a ranged key-value pair in the map.
    #[inline]
    pub fn insert_raw(&mut self, pair: LiteralPair) -> Option<LiteralPair> {
        self.inner.insert(pair.key.value.to_owned(), pair)
    }
    /// Insert a key-value pair in the map.
    #[inline]
    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        let pair = LiteralPair { key: Literal { value: key, range: None }, value: Literal { value, range: None } };
        self.inner.insert(pair.key.value.to_owned(), pair).map(|pair| pair.value.value)
    }
}

/// Iterator related methods
impl OrderedMap {
    /// Return an iterator over the key-value pairs of the map in their order
    #[inline]
    pub fn iter(&self) -> OrderedMapIter {
        OrderedMapIter { inner: self.inner.iter() }
    }
    /// Return an iterator over the key-value pairs of the map in their order
    #[inline]
    pub fn iter_raw(&self) -> OrderedMapIterRaw {
        OrderedMapIterRaw { inner: self.inner.iter() }
    }
    /// Return an iterator over the keys of the map in their order
    #[inline]
    pub fn keys(&self) -> OrderedMapKeys {
        OrderedMapKeys { inner: self.inner.keys() }
    }
    /// Return an iterator over the values of the map in their order
    #[inline]
    pub fn values(&self) -> OrderedMapValues {
        OrderedMapValues { inner: self.inner.values() }
    }
}

impl<'a> IntoIterator for &'a OrderedMap {
    type Item = (&'a String, &'a Value);
    type IntoIter = OrderedMapIter<'a>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// impl<'a, K, V, S> IntoIterator for &'a mut IndexMap<K, V, S> {
//     type Item = (&'a K, &'a mut V);
//     type IntoIter = IterMut<'a, K, V>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }

/// Wrapper type of [`OrderedMap::iter`]
pub struct OrderedMapIter<'a> {
    inner: Iter<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapIter<'a> {
    type Item = (&'a String, &'a Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &v.value.value))
    }
}

impl<'a> ExactSizeIterator for OrderedMapIter<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// Wrapper type of [`OrderedMap::iter_raw`]
pub struct OrderedMapIterRaw<'a> {
    inner: Iter<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapIterRaw<'a> {
    type Item = (&'a Literal<String>, &'a Literal<Value>);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| (&v.key, &v.value))
    }
}

/// Wrapper type of [`OrderedMap::values`]
pub struct OrderedMapKeys<'a> {
    inner: Keys<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapKeys<'a> {
    type Item = &'a String;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Wrapper type of [`OrderedMap::values`]
pub struct OrderedMapValues<'a> {
    inner: Values<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapValues<'a> {
    type Item = &'a Value;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|f| &f.value.value)
    }
}

impl<'a> DoubleEndedIterator for OrderedMapValues<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|f| &f.value.value)
    }
}
