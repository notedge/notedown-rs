#[cfg(feature = "lsp")]
mod lsp;
mod meta;

pub use self::meta::{
    author::{DocumentAuthor, DocumentAuthorIter},
    datetime::DocumentTime,
    toc::{TableOfContent, TocConfig, TocNode},
    DocumentMeta,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use notedown_ast::{
    value::{LiteralPair, OrderedMap},
    ASTNode, Value,
};
use notedown_error::{NoteError, Result};
use std::{collections::BTreeMap, fs::create_dir};
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
    pub fn set_title(&mut self, title: String) {
        self.meta.title = Some(title)
    }

    #[inline]
    pub fn set_value_raw(&mut self, pair: LiteralPair) -> Option<LiteralPair> {
        self.variable.insert_raw(pair)
    }
    #[inline]
    pub fn set_value(&mut self, name: String, value: Value) -> Option<Value> {
        self.variable.insert(name, value)
    }

    #[inline]
    pub fn get_ast(&self) -> &ASTNode {
        &self.context
    }
    #[inline]
    pub fn get_toc(&self) -> &TocNode {
        &self.meta.toc
    }

    #[inline]
    pub fn extend_error_one(&mut self, e: NoteError) {
        self.errors.push(e)
    }
    #[inline]
    pub fn extend_error_iter(&mut self, e: impl Iterator<Item = NoteError>) {
        self.errors.extend(e.into_iter())
    }

    #[inline]
    pub fn can_gc(&self) -> bool {
        false
    }
}
