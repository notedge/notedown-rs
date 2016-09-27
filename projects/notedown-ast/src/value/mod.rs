mod methods;
mod traits;
mod typing;

pub use self::typing::ValueType;
use crate::nodes::Literal;
use indexmap::{IndexMap, IndexSet};
use num::{BigInt, BigUint};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, BTreeSet};

/// Sparse representation of the array, the subscript can be any non-zero integer
pub type SparseArray = BTreeMap<BigUint, Literal<Value>>;
/// Ordered set of values
pub type OrderedSet = IndexSet<Literal<Value>>;
/// Ordered map of key value pairs
pub type OrderedMap = IndexMap<String, Literal<Value>>;

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
    Array(SparseArray),
    /// Ordered map of key value pairs
    Object(OrderedMap),
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
