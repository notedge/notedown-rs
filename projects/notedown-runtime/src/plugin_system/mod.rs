use crate::FileMeta;
use notedown_ast::{utils::DashMap, ASTNode, Result};
use std::{
    collections::BTreeSet,
    fmt::{Debug, Formatter},
};

pub type Parser = fn(&str, &mut FileMeta) -> Result<ASTNode>;

#[derive(Debug, Default)]
pub struct PluginSystem {
    parsers: DashMap<String, PluginParser>,
}

impl PluginSystem {
    /// Register a new parser
    #[inline]
    pub fn register_parser(&self, new: PluginParser) -> Option<PluginParser> {
        self.parsers.insert(new.name.to_owned(), new)
    }
    pub fn get_parser(&self, name: &str) -> Option<Parser> {
        match self.parsers.get(name) {
            None => None,
            Some(s) => Some(s.parser),
        }
    }
    pub fn get_parser_by_extension(&self, e: &str) -> Option<Parser> {
        for parser in &self.parsers {
            if parser.name.contains(e) {
                return Some(parser.parser);
            }
        }
        return None;
    }
}

pub struct PluginParser {
    pub name: String,
    pub parser: Parser,
    pub try_extension: BTreeSet<String>,
}

impl Debug for PluginParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut f.debug_struct("Parser");
        w.field("name", &self.name);
        w.field("formats", &self.try_extension);
        w.finish()
    }
}

impl Default for PluginParser {
    fn default() -> Self {
        let mut set = BTreeSet::new();
        set.insert("text".to_string());
        Self { name: "text".to_string(), parser: text_view_parser, try_extension: set }
    }
}
pub fn text_view_parser(_: &str, _: &mut FileMeta) -> Result<ASTNode> {
    Ok(ASTNode::default())
}

pub struct ExtendedPackage {}
