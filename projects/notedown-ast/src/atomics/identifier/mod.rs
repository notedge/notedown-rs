use super::*;

/// `XID_START XID_CONTINUE*`
#[derive(Debug)]
pub struct IdentifierNode {
    /// unescaped name of identifier
    pub name: String,
}

impl IdentifierNode {
    pub fn new<S: ToString>(body: S) -> Self {
        Self { name: body.to_string() }
    }
}
