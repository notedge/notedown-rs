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
    lang: String,
    inline: bool,
    highlight: bool,
    code: String,
    file: Option<String>,
    high_line: Vec<usize>,
}

impl Default for CodeNode {
    fn default() -> Self {
        Self {
            lang: String::from("text"),
            code: String::new(),
            inline: false,
            highlight: false,
            file: None,
            high_line: vec![],
        }
    }
}

impl Display for CodeNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.inline {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = self.code)
        } else {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
        }
    }
}

impl CodeNode {
    #[inline]
    pub fn into_node(self, range: Option<(u32, u32)>) -> ASTNode {
        ASTNode { value: ASTKind::CodeNode(Box::new(self)), range }
    }
    #[inline]
    pub fn set_language(mut self, lang: String) -> Self {
        self.lang = lang;
        return self;
    }
    #[inline]
    pub fn set_highlight_line(mut self, lines: Vec<usize>) -> Self {
        self.high_line = lines;
        return self;
    }
    #[inline]
    pub fn set_file_name(mut self, name: String) -> Self {
        self.file = Some(name);
        return self;
    }
}

impl CodeNode {
    ///
    /// ```notedown
    /// `s`
    /// ```
    #[inline]
    pub fn code_inline(code: String) -> Self {
        Self {
            lang: String::from("text"),
            inline: true,
            highlight: false,
            code,
            file: None,
            high_line: vec![],
        }
    }
    ///
/// ```notedown
/// `s`
/// ```
    #[inline]
    pub fn code_block(lang: String, code: String) -> Self {
        Self {
            lang,
            inline: false,
            highlight: true,
            code,
            file: None,
            high_line: vec![],
        }
    }
}