mod context;
mod display;
mod slugify;
mod toc;

pub use context::ContextKind;
pub use toc::{TableOfContent, TocNode};

/// Slugify the element of notedown
pub trait Slugify {
    /// Slugify the element of notedown
    fn slugify(&self) -> String;
}
/// Aware the context in which the cursor is located
pub trait ContextAware {
    /// Aware the context in which the cursor is located
    fn context_aware(&self, offset: u32) -> ContextKind;
}
