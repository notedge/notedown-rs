#[cfg(feature = "lsp")]
mod lsp;
mod meta;

pub use self::meta::DocumentMeta;
use chrono::NaiveDateTime;
use notedown_ast::{
    traits::{TableOfContent, TocConfig, TocNode},
    value::{LiteralPair, OrderedMap},
    ASTNode, Value,
};
use notedown_error::{NoteError, Result};
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DocumentSymbolResponse},
    TextIndex,
};

pub struct NoteDocument {
    context: ASTNode,
    meta: DocumentMeta,
    variable: OrderedMap,
    errors: Vec<NoteError>,
}

impl NoteDocument {
    #[inline]
    pub fn get_date(&self) -> &Option<NaiveDateTime> {
        &self.meta.date
    }
    #[inline]
    pub fn set_date(&mut self, date: NaiveDateTime) {
        self.meta.date = Some(date);
    }
    #[inline]
    pub fn set_date_unix(&mut self, date: i64) {
        let date = NaiveDateTime::from_timestamp(date, 0);
        self.meta.date = Some(date)
    }
    #[inline]
    pub fn set_date_fmt(&mut self, date: &str, fmt: &str) -> Result<()> {
        let date = NaiveDateTime::parse_from_str(date, fmt)?;
        Ok(self.meta.date = Some(date))
    }
    #[inline]
    pub fn set_title(&mut self, title: String) {
        self.meta.title = Some(title)
    }

    #[inline]
    pub fn get_author(&self) -> &Vec<String> {
        &self.meta.author
    }
    #[inline]
    pub fn set_author(&mut self, authors: Vec<String>) {
        self.meta.author = authors
    }
    #[inline]
    pub fn add_author(&mut self, author: String) {
        self.meta.author.push(author)
    }
    #[inline]
    pub fn set_value_raw(&mut self, pair: LiteralPair) -> Option<LiteralPair> {
        self.variable.insert_raw(pair)
    }
    #[inline]
    pub fn set_value(&mut self, name: String, value: Value) -> Option<Value> {
        self.variable.insert(name, value)
    }
}
