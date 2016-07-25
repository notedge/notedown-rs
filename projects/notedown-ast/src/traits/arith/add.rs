use super::*;
use crate::NoteError;
use std::ops::{Mul, Shr, Sub};
use num::ToPrimitive;


impl Add for Value {
    type Output = Result<Self>;

    /// a + b
    fn add(self, other: Self) -> Self::Output {
        let out = match (self, other) {
            (Self::String(lhs), Self::String(rhs)) => {
                Self::String(lhs + &rhs)
            }
            (Self::Integer(lhs), Self::Integer(rhs)) => {
                Self::Integer(lhs + &rhs)
            }
            (Self::Integer(lhs), Self::Decimal(rhs)) | (Self::Decimal(lhs), Self::Integer(rhs)) => {
                match lhs.to_f64() {
                    Some(s) => { Self::Decimal(s + &rhs) }
                    None => { unimplemented!() }
                }
            }
            (Self::Decimal(lhs), Self::Decimal(rhs)) => {
                Self::Decimal(lhs + &rhs)
            }
            _ => return Err(NoteError::type_mismatch(format!("Can not apply `+` on lhs: {}, rhs: {}", self.get_type_name(), other.get_type_name())))
        };
        return Ok(out);
    }
}


impl Shr for Value {
    type Output = Result<Self>;

    /// a ++ b
    fn shr(self, other: Self) -> Self::Output {}
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