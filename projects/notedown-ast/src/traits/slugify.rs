use crate::AST;
use crate::traits::Slugify;
pub use text_utils::slugify;

impl Slugify for Vec<AST> {
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

impl Slugify for AST {
    fn slugify(&self) -> String {
        match self {
            AST::None => String::new(),
            AST::Statements(_) => unimplemented!(),
            AST::Header { .. } => unimplemented!(),
            AST::HorizontalRule { .. } => unimplemented!(),
            AST::Paragraph { .. } => unimplemented!(),
            AST::Highlight { .. } => unimplemented!(),
            AST::MathBlock { .. } => unimplemented!(),
            AST::TableView { .. } => unimplemented!(),
            AST::QuoteList { .. } => unimplemented!(),
            AST::OrderedList { .. } => unimplemented!(),
            AST::OrderlessList { .. } => unimplemented!(),
            AST::Normal { inner, .. } => inner.to_owned().slugify(),
            AST::Raw { .. } => unimplemented!(),
            AST::Code { .. } => unimplemented!(),
            AST::Italic { .. } => unimplemented!(),
            AST::Bold { .. } => unimplemented!(),
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
