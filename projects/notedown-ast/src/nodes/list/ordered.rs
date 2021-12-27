use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct OrderedList {
    pub first_order: usize,
    pub children: Vec<ListItem>,
}
