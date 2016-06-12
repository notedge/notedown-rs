mod command;
mod link;

use std::fmt::{self, Debug, Display, Formatter};

pub use command::CommandKind;
pub use link::SmartLink;
use std::collections::HashMap;
pub use url::Url;

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
        args: Vec<AST>,
        kvs: HashMap<String, AST>,
        kind: CommandKind,
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

#[derive(Clone, Eq, PartialEq)]
pub struct TextRange {
    // pub index: u64,
    pub start: (u64, u64),
    pub end: (u64, u64),
}

impl Default for TextRange {
    fn default() -> Self {
        Self {
            // index: 0,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Default for AST {
    fn default() -> Self {
        Self::None
    }
}

impl Debug for TextRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}) → ({}, {})", self.start.0, self.start.1, self.end.0, self.end.1)
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
