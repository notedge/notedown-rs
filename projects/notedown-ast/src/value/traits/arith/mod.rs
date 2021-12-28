mod add;
mod index;
mod mul;

use crate::Value;
use notedown_error::{NoteError, Result};
use num::{BigInt, Signed, ToPrimitive};
use std::ops::{Add, Div, Mul, Neg, Shr, Sub};

fn fail_int2dec(n: BigInt) -> Result<Value> {
    Err(NoteError::runtime_error(format!("Can not convert `Integer` {} to `Decimal`", n)))
}
