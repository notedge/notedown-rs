use crate::{
    nodes::{ASTKind, ASTNode, ASTNodes, Header},
    traits::Slugify,
};
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
        self.value.slugify()
    }
}

impl Slugify for ASTKind {
    fn slugify(&self) -> String {
        match self {
            _ => format!("Slugify: {:#?}", self),
        }
    }
}

impl Slugify for Header {
    fn slugify(&self) -> String {
        todo!()
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
