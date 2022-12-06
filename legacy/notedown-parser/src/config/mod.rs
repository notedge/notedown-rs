use notedown_pest::Span;
use std::ops::Range;
use url::Url;

pub struct NotedownParser {
    pub file_url: Option<Url>,
    pub tab_size: usize,
}

impl Default for NotedownParser {
    fn default() -> Self {
        Self { file_url: None, tab_size: 4 }
    }
}

impl NotedownParser {
    pub fn get_position(&self, s: Span) -> MaybeRanged {
        Some(Range { start: s.start(), end: s.end() })
    }
}
