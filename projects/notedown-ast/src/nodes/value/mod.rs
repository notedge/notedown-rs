mod methods;
mod typing;

pub use self::typing::ValueType;
use super::*;

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

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Null => 0_u8.hash(state),
            Self::Boolean(v) => v.hash(state),
            Self::Integer(v) => v.hash(state),
            Self::Decimal(v) => v.to_ne_bytes().hash(state),
            Self::String(v) => v.hash(state),
            Self::Set(v) => {
                v.len().hash(state);
                for e in v {
                    e.hash(state);
                }
            }
            Self::Array(v) => v.hash(state),
            Self::Object(v) => {
                v.len().hash(state);
                for e in v {
                    e.hash(state);
                }
            }
        }
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Boolean(l), Self::Boolean(r)) => l == r,
            (Self::Integer(l), Self::Integer(r)) => l == r,
            (Self::Decimal(l), Self::Decimal(r)) => l == r,
            (Self::String(l), Self::String(r)) => l == r,
            (Self::Set(l), Self::Set(r)) => l == r,
            (Self::Array(l), Self::Array(r)) => l == r,
            (Self::Object(l), Self::Object(r)) => l == r,
            _ => false,
        }
    }
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
