use diagnostic_quick::{FileID, Span};

use crate::{
    nodes::{EmailLink, ImageLink, SmartLink},
    NotedownKind, NotedownNode,
};

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

impl IntoNotedown for SmartLink {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        NotedownKind::LinkNode(self).into_node(span, file)
    }
}

impl IntoNotedown for ImageLink {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        SmartLink::Image(Box::new(self)).into_node(span, file)
    }
}

impl IntoNotedown for EmailLink {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        SmartLink::Image(Box::new(self)).into_node(span, file)
    }
}

macro_rules! impl_into_notedown {
    ($($t:ty),*) => {
        $(
            impl IntoNotedown for $t {
                #[inline]
                fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
                    NotedownKind::from(self).into_node(span, file)
                }
            }
        )*
    };
}
