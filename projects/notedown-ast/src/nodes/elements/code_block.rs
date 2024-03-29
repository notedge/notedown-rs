use crate::nodes::*;
use std::ops::RangeInclusive;

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
/// // You can also add additional parameters
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
    ///
    pub inline: bool,
    ///
    pub highlight: bool,
    ///
    pub language: String,
    ///
    pub code: String,
    ///
    pub show_file_name: Option<String>,
    /// None means not show line_number
    /// usize means starts with n
    pub show_line_number: Option<usize>,
    ///
    pub highlight_lines: Vec<RangeInclusive<usize>>,
    ///
    pub hide_lines: Vec<RangeInclusive<usize>>,
}

impl Default for CodeNode {
    fn default() -> Self {
        Self {
            language: String::from("text"),
            code: String::new(),
            inline: false,
            highlight: false,
            show_file_name: None,
            show_line_number: None,
            highlight_lines: vec![],
            hide_lines: vec![],
        }
    }
}

impl Display for CodeNode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.inline {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = self.code)
        }
        else {
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.language, body = self.code)
        }
    }
}

impl CodeNode {
    /// TODO: doc
    #[inline]
    pub fn set_file_name(mut self, name: String) -> Self {
        self.show_file_name = Some(name);
        return self;
    }
    /// TODO: doc
    #[inline]
    pub fn clear_highlight_line(mut self) -> Self {
        self.highlight_lines.clear();
        return self;
    }
    /// TODO: doc
    #[inline]
    pub fn set_highlight_line(mut self, lines: Vec<RangeInclusive<usize>>) -> Self {
        self.highlight_lines = lines;
        return self;
    }
    /// TODO: doc
    #[inline]
    pub fn add_highlight_line(mut self, line: usize) -> Self {
        self.highlight_lines.push(RangeInclusive::new(line, line));
        return self;
    }
    /// TODO: doc
    #[inline]
    pub fn add_highlight_range(mut self, lines: RangeInclusive<usize>) -> Self {
        self.highlight_lines.push(lines);
        return self;
    }
}

impl CodeNode {
    /// ```notedown
    /// `s`
    /// ```
    #[inline]
    pub fn code_inline(code: String) -> Self {
        Self { language: String::from("text"), inline: true, highlight: false, code, ..Default::default() }
    }
    /// ```notedown
    /// `s`
    /// ```
    #[inline]
    pub fn code_block(lang: String, code: String) -> Self {
        Self { language: lang, inline: false, highlight: true, code, ..Default::default() }
    }
}

impl ASTKind {
    /// TODO: doc
    #[inline]
    pub fn code_inline(code: impl Into<String>, range: MaybeRanged) -> ASTNode {
        CodeNode::code_inline(code.into()).into_node(range)
    }
    /// TODO: doc
    #[inline]
    pub fn code_block(code: impl Into<String>, language: impl Into<String>, range: MaybeRanged) -> ASTNode {
        CodeNode::code_block(language.into(), code.into()).into_node(range)
    }
}
