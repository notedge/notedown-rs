use crate::Value;
use notedown_error::NoteError;
use std::convert::TryFrom;

macro_rules! from_value {
    ($source:tt => $target:ty) => {
        impl TryFrom<Value> for $target {
            type Error = NoteError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    Value::$source(v) => Ok(Self::try_from(v)?),
                    _ => Err(NoteError::unreachable()),
                }
            }
        }
    };
    ($($source:tt => $target:ty),+ $(,)?) => (
        $(from_value!($source=> $target);)+
    );
}

from_value! [
    Boolean => bool,
    Decimal => f64,
    Integer => u8,
    Integer => u16,
    Integer => u32,
    Integer => u64,
    Integer => u128,
    Integer => usize,
    Integer => i8,
    Integer => i16,
    Integer => i32,
    Integer => i64,
    Integer => i128,
    Integer => isize,
    String => String,
];
