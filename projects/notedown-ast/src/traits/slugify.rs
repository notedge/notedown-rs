use crate::{ast::ASTKind, traits::Slugify, ASTNode};
use std::fmt::Debug;
pub use text_utils::slugify;

impl Slugify for Vec<ASTNode> {
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

impl<T: Debug> Slugify for ASTKind<T> {
    fn slugify(&self) -> String {
        match self {
            ASTKind::None => String::new(),
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
