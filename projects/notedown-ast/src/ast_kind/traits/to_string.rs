use crate::{ast_kind::ASTKind, utils::join_ast_list, ASTNode};
use std::fmt::{self, Debug, Display, Formatter};

impl<M: Debug + Display> Display for ASTNode<M> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ASTKind::Null => write!(f, ""),
            ASTKind::Statements(children) => {
                let s: Vec<_> = children.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{}", s.join("\n\n"))
            }
            ASTKind::Header { .. } => unimplemented!(),
            ASTKind::HorizontalRule { .. } => unimplemented!(),
            ASTKind::Paragraph { .. } => unimplemented!(),
            ASTKind::CodeBlock(inner) => Display::fmt(inner, f),
            ASTKind::MathBlock { .. } => unimplemented!(),
            ASTKind::TableView { .. } => unimplemented!(),
            ASTKind::ListView(inner) => Display::fmt(inner, f),
            ASTKind::Normal(inner) => write!(f, "{}", inner),
            ASTKind::Raw { .. } => unimplemented!(),
            ASTKind::Code { .. } => unimplemented!(),
            ASTKind::Italic { .. } => unimplemented!(),
            ASTKind::Bold(children) => write!(f, "**{}**", join_ast_list(&children)),
            ASTKind::Emphasis { .. } => unimplemented!(),
            ASTKind::Underline { .. } => unimplemented!(),
            ASTKind::Strikethrough { .. } => unimplemented!(),
            ASTKind::Undercover { .. } => unimplemented!(),
            ASTKind::MathInline { .. } => unimplemented!(),
            ASTKind::MathDisplay { .. } => unimplemented!(),
            ASTKind::Link { .. } => unimplemented!(),
            ASTKind::Escaped { .. } => unimplemented!(),
            ASTKind::Command { .. } => unimplemented!(),
            ASTKind::String { .. } => unimplemented!(),
            ASTKind::Number { .. } => unimplemented!(),
            ASTKind::Boolean { .. } => unimplemented!(),
            ASTKind::Array { .. } => unimplemented!(),
            ASTKind::Object => unimplemented!(),
        }
    }
}

// impl Display for AST {
// fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// match self {
// ASTKind::None => write!(f, ""),
// ASTKind::Newline { .. } => write!(f, "\n"),
// ASTKind::Header { children: c, level: l, .. } => write!(f, "{} {}", "#".repeat(*l), join_span(c)),
// ASTKind::Statements { children: e, .. } => {
// let fs: Vec<_> = e.iter().map(|ast| format!("{}", ast)).collect();
// write!(f, "{}", fs.join("\n\n"))
// }
//
// ASTKind::Paragraph { children: span, .. } => write!(f, "{}", join_span(span)),
//
// ASTKind::Raw { inner, .. } => write!(f, "{}", inner),
// ASTKind::Code { inner, .. } => write!(f, "`{}`", inner),
// ASTKind::Normal { inner, .. } => write!(f, "{}", inner),
// ASTKind::Emphasis { children: s, .. } => write!(f, "*{}*", join_span(s)),
// ASTKind::Strong { children: s, .. } => write!(f, "**{}**", join_span(s)),
// ASTKind::Underline { children: s, .. } => write!(f, "~{}~", join_span(s)),
// ASTKind::Strikethrough { children: s, .. } => write!(f, "~~{}~~", join_span(s)),
// ASTKind::Undercover { children: s, .. } => write!(f, "~~~{}~~~", join_span(s)),
//
// ASTKind::MathInline { inner, .. } => write!(f, "${}$", inner),
// ASTKind::MathDisplay { inner, .. } => write!(f, "$${}$$", inner),
// ASTKind::MathBlock { inner, .. } => write!(f, "$${}$$", inner),
//
// ASTKind::Link { inner: link, .. } => write!(f, "{}", link),
// ASTKind::List { inner: list, .. } => write!(f, "{}", list),
// ASTKind::Table { inner: table, .. } => write!(f, "{}", table),
// ASTKind::Highlight { inner: code, .. } => write!(f, "{}", code),
// ASTKind::Command { inner: cmd, .. } => write!(f, "{}", cmd),
//
// ASTKind::Escaped { inner: c, .. } => write!(f, "{}", c),
// }
// }
// }
//
// fn join_span(v: &[AST]) -> String {
// let s: Vec<String> = v.iter().map(|k| format!("{}", k)).collect();
// s.join("")
// }
