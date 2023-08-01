use std::{fmt::Display, ops::Range};

/// Marker trait for notedown node
pub trait NotedownNode: Display {
    /// Get the span of the node
    fn get_span(&self) -> Range<u32>;
}

// #![doc = include_str!("readme.md")]
// mod context;
// mod display;
// mod into_node;
// mod slugify;
//
// pub use self::{context::ContextKind, into_node::IntoNotedown};
//
// /// Slugify the element of notedown
// pub trait Slugify {
//     /// Slugify the element of notedown
//     fn slugify(&self) -> String;
// }
// /// Aware the context in which the cursor is located
// pub trait ContextAware {
//     /// Aware the context in which the cursor is located
//     fn context_aware(&self, offset: u32) -> ContextKind;
// }
