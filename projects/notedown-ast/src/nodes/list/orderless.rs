use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct OrderlessList {
    pub custom_style: Option<String>,
    pub children: Vec<ListItem>,
}
