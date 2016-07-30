mod add;
mod mul;
mod index;

use crate::{nodes::Value, Result};
use std::ops::{Add};
use crate::NoteError;
use num::ToPrimitive;
use std::ops::{Mul, Shr, Sub};
use std::ops::{Div};
use num::BigInt;

fn fail_int2dec(n: BigInt) -> Result<Value> {
    Err(NoteError::runtime_error(format!("Can not convert `Integer` {} to `Decimal`", n)))
}


