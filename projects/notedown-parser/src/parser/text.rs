use super::*;
use std::path::PathBuf;
use std::mem;

pub trait CanParse {
    fn as_url(&self) -> Option<Url> {
        None
    }
    fn as_text(&self) -> ParserResult<&str>;
}

impl CanParse for &str {
    fn as_text(&self) -> ParserResult<&str> {
        Ok(self)
    }
}



impl CanParse for String {
    fn as_text(&self) -> ParserResult<&str> {
        Ok(self.as_str())
    }
}


impl CanParse for PathBuf {
    fn as_text(&self) -> ParserResult<&str> {
        unimplemented!()
    }
}

impl CanParse for Url {
    fn as_url(&self) -> Option<Url> {
        Some(self.to_owned())
    }

    fn as_text(&self) -> ParserResult<&str> {
        match self.to_file_path() {
            Ok(o) => fs::read_to_string(o)?.as_text(),
            Err(_) => Err(FileNotFound(self.to_string().to_owned())),
        }
    }
}