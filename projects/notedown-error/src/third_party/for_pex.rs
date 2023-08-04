use crate::NoteError;
use pex::StopBecause;

impl From<StopBecause> for NoteError {
    fn from(value: StopBecause) -> Self {
        todo!()
    }
}
