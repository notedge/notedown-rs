mod ordered_map;

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
