use super::*;

/// A list item contains the prefix and rest text nodes
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListItem {
    /// Prefix of the list item
    pub prefix: Literal<ListPrefixMark>,
    /// Rest parts the list item
    pub rest: NotedownNodes,
}

impl From<NotedownNodes> for ListItem {
    fn from(node: NotedownNodes) -> Self {
        Self { prefix: Default::default(), rest: node }
    }
}

impl ListItem {}
