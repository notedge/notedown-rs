mod methods;
mod traits;
mod typing;

pub use self::typing::ValueType;
use crate::nodes::Literal;
use indexmap::{IndexMap, IndexSet};
use num::{BigInt, BigUint};
use std::collections::{BTreeMap, BTreeSet};

pub type Set = IndexSet<Literal<Value>>;
pub type Array = BTreeMap<BigUint, Literal<Value>>;
pub type Object = IndexMap<String, Literal<Value>>;

#[derive(Clone, Debug)]
pub enum Value {
    /// - `None`: It doesn't look like anything to me
    Null,
    Boolean(bool),
    Integer(BigInt),
    Decimal(f64),
    String(String),
    Set(Set),
    Array(Array),
    Object(Object),
}

impl Value {
    pub fn integer(value: impl Into<BigInt>) -> Self {
        Self::Integer(value.into())
    }
    pub fn decimal(value: impl Into<f64>) -> Self {
        Self::Decimal(value.into())
    }
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }
}
