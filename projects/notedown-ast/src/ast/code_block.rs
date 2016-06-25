use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CodeBlock {
   pub lang: String,
    pub code: String,
    pub inline: bool,
    pub  high_line: Vec<usize>,
}

impl Display for CodeBlock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.inline {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = self.code)
        }
        else {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
        }
    }
}
