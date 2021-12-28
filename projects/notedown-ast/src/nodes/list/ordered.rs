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
