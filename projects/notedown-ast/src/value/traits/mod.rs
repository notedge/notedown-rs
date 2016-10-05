mod arith;
mod convert;

use super::*;
use std::hash::{Hash, Hasher};

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Null => 0_u8.hash(state),
            Self::Boolean(v) => v.hash(state),
            Self::Integer(v) => v.hash(state),
            Self::Decimal(v) => v.hash(state),
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
