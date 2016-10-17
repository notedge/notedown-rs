use super::*;
use indexmap::map::Iter;

impl OrderedMap {
    pub fn get(&self, key: &str) -> Option<Value> {
        self.inner.get(key).map(|f| f.value.value)
    }
    #[inline]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|f| bool::try_from(f).ok())
    }
    #[inline]
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|f| String::try_from(f.to_owned()).ok())
    }
}

impl OrderedMap {
    #[inline]
    pub fn extract(&mut self, key: &str) -> Option<Value> {
        self.inner.remove(key).map(|f| f.value.value)
    }
    #[inline]
    pub fn extract_bool(&mut self, key: &str) -> Option<bool> {
        self.extract(key).and_then(|f| bool::try_from(f).ok())
    }

    #[inline]
    pub fn extract_string(&mut self, key: &str) -> Option<String> {
        self.extract(key).and_then(|f| String::try_from(f).ok())
    }
}

impl OrderedMap {
    pub fn iter<'a>(self) -> OrderedMapIter<'a> {
        OrderedMapIter { inner: self.inner.iter() }
    }
    pub fn iter_raw<'a>(self) -> OrderedMapIterRaw<'a> {
        OrderedMapIterRaw { inner: self.inner.iter() }
    }
}

pub struct OrderedMapIter<'a> {
    inner: Iter<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapIter<'a> {
    type Item = (&'a String, &'a Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k, &v.value.value))
    }
}

struct OrderedMapIterRaw<'a> {
    inner: Iter<'a, String, LiteralPair>,
}

impl<'a> Iterator for OrderedMapIterRaw<'a> {
    type Item = (&'a Literal<String>, &'a Literal<Value>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| (&v.key, &v.value))
    }
}
