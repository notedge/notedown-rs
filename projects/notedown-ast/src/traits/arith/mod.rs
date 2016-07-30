mod add;
mod index;
mod mul;

use crate::{nodes::Value, NoteError, Result};
use num::{BigInt, ToPrimitive};
use std::ops::{Add, Div, Mul, Shr, Sub};

fn fail_int2dec(n: BigInt) -> Result<Value> {
    Err(NoteError::runtime_error(format!("Can not convert `Integer` {} to `Decimal`", n)))
}
