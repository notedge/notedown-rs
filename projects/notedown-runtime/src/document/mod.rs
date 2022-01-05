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
use crate::plugin_system::Parser;
use chrono::{NaiveDateTime, Utc};
use notedown_ast::{
    value::{LiteralPair, OrderedMap},
    ASTNode, Value,
};
use notedown_error::{NoteError, Result};
use std::collections::BTreeMap;
use yggdrasil_shared::records::{Rope, TextIndex, Url};

#[cfg(feature = "native")]
pub(crate) mod native_wrap {
    pub use crate::VMFileSystem;
    pub use chrono::{DateTime, TimeZone};
    pub use std::{
        fs::read_to_string,
        time::{SystemTime, UNIX_EPOCH},
    };
}
#[cfg(feature = "wasm")]
pub(crate) mod wasm_wrap {}
#[cfg(feature = "native")]
use native_wrap::*;
#[cfg(feature = "wasm")]
use wasm_wrap::*;

#[derive(Debug)]
pub struct NoteDocument {
    url: Url,
    /// used to check weather the file needs re-parse
    fingerprint: u128,
    text: Rope,
    ast: ASTNode,
    variable: OrderedMap,
    errors: Vec<NoteError>,
    meta: DocumentMeta,
}

#[derive(Debug, Clone)]
pub struct DocumentMeta {
    title: DocumentTitle,
    authors: BTreeMap<String, DocumentAuthor>,
    date: DocumentTime,
    toc: TocNode,
}

impl PartialEq for NoteDocument {
    fn eq(&self, other: &Self) -> bool {
        self.fingerprint.eq(&other.fingerprint)
    }
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
    pub fn get_text(&self) -> String {
        self.text.chars().collect()
    }

    #[inline]
    pub fn get_text_index(&self) -> TextIndex {
        TextIndex::new(self.get_text())
    }

    #[inline]
    pub fn get_ast(&self) -> &ASTNode {
        &self.ast
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

impl NoteDocument {
    #[inline]
    pub async fn update_text(&mut self) -> Result<()> {
        self.text = Rope::from_str(&Self::load_url(&self.url).await?);
        Ok(())
    }
    #[inline]
    pub async fn update_document(&mut self, parse: &Parser) -> Result<()> {
        let text: String = self.text.chars().collect();
        let mut errors = vec![];
        let parsed = parse(&text, &mut errors)?;
        todo!()
    }
}

impl NoteDocument {
    pub async fn load_url(url: &Url) -> Result<String> {
        #[cfg(feature = "native")]
        match url.scheme() == "file" {
            true => Self::load_local_url(url).await,
            false => Self::load_remote_url_native(url).await,
        };
        #[cfg(feature = "wasm")]
        match url.scheme() == "file" {
            true => Err(NoteError::runtime_error("Can not load local file from wasm")),
            false => Self::load_remote_url_wasm(url).await,
        }
    }
    #[cfg(feature = "native")]
    #[inline(always)]
    async fn load_local_url(url: &Url) -> Result<String> {
        let path = url.to_file_path()?;
        Ok(read_to_string(path)?)
    }
    #[cfg(feature = "native")]
    #[inline(always)]
    async fn load_remote_url_native(url: &Url) -> Result<String> {
        todo!("Remote: {}", url)
    }
    #[cfg(feature = "wasm")]
    #[inline(always)]
    async fn load_remote_url_wasm(url: &Url) -> Result<String> {
        todo!("Remote: {}", url)
    }
}
