use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct QuoteList {
    pub custom_style: Option<String>,
    pub children: Vec<ListItem>,
}
