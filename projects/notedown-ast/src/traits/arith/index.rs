use super::*;
use std::ops::Index;

impl Index<BigInt> for Value {
    type Output = Result<Self>;

    fn index(&self, index: BigInt) -> &Self::Output {
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
                let index = match index.to_usize() {
                    Some(s) => {}
                    None => {}
                };

                unimplemented!()
            }
            Self::Set(_) => {
                unimplemented!()
            }
            Self::Array(_) => {
                unimplemented!()
            }
            Self::Object(_) => {
                unimplemented!()
            }
        }
    }
}
