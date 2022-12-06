#![doc = include_str!("readme.md")]
mod collections;
mod methods;
mod traits;
mod typing;

pub use self::{collections::*, typing::ValueType};
use indexmap::{IndexMap, IndexSet};
use num::{BigInt, BigUint};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, BTreeSet};

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    /// It doesn't look like anything to me
    Null,
    /// `true` or `false`
    Boolean(bool),
    /// Arbitrarily large integer
    Integer(BigInt),
    /// 128-bit fixed point decimal, enough for financial calculations
    Decimal(Decimal),
    /// A UTF-8–encoded string
    String(String),
    /// Ordered set of values
    Set(OrderedSet),
    /// Array of values
    Array(Box<SparseArray>),
    /// Ordered map of key value pairs
    Object(Box<OrderedMap>),
}

impl Default for Value {
    fn default() -> Self {
        Self::Null
    }
}

impl Value {
    /// convert a integer to value
    pub fn integer(value: impl Into<BigInt>) -> Self {
        Self::Integer(value.into())
    }
    /// convert a decimal to value
    pub fn decimal(value: impl Into<Decimal>) -> Self {
        Self::Decimal(value.into())
    }
    /// convert a string to value
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }
}
