use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

use diagnostic_quick::error_3rd::NodeLocation;

struct Bucket<T> {
    key: NodeLocation<String>,
    value: NodeLocation<T>,
}

#[derive(Default)]
pub struct Dict<T> {
    dict: BTreeMap<String, Bucket<T>>,
}

impl<T: Debug> Debug for Dict<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Dict<T> {
    pub fn insert(&mut self, key: NodeLocation<String>, value: NodeLocation<T>) {
        self.dict.insert(key.value, Bucket { key, value });
    }
    pub fn get_key(&self, key: &str) -> Option<&NodeLocation<String>> {
        self.dict.get(key).map(|v| &v.key)
    }
    pub fn mut_key(&mut self, key: &str) -> Option<&mut NodeLocation<String>> {
        self.dict.get_mut(key).map(|v| &mut v.key)
    }
    pub fn get_value(&self, key: &str) -> Option<&NodeLocation<T>> {
        self.dict.get(key).map(|v| &v.value)
    }
    pub fn mut_value(&mut self, key: &str) -> Option<&mut NodeLocation<T>> {
        self.dict.get_mut(key).map(|v| &mut v.value)
    }
}
