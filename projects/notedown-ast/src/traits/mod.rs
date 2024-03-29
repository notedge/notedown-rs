mod context;
mod display;
mod into_node;
mod slugify;
mod toc;

pub use self::{
    context::ContextKind,
    into_node::IntoASTNode,
    toc::{TableOfContent, TocConfig, TocNode},
};

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
