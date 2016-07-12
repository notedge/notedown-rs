use super::*;
use std::fmt::{self, Display, Formatter, Debug};


impl Display for PrimitiveValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveValue::Null => { f.write_str("null") }
            PrimitiveValue::Boolean(v) => { f.write_str(&v.to_string()) }
            PrimitiveValue::Integer(v) => { f.write_str(&v.to_string()) }
            PrimitiveValue::Decimal(v) => {
                unsafe {
                    f.write_str(&transmute::<[u8; 8], f64>(*v).to_string())
                }
            }
            PrimitiveValue::String(v) => { write!(f, "{:?}", v) }
        }
    }
}

impl<T> Display for Literal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.value, f)
    }
}