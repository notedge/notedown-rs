use super::*;

/// # Ordered List
/// ```note
/// 1. part1
/// 2. part2
///    part2
/// 3. part3
///
/// 4. part4
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderedList {
    /// First element number of the list
    pub first_order: usize,
    /// Children nodes of the list
    pub children: Vec<ListItem>,
}

impl ListView {
    /// Constructor of [`OrderedList`]
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>) -> Self {
        let list = OrderedList { first_order: 0, children };
        Self::Ordered(Box::new(list))
    }
}

impl NotedownKind {
    /// Constructor of [`OrderedList`]
    #[inline]
    pub fn ordered_list(children: Vec<ListItem>, span: &Span, file: &FileID) -> NotedownNode {
        ListView::ordered_list(children).into_node(span, file)
    }
    pub fn get_ordered_list(&self) -> Option<&OrderedList> {
        match self.get_listview()? {
            ListView::Ordered(o) => Some(o),
            _ => None,
        }
    }
    pub fn mut_ordered_list(&mut self) -> Option<&mut OrderedList> {
        match self.mut_listview()? {
            ListView::Ordered(o) => Some(o),
            _ => None,
        }
    }
}
