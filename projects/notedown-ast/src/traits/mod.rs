mod arith;
mod context;
mod display;
mod slugify;
#[cfg(feature = "lsp-types")]
mod toc;

pub use context::ContextKind;
pub use toc::{TableNode, TableOfContent};

pub trait Slugify {
    fn slugify(&self) -> String;
}

pub trait ContextAware {
    fn context_aware(&self, offset: u32) -> ContextKind;
}
