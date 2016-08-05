use crate::nodes::*;

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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct CodeNode {
    pub lang: String,
    pub inline: bool,
    pub highlight: bool,
    pub code: String,
    pub high_line: Vec<usize>,
}

impl Default for CodeNode {
    fn default() -> Self {
        Self {
            lang: String::from("text"),
            code: String::new(),
            inline: false,
            highlight: false,
            high_line: vec![],
        }
    }
}

impl Display for CodeNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.inline {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = self.code)
        }
        else {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
        }
    }
}

impl CodeNode {
    #[inline]
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: self, range }
    }
}