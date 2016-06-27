use super::*;
use std::path::PathBuf;

pub trait CanParse {
    fn as_url(&self) -> Option<Url> {
        None
    }
    fn as_text(&self) -> ParserResult<String>;
}

impl CanParse for &str {
    fn as_text(&self) -> ParserResult<String> {
        Ok(self.to_string())
    }
}

impl CanParse for String {
    fn as_text(&self) -> ParserResult<String> {
        Ok(self.to_owned())
    }
}

impl CanParse for PathBuf {
    fn as_text(&self) -> ParserResult<String> {
        unimplemented!()
    }
}

#[cfg(any(unix, windows, target_os = "redox"))]
impl CanParse for Url {
    fn as_url(&self) -> Option<Url> {
        Some(self.to_owned())
    }

    fn as_text(&self) -> ParserResult<String> {
        match self.to_file_path() {
            Ok(o) => fs::read_to_string(o)?.as_text(),
            Err(_) => Err(FileNotFound(self.to_string().to_owned())),
        }
    }
}
