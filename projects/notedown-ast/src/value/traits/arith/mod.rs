mod add;
mod index;
mod mul;

use crate::Value;
use diagnostic_quick::{QError, QResult};
use num::{BigInt, Signed, ToPrimitive};
use std::ops::{Add, Div, Mul, Neg, Shr, Sub};

fn fail_int2dec(n: BigInt) -> QResult<Value> {
    Err(QError::runtime_error(format!("Can not convert `Integer` {} to `Decimal`", n)))
}
