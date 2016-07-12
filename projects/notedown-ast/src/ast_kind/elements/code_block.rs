use crate::ast_kind::*;

/// # Code Block
///
/// ## Code Inline
/// Code block in the body
/// ````note
/// text `code` text
/// ````
/// ## Code Block
/// Independent code block
/// ````note
/// ```lang
/// some code
/// following code
/// another code
/// ```
/// 
/// You can also add additional parameters
/// ```lang {
///     key = args
/// }
/// some code
/// following code
/// another code
/// ```
/// ````
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CodeHighlight {
    pub lang: String,
    pub code: String,
    pub inline: bool,
    pub high_line: Vec<usize>,
}

impl Display for CodeHighlight {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.inline {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = self.code)
        }
        else {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
        }
    }
}
