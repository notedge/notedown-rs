use super::*;
use diagnostic_quick::{FileID, Span};

/// Serialized external data
#[derive(Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    /// Name of the command
    pub cmd: String,
    /// Value of the command
    pub data: Vec<u8>,
}

impl ExternalCommand {
    /// Create a new external command
    #[inline]
    pub fn new(cmd: String, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }
}
