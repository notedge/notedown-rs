mod ordered_map;
mod sparse_array;

pub use ordered_map::*;

use super::*;

/// Ordered map of key value pairs
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct OrderedMap {
    inner: IndexMap<String, LiteralPair>,
}

/// Ordered map of key value pairs
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiteralPair {
    key: Literal<String>,
    value: Literal<Value>,
}

/// Sparse representation of the array, the subscript can be any non-zero integer
/// 1-index
#[derive(Clone, Default, Debug, Eq, PartialEq, Hash)]
pub struct SparseArray {
    default: Value,
    inner: BTreeMap<BigUint, Literal<Value>>,
}
