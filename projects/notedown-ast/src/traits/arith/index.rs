use super::*;
use num::Signed;
use std::ops::Neg;

pub trait Index<I> {
    type Output;
    fn get_index(&self, index: &I) -> Self::Output;
}

impl Index<BigInt> for Value {
    type Output = Result<Self>;

    fn get_index(&self, index: &BigInt) -> Self::Output {
        match self {
            Self::Null | Self::Boolean(_) => {
                unimplemented!()
                // &Err(NoteError::runtime_error(format!("Can not take `Integer` index of type `{}`", self.get_type_name())))
            }
            Self::Integer(_) => {
                unimplemented!()
            }
            Self::Decimal(_) => {
                unimplemented!()
            }
            Self::String(s) => {
                let i = if index.is_negative() {
                    match index.neg().to_usize() {
                        Some(u) => {
                            let max = s.chars().count();
                            // (u <= max).then_some(max);
                            if u <= max { Some(max) } else { None }
                        }
                        None => { None }
                    }
                } else {
                    index.to_usize()
                };

                match i.and_then(|e| s.chars().nth(e)) {
                    Some(s) => { Ok(Value::string(s)) }
                    None => { Err(NoteError::runtime_error(format!("Index `{}` of `String` out of range.", index))) }
                }
            }
            Self::Set(_) => {
                unimplemented!()
            }
            Self::Array(_) => {
                if index.is_negative() {}
                unimplemented!()
            }
            Self::Object(_) => {
                unimplemented!()
            }
        }
    }
}
