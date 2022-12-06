use super::*;

/// ## Orderless List
/// ```note
/// - part1
/// - part2
///   part2
/// - part3
///
/// - part4
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderlessList {
    /// list items
    pub children: Vec<ListItem>,
}

impl ListView {
    /// Constructor of [`OrderlessList`]
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>) -> Self {
        let list = OrderlessList { children };
        Self::Orderless(Box::new(list))
    }
}

impl NotedownKind {
    /// Constructor of [`OrderlessList`]
    #[inline]
    pub fn orderless_list(children: Vec<ListItem>, span: &Span, file: &FileID) -> NotedownNode {
        ListView::orderless_list(children).into_node(span, file)
    }
    pub fn get_orderless_list(&self) -> Option<&OrderlessList> {
        match self.get_listview()? {
            ListView::Orderless(o) => Some(o),
            _ => None,
        }
    }
    pub fn mut_orderless_list(&mut self) -> Option<&mut OrderlessList> {
        match self.mut_listview()? {
            ListView::Orderless(o) => Some(o),
            _ => None,
        }
    }
}
