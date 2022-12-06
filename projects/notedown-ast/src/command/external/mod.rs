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

impl Command {
    /// Constructor of [`ExternalCommand`]
    #[inline]
    pub fn external(cmd: String, data: Vec<u8>) -> Self {
        Self::External(ExternalCommand::new(cmd, data))
    }
}

impl NotedownKind {
    /// Constructor of [`ExternalCommand`]
    #[inline]
    pub fn command_external<S: Into<String>>(cmd: S, data: Vec<u8>, span: &Span, &file: FileID) -> NotedownNode {
        Command::external(cmd.into(), data).into_node(span, file)
    }
    pub fn as_command_external(&self) -> Option<&ExternalCommand> {
        match self {
            Self::Command(Command::External(v)) => Some(v),
            _ => None,
        }
    }
}
