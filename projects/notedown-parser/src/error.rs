use notedown_pest::Rule;

#[derive(Debug)]
pub enum Error {
    LexerError(String),
    FileNotFound(String),
    IOError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<notedown_pest::Error<Rule>> for Error {
    fn from(e: notedown_pest::Error<Rule>) -> Self {
        Error::LexerError(format!("{}", e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{:?}", e))
    }
}
