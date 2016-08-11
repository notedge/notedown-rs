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
    lang: String,
    inline: bool,
    highlight: bool,
    code: String,
    show_file_name: Option<String>,
    /// None means not show line_number
    show_line_number: Option<usize>,
    highlight_lines: Vec<RangeInclusive<usize>>,
    hide_lines: Vec<RangeInclusive<usize>>,
}

impl Default for CodeNode {
    fn default() -> Self {
        Self {
            lang: String::from("text"),
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
            write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = self.lang, body = self.code)
        }
    }
}

impl CodeNode {
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::CodeNode(box self), range }
    }
    #[inline]
    pub fn set_language(mut self, lang: String) -> Self {
        self.lang = lang;
        return self;
    }

    #[inline]
    pub fn set_file_name(mut self, name: String) -> Self {
        self.show_file_name = Some(name);
        return self;
    }
    #[inline]
    pub fn clear_highlight_line(mut self) -> Self {
        self.highlight_lines.clear();
        return self;
    }
    #[inline]
    pub fn set_highlight_line(mut self, lines: Vec<RangeInclusive<usize>>) -> Self {
        self.highlight_lines = lines;
        return self;
    }

    #[inline]
    pub fn add_highlight_line(mut self, line: usize) -> Self {
        self.highlight_lines.push(RangeInclusive::new(line, line));
        return self;
    }
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
        Self { lang: String::from("text"), inline: true, highlight: false, code, ..Default::default() }
    }
    /// ```notedown
    /// `s`
    /// ```
    #[inline]
    pub fn code_block(lang: String, code: String) -> Self {
        Self { lang, inline: false, highlight: true, code, ..Default::default() }
    }
}
