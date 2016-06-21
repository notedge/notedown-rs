use super::*;
use std::path::PathBuf;

pub trait ParseText {
    fn parse_text(&self) -> ParserResult<Pairs<Rule>>;
}

impl ParseText for &str {
    fn parse_text(&self) -> ParserResult<Pairs<Rule>> {
        let input = self.replace("\r\n", "\n").replace("\\\n", "");
        let pairs = NoteDownParser::parse(Rule::program, &input)?;
        return Ok(pairs);
    }
}

impl ParseText for String {
    fn parse_text(&self) -> ParserResult<Pairs<Rule>> {
        self.parse_text()
    }
}

impl ParseText for PathBuf {
    fn parse_text(&self) -> ParserResult<Pairs<Rule>> {
        fs::read_to_string(o)?.parse_text()
    }
}

impl ParseText for Url {
    fn parse_text(&self) -> ParserResult<Pairs<Rule>> {
        match self.to_file_path() {
            Ok(o) => o.parse_text(),
            Err(_) => Err(FileNotFound(self.to_string().to_owned())),
        }
    }
}
