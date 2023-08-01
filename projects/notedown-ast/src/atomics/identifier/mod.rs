use super::*;

/// `XID_START XID_CONTINUE*`
#[derive(Debug)]
pub struct IdentifierNode {
    /// unescaped name of identifier
    pub name: String,
}

impl IdentifierNode {
    /// Create a new identifier node with the given name.
    pub fn new<S: ToString>(body: S) -> Self {
        Self { name: body.to_string() }
    }
}
