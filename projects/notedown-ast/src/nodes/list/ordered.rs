use super::*;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct OrderedList {
    pub children: Vec<ListItem>,
}
