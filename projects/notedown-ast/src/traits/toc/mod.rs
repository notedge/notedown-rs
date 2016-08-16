use yggdrasil_shared::records::{LSPRange, TextIndex};

mod toc;

pub trait TableOfContent {
    fn table_of_content(&self, config: &TableConfig) -> TableNode;
}

#[derive(Debug)]
pub struct TableNode {
    pub level: u8,
    pub detail: String,
    pub range: LSPRange,
    pub children: Vec<TableNode>,
}


pub struct TableConfig {
    pub text: TextIndex,
    pub max_depth: u8,
}

impl Default for TableNode {
    fn default() -> Self {
        Self { level: 0, detail: String::from("ROOT"), range: Default::default(), children: vec![] }
    }
}
