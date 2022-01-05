#[cfg(feature = "lsp")]
pub(crate) mod lsp;
pub(crate) mod meta;

pub use self::meta::{
    author::{DocumentAuthor, DocumentAuthorIter},
    class::{DocumentClass, DocumentClassArticle},
    datetime::DocumentTime,
    title::DocumentTitle,
    toc::{TableOfContent, TocConfig, TocNode},
};
use chrono::{NaiveDateTime, Utc};
use notedown_ast::{
    value::{LiteralPair, OrderedMap},
    ASTNode, Value,
};
use notedown_error::{NoteError, Result};
use std::collections::BTreeMap;
use yggdrasil_shared::records::{TextIndex, Url};

#[derive(Debug)]
pub struct NoteDocument {
    context: ASTNode,
    meta: DocumentMeta,
    variable: OrderedMap,
    errors: Vec<NoteError>,
}

#[derive(Debug, Clone)]
pub struct DocumentMeta {
    path: Option<Url>,
    title: DocumentTitle,
    authors: BTreeMap<String, DocumentAuthor>,
    date: DocumentTime,
    toc: TocNode,
}

impl NoteDocument {
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
