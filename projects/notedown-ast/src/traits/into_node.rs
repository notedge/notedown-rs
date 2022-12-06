use diagnostic_quick::{FileID, Span};

use crate::{nodes::*, Command, NotedownKind, NotedownNode, Value};

/// Convert element into [`NotedownNode`]
pub trait IntoNotedown {
    /// Convert element into [`NotedownNode`] with position
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode;
}

impl Into<NotedownNode> for NotedownKind {
    #[inline]
    fn into(self) -> NotedownNode {
        self.into_node(&Span::default(), &FileID::default())
    }
}

impl IntoNotedown for NotedownKind {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        NotedownNode { value: self, range: span.clone(), file: file.clone() }
    }
}
