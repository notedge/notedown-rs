use chrono::{DateTime, NaiveDateTime};
use notedown_ast::{value::OrderedMap, ASTNode};

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
    pub fn set_date_string(&mut self, date: i32) -> Result<()> {
        NaiveDateTime::parse_from_str(s);
        self.meta.date = Some(date);
    }
}
