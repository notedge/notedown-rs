use crate::{HTMLConfig, ToHTML};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    /// - `None`: It doesn't look like anything to me
    None,
    /// - ``
    Statements(Vec<AST>),

    /// - `Header`: TEXT, level, args
    Header(Box<AST>, u8, HashMap<String, String>),

    /// - `String`: Normal string with no style
    String(String),
    /// - `Bold`:
    Bold(Box<AST>, u8),
    /// - `Italic`:
    Italic(Box<AST>, u8),
    /// - `Underline`:
    Underline(Box<AST>, u8),
    /// - `Font`:
    Font(Box<AST>, HashMap<String, String>),

    /// - `Math`:
    Math(String, HashMap<String, String>),
    /// - `Code`:
    Code(String, HashMap<String, String>),

    /// - `Text`: For inline style
    Text(Box<AST>),
    /// -
    Word(String),
    Punctuation(String),
    Newline,
    ///  - `Paragraph`:
    Paragraph(Box<AST>),
    /// - `Function`: input, args, kvs
    Function(String, Vec<AST>, HashMap<String, String>),
}

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "unknown"),
        }
    }
}

impl From<&str> for AST {
    fn from(s: &str) -> Self {
        AST::String(s.to_string())
    }
}

/// Unwrap Box<AST>
impl ToHTML for Box<AST> {
    fn to_html(&self, cfg: HTMLConfig) -> String {
        let unbox = self.as_ref();
        unbox.to_html(cfg)
    }
}

impl ToHTML for AST {
    fn to_html(&self, cfg: HTMLConfig) -> String {
        macro_rules! unbox {
            ($e:ident) => {
                $e.to_html(cfg)
            };
        }
        match *self {
            AST::Header(ref e, ref level, ref kv) => format!("{} {}{:?}", unbox!(e), level, kv),
            AST::String(ref s) => format!("{}", s),
            AST::Bold(ref e, _) => format!("<b>{}</b>", unbox!(e)),
            AST::Italic(ref e, _) => format!("<i>{}</i>", unbox!(e)),
            AST::Font(ref e, ref kv) => {
                let mut tags = String::new();
                for (k, v) in kv.iter() {
                    tags += &format!(" {}=\"{}\"", k, v);
                }
                format!("<font{}>{}</font>", tags, unbox!(e))
            }
            AST::Underline(ref e, _) => format!("<u>{}</u>", unbox!(e)),
            _ => format!(""),
        }
    }
}
