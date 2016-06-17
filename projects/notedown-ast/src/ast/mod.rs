mod command;
mod link;
mod range;
mod literal;

pub use command::CommandKind;
pub use link::SmartLink;
pub use range::TextRange;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    None,
    Statements,
    Header {
        level: usize,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CST {
    Node {
        kind: ASTKind,
        children: Vec<CST>,
        r: TextRange,
    },
    Leaf {
        kind: ASTKind,
        r: TextRange,
    },
}

impl CST {
    pub fn node() {

    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    Statements(Vec<AST>),
    // Blocks
    /// - `Header`: TEXT, level
    Header {
        level: usize,
        children: Vec<AST>,
        r: TextRange,
    },
    HorizontalRule {
        r: TextRange,
    },
    ///  - `Paragraph`:
    Paragraph {
        children: Vec<AST>,
        r: TextRange,
    },
    Highlight {
        lang: String,
        code: String,
        inline: bool,
        high_line: Vec<usize>,
        r: TextRange,
    },
    /// - `Math`:
    MathBlock {
        inner: String,
        r: TextRange,
    },
    TableView {
        head: Vec<AST>,
        align: Vec<Option<bool>>,
        terms: Vec<Vec<AST>>,
        column: usize,
        r: TextRange,
    },
    QuoteList {
        style: Option<String>,
        body: Vec<AST>,
        r: TextRange,
    },
    OrderedList {
        head: usize,
        body: Vec<AST>,
        r: TextRange,
    },
    OrderlessList {
        body: Vec<AST>,
        r: TextRange,
    },
    /// - `Code`:
    // inlined
    Normal {
        inner: String,
        r: TextRange,
    },
    Raw {
        inner: String,
        r: TextRange,
    },
    /// `` `code` ``
    Code {
        inner: String,
        r: TextRange,
    },
    Italic {
        children: Vec<AST>,
        r: TextRange,
    },
    Bold {
        children: Vec<AST>,
        r: TextRange,
    },
    Emphasis {
        children: Vec<AST>,
        r: TextRange,
    },
    Underline {
        children: Vec<AST>,
        r: TextRange,
    },
    Strikethrough {
        children: Vec<AST>,
        r: TextRange,
    },
    Undercover {
        children: Vec<AST>,
        r: TextRange,
    },

    MathInline {
        inner: String,
        r: TextRange,
    },
    MathDisplay {
        inner: String,
        r: TextRange,
    },

    Link {
        inner: SmartLink,
        r: TextRange,
    },

    Escaped {
        inner: char,
        r: TextRange,
    },
    //
    Command {
        cmd: String,
        kind: CommandKind,
        args: Vec<AST>,
        kvs: HashMap<String, AST>,
        r: TextRange,
    },
    String {
        inner: String,
        r: TextRange,
    },
    Integer {
        inner: String,
        r: TextRange,
    },
    Decimal {
        inner: String,
        r: TextRange,
    },
    Boolean {
        inner: bool,
        r: TextRange,
    },
    Array {
        inner: Vec<AST>,
        r: TextRange,
    },
}

impl Default for AST {
    fn default() -> Self {
        Self::None
    }
}

impl AST {
    pub fn to_vec(&self) -> Vec<AST> {
        match self {
            AST::Statements(v) => v.to_owned(),
            AST::Paragraph { children, .. } => children.to_owned(),
            _ => vec![],
        }
    }
}
