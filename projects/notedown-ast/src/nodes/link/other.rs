#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct EmailLink {
    pub is_bare: bool,
    pub name: String,
}
