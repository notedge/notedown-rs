use super::*;

impl CommandOptions {
    #[inline]
    pub fn get_index(&self, index: &BigUint) -> Option<&Literal<Value>> {
        self.args.get(index)
    }
    #[inline]
    pub fn extract_index(&mut self, index: &BigUint) -> Option<Literal<Value>> {
        self.args.remove(index)
    }
    #[inline]
    pub fn get_bool_index(&self, index: &BigUint) -> Option<bool> {
        self.get_index(index).and_then(|f| bool::try_from(f.value.to_owned()).ok())
    }
    #[inline]
    pub fn get_string_index(&self, index: &BigUint) -> Option<String> {
        self.get_index(index).and_then(|f| String::try_from(f.value.to_owned()).ok())
    }
    #[inline]
    pub fn get_key(&self, key: &str) -> Option<&Literal<Value>> {
        // let mut v: std::collections::BTreeMap<String, Literal<Value>>;
        self.kvs.get(key)
    }
    #[inline]
    pub fn extract_key(&mut self, key: &str) -> Option<Literal<Value>> {
        self.kvs.remove(key)
    }
    #[inline]
    pub fn get_bool_key(&self, key: &str) -> Option<bool> {
        self.get_key(key).and_then(|f| bool::try_from(f.value.to_owned()).ok())
    }
    #[inline]
    pub fn extract_bool_key(&mut self, key: &str) -> Option<bool> {
        self.extract_key(key).and_then(|f| bool::try_from(f.value).ok())
    }
    #[inline]
    pub fn get_string_key(&self, key: &str) -> Option<String> {
        self.get_key(key).and_then(|f| String::try_from(f.value.to_owned()).ok())
    }
    #[inline]
    pub fn extract_string_key(&mut self, key: &str) -> Option<String> {
        self.extract_key(key).and_then(|f| String::try_from(f.value).ok())
    }
}

impl CommandPattern {
    #[inline]
    pub fn get_view(&self) -> Vec<String> {
        self.pts.iter().cloned().map(|s| s.value).collect()
    }
    #[inline]
    pub fn get_length(&self) -> usize {
        self.pts.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pts.is_empty()
    }
}
