use super::*;
use crate::NoteDocument;
use std::fmt::Display;

/// A parser which can parse text into ast, and report errors at the same time
pub type Parser = fn(&str, &mut Vec<NoteError>) -> Result<ASTNode>;

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

impl Display for PluginParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Default for PluginParser {
    fn default() -> Self {
        let mut set = BTreeSet::new();
        set.insert("text".to_string());
        Self { name: "text".to_string(), parser: text_view_parser, try_extension: set }
    }
}

pub fn text_view_parser(_: &str, _: &mut NoteDocument) -> Result<()> {
    Ok(ASTNode::default())
}

impl Hash for PluginParser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq<Self> for PluginParser {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for PluginParser {}
