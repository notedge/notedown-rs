mod methods;
mod typing;

use super::*;
pub use self::typing::ValueType;

#[derive(Clone, Debug)]
pub enum Value {
    /// - `None`: It doesn't look like anything to me
    Null,
    Boolean(bool),
    Integer(BigInt),
    Decimal(f64),
    String(String),
    Set(Set<Literal<Value>>),
    Array(Map<Literal<BigUint>, Literal<Value>>),
    Object(Map<Literal<String>, Literal<Value>>),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Null => ().hash(state),
            Self::Boolean(v) => v.hash(state),
            Self::Integer(v) => v.hash(state),
            Self::Decimal(v) => unsafe { transmute::<f64, [u8; 8]>(*v).hash(state) },
            Self::String(v) => v.hash(state),
            Self::Set(v) => v.hash(state),
            Self::Array(v) => v.hash(state),
            Self::Object(v) => v.hash(state),
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
