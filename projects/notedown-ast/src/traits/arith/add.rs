use super::*;
use crate::NoteError;
use num::ToPrimitive;
use std::ops::{Mul, Shr, Sub};

impl Add for Value {
    type Output = Result<Self>;

    /// a + b
    fn add(self, other: Self) -> Self::Output {
        let type_mismatch = Err(NoteError::type_mismatch(format!(
            "Can not apply `+` on lhs: {}, rhs: {}",
            self.get_type_name(),
            other.get_type_name()
        )));

        let out = match (self, other) {
            (Self::String(lhs), Self::String(rhs)) => Self::String(lhs + &rhs),
            (Self::Integer(lhs), Self::Integer(rhs)) => Self::Integer(lhs + &rhs),
            (Self::Integer(lhs), Self::Decimal(rhs)) | (Self::Decimal(rhs), Self::Integer(lhs)) => match lhs.to_f64() {
                Some(s) => Self::Decimal(s + &rhs),
                None => {
                    return Err(NoteError::runtime_error(format!("Can not convert `Integer` {} to `Decimal`", lhs)));
                }
            },
            (Self::Decimal(lhs), Self::Decimal(rhs)) => Self::Decimal(lhs + &rhs),
            _ => {
                return type_mismatch;
            }
        };
        return Ok(out);
    }
}

impl Shr for Value {
    type Output = Result<Self>;

    /// a ++ b
    fn shr(self, other: Self) -> Self::Output {
        fn type_mismatch(l: String, r: String) -> Result<Value> {
            Err(NoteError::type_mismatch(format!(
                "Can not apply `++` on lhs: {}, rhs: {}",
                l,
                r
            )))
        }
        let error = type_mismatch(
            self.get_type_name(),
            other.get_type_name(),
        );
        let out = match (self, other) {
            (Self::String(lhs), rhs) => Self::string_join(lhs, rhs)?,
            _ => {
                return error;
            }
        };
        return Ok(out);
    }
}


impl Value {
    pub fn string_join(lhs: String, other: Value) -> Result<Self> {
        fn type_mismatch(r: String) -> Result<Value> {
            Err(NoteError::type_mismatch(format!(
                "Can not apply `++` on lhs: `String`, rhs: {}",
                r
            )))
        }
        let out = match &other {
            Value::Null => { lhs }
            Value::Boolean(v) => { format!("{}{}", lhs, v) }
            Value::Integer(v) => { format!("{}{}", lhs, v) }
            Value::Decimal(v) => { format!("{}{}", lhs, v) }
            Value::String(v) => { format!("{}{}", lhs, v) }
            Value::Set(_) => {
                return type_mismatch(other.get_type_name());
            }
            Value::Array(_) => { return type_mismatch(other.get_type_name()); }
            Value::Object(_) => { return type_mismatch(other.get_type_name()); }
        };
        Ok(Value::String(out))
    }
}

impl Sub for Value {
    type Output = Result<Self>;
    /// a- b
    fn sub(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Value {
    type Output = Result<Self>;
    // a -- b
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
