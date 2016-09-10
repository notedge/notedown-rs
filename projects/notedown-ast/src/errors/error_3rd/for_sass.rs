use crate::{utils::Url, NoteError};
use rsass::Error;

impl From<Error> for NoteError {
    fn from(e: Error) -> Self {
        match e {
            Error::Input(path, io) => {
                let mut error = Self::from(io);
                match Url::from_file_path(path) {
                    Ok(o) => error.file = Some(o),
                    Err(_) => {}
                }
                error
            }
            Error::IoError(io) => Self::from(io),
            Error::BadCall(_, _, _) => {
                unimplemented!()
            }
            Error::InvalidFunctionName(_) => {
                unimplemented!()
            }
            Error::BadValue(_) => {
                unimplemented!()
            }
            Error::BadArgument(_, _) => {
                unimplemented!()
            }
            Error::BadArguments(_, _) => {
                unimplemented!()
            }
            Error::BadRange(_) => {
                unimplemented!()
            }
            Error::ParseError(_) => {
                unimplemented!()
            }
            Error::UndefinedVariable(_) => {
                unimplemented!()
            }
            Error::AtError(_, _) => {
                unimplemented!()
            }
            Error::S(_) => {
                unimplemented!()
            }
        }
    }
}
