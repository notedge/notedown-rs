use crate::{ast_kind::ASTKind, ast_node::ASTNode, traits::Slugify, ASTNodes};
use std::fmt::Debug;
pub use text_utils::slugify;

impl Slugify for ASTNodes {
    fn slugify(&self) -> String {
        let mut out = String::new();
        for span in self {
            if !out.is_empty() {
                out.push('-');
            }
            out.push_str(&span.slugify());
        }
        return out;
    }
}

impl Slugify for ASTNode {
    fn slugify(&self) -> String {
        self.kind.slugify()
    }
}

impl Slugify for ASTKind {
    fn slugify(&self) -> String {
        match self {
            ASTKind::Null => String::new(),
            ASTKind::Normal { 0: inner } => inner.to_owned().slugify(),
            _ => format!("Slugify: {:#?}", self),
        }
    }
}

impl Slugify for String {
    fn slugify(&self) -> String {
        slugify!(self)
    }
}

impl Slugify for &String {
    fn slugify(&self) -> String {
        slugify!(self)
    }
}

impl Slugify for &str {
    fn slugify(&self) -> String {
        slugify!(self)
    }
}
