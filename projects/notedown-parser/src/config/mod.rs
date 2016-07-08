use notedown_ast::utils::{Range, Position};
use notedown_pest::Span;
use url::{Url};

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
    pub fn get_position(&self, s: Span) -> Range {
        let us = s.start_pos().line_col();
        let es = s.end_pos().line_col();
        Range {
            // index: s.start_pos().pos() as u64,
            start: Position {
                line: us.0 as u32,
                character: us.1 as u32
            },
            end: Position {
                line: es.0 as u32,
                character: es.1 as u32
            },
        }
    }
}
