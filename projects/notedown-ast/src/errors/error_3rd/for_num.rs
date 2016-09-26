use crate::NoteError;
use num::{bigint::TryFromBigIntError, BigInt};

impl From<TryFromBigIntError<BigInt>> for NoteError {
    fn from(e: TryFromBigIntError<BigInt>) -> Self {
        Self::runtime_error(format!("{}", e))
    }
}
