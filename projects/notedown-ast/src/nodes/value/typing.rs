use super::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ValueType {
    /// - `None`: It doesn't look like anything to me
    Null,
    Boolean,
    Integer,
    Decimal,
    String,
    Set(BTreeSet<ValueType>),
    List(BTreeSet<ValueType>),
    Object(BTreeMap<String, ValueType>),
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Self::Null => ValueType::Null,
            Self::Boolean(_) => ValueType::Boolean,
            Self::Integer(_) => ValueType::Integer,
            Self::Decimal(_) => ValueType::Decimal,
            Self::String(_) => ValueType::String,
            Self::Set(v) => self.check_set_type(v),
            Self::Array(v) => self.check_list_type(v),
            Self::Object(v) => self.check_dict_type(v),
        }
    }
    pub fn get_type_name(&self) -> String {
        self.get_type().to_string()
    }
    fn check_set_type(&self, input: &Set) -> ValueType {
        let mut count = BTreeSet::new();
        for v in input {
            count.insert(v.value.get_type());
        }
        ValueType::Set(count)
    }
    fn check_list_type(&self, input: &Array) -> ValueType {
        let mut count = BTreeSet::new();
        for (_, v) in input {
            count.insert(v.value.get_type());
        }
        ValueType::List(count)
    }
    fn check_dict_type(&self, input: &Object) -> ValueType {
        unimplemented!()
    }
}
