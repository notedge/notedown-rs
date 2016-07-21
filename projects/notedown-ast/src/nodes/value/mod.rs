use super::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /// - `None`: It doesn't look like anything to me
    Null,
    String(String),
    Integer(BigInt),
    Decimal(f64),
    Boolean(bool),
    Set(BTreeSet<Literal<Value>>),
    Array(BTreeMap<Literal<BigUint>, Literal<Value>>),
    Object(BTreeMap<Literal<String>, Literal<Value>>),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => {().hash(state)}
            Value::String(v) => {v.hash(state)}
            Value::Integer(v) => {v.hash(state)}
            Value::Decimal(v) => {v.hash(state)}
            Value::Boolean(v) => {v.hash(state)}
            Value::Set(v) => {v.hash(state)}
            Value::Array(v) => {v.hash(state)}
            Value::Object(v) => {v.hash(state)}
        }
    }
}
