use crate::nodes::{Array, Literal, Object, Value};
use num::BigUint;
use std::{
    convert::TryFrom,
    hash::{Hash, Hasher},
};

#[derive(Clone, Default, Eq, PartialEq)]
pub struct CommandOptions {
    args: Array,
    kvs: Object,
}

impl Hash for CommandOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.args.hash(state);
        for i in &self.kvs {
            i.hash(state)
        }
    }
}

impl CommandOptions {
    #[inline]
    pub fn extract_index(&mut self, index: &BigUint) -> Option<Literal<Value>> {
        self.args.remove(index)
    }
    #[inline]
    pub fn extract_bool_index(&mut self, index: &BigUint) -> Option<bool> {
        self.extract_index(index).and_then(|f| bool::try_from(f.value).ok())
    }
    #[inline]
    pub fn extract_string_index(&mut self, index: &BigUint) -> Option<String> {
        self.args.remove(index).and_then(|f| String::try_from(f.value).ok())
    }
    #[inline]
    pub fn extract_key(&mut self, key: &str) -> Option<Literal<Value>> {
        // let mut v: std::collections::BTreeMap<String, Literal<Value>>;
        self.kvs.remove(key)
    }
    #[inline]
    pub fn extract_bool_key(&mut self, key: &str) -> Option<bool> {
        self.extract_key(key).and_then(|f| bool::try_from(f.value).ok())
    }
    #[inline]
    pub fn extract_string_key(&mut self, key: &str) -> Option<String> {
        self.extract_key(key).and_then(|f| String::try_from(f.value).ok())
    }
}
