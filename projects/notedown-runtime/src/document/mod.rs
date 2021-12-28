use chrono::{DateTime, NaiveDateTime};
use notedown_ast::{value::OrderedMap, ASTNode};
use notedown_error::Result;

pub struct NoteDocument {
    context: ASTNode,
    meta: DocumentMeta,
    var: OrderedMap,
}

pub struct DocumentMeta {
    title: Option<String>,
    author: Vec<String>,
    date: Option<NaiveDateTime>,
}

impl NoteDocument {
    pub fn get_date(&self) -> &Option<NaiveDateTime> {
        &self.meta.date
    }
    pub fn set_date(&mut self, date: NaiveDateTime) {
        self.meta.date = Some(date);
    }
    pub fn set_date_unix(&mut self, date: i32) {
        self.meta.date = Some(date);
    }
    pub fn set_date_fmt(&mut self, date: &str, fmt: &str) -> Result<()> {
        let date = NaiveDateTime::parse_from_str(date, fmt)?;
        self.meta.date = Some(date);
        Ok(())
    }
}
