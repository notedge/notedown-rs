use notedown_pest::Span;
use std::ops::Range;
use url::Url;

pub struct ParserConfig {
    pub file_url: Option<Url>,
    pub tab_size: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self { file_url: None, tab_size: 4 }
    }
}

impl ParserConfig {
    pub fn get_position(&self, s: Span) -> Option<Range<usize>> {
        Some(Range { start: s.start(), end: s.end() })
    }
}
