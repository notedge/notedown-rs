mod arith;
mod display;
mod slugify;
mod context;
#[cfg(feature = "lsp-types")]
mod toc;

pub use context::ContextKind;
pub use toc::{TableOfContent, TableNode};

pub trait Slugify {
    fn slugify(&self) -> String;
}

pub trait ContextAware {
    fn context_aware(&self, offset: u32) -> ContextKind;
}