use crate::AST;
use std::fmt::{self, Display, Formatter};
use crate::utils::join_ast_list;

impl Display for AST {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            AST::None => write!(f, ""),
            AST::Statements(v) => {
                let s: Vec<_> = v.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{}", s.join("\n\n"))
            }
            AST::Header { .. } => unimplemented!(),
            AST::HorizontalRule { .. } => unimplemented!(),
            AST::Paragraph { .. } => unimplemented!(),
            AST::Highlight { lang, code, inline, .. } => {
                if *inline {
                    write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`", lang = "", body = code)
                }
                else {
                    write!(f, "{mark}{lang}\n{body}\n{mark}", mark = "`".repeat(3), lang = lang, body = code)
                }
            }
            AST::MathBlock { .. } => unimplemented!(),
            AST::TableView { .. } => unimplemented!(),
            AST::QuoteList { .. } => unimplemented!(),
            AST::OrderedList { .. } => unimplemented!(),
            AST::OrderlessList { .. } => unimplemented!(),
            AST::Normal { inner, .. } => write!(f, "{}", inner),
            AST::Raw { .. } => unimplemented!(),
            AST::Code { .. } => unimplemented!(),
            AST::Italic { .. } => unimplemented!(),
            AST::Bold { children, .. } => write!(f, "**{}**", join_ast_list(children)),
            AST::Emphasis { .. } => unimplemented!(),
            AST::Underline { .. } => unimplemented!(),
            AST::Strikethrough { .. } => unimplemented!(),
            AST::Undercover { .. } => unimplemented!(),
            AST::MathInline { .. } => unimplemented!(),
            AST::MathDisplay { .. } => unimplemented!(),
            AST::Link { .. } => unimplemented!(),
            AST::Escaped { .. } => unimplemented!(),
            AST::Command { .. } => unimplemented!(),
            AST::String { .. } => unimplemented!(),
            AST::Integer { .. } => unimplemented!(),
            AST::Decimal { .. } => unimplemented!(),
            AST::Boolean { .. } => unimplemented!(),
            AST::Array { .. } => unimplemented!(),
        }
    }
}

// impl Display for AST {
// fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// match self {
// AST::None => write!(f, ""),
// AST::Newline { .. } => write!(f, "\n"),
// AST::Header { children: c, level: l, .. } => write!(f, "{} {}", "#".repeat(*l), join_span(c)),
// AST::Statements { children: e, .. } => {
// let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
// write!(f, "{}", fs.join("\n\n"))
// }
//
// AST::Paragraph { children: span, .. } => write!(f, "{}", join_span(span)),
//
// AST::Raw { inner, .. } => write!(f, "{}", inner),
// AST::Code { inner, .. } => write!(f, "`{}`", inner),
// AST::Normal { inner, .. } => write!(f, "{}", inner),
// AST::Emphasis { children: s, .. } => write!(f, "*{}*", join_span(s)),
// AST::Strong { children: s, .. } => write!(f, "**{}**", join_span(s)),
// AST::Underline { children: s, .. } => write!(f, "~{}~", join_span(s)),
// AST::Strikethrough { children: s, .. } => write!(f, "~~{}~~", join_span(s)),
// AST::Undercover { children: s, .. } => write!(f, "~~~{}~~~", join_span(s)),
//
// AST::MathInline { inner, .. } => write!(f, "${}$", inner),
// AST::MathDisplay { inner, .. } => write!(f, "$${}$$", inner),
// AST::MathBlock { inner, .. } => write!(f, "$${}$$", inner),
//
// AST::Link { inner: link, .. } => write!(f, "{}", link),
// AST::List { inner: list, .. } => write!(f, "{}", list),
// AST::Table { inner: table, .. } => write!(f, "{}", table),
// AST::Highlight { inner: code, .. } => write!(f, "{}", code),
// AST::Command { inner: cmd, .. } => write!(f, "{}", cmd),
//
// AST::Escaped { inner: c, .. } => write!(f, "{}", c),
// }
// }
// }
//
// fn join_span(v: &[AST]) -> String {
// let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
// s.join("")
// }
