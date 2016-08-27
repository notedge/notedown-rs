#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct TagReference {
    /// ## Tag Block
    /// ```note
    /// [^tag]: text text text
    /// [^tag]:
    ///     text text text
    ///     text text text
    /// ```
    pub inline: bool,
    /// ## Tag Reference
    /// Quote a number, note that the name is irrelevant, and a number will always be generated in sequence at the end
    /// ```note
    /// text [^tag] text text text
    /// ```
    pub tag: String,
    /// ## Tag Inline
    /// Quote a number while defining
    /// ```note
    /// text [^tag][text text text] text
    /// ```
    pub text: Vec<String>,
}
