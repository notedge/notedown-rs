use notedown_ast::Value;
use std::collections::HashMap;

/// https://ctan.org/topic/class
pub enum DocumentClass {
    Article(DocumentClassArticle),
}

pub struct DocumentClassArticle {
    font_size: usize,
}

impl DocumentClass {
    pub fn parse(&mut self) {}
}

impl DocumentClassArticle {
    pub fn parse(&mut self) {}
}
