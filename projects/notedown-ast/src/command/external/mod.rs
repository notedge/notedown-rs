use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    pub cmd: String,
    pub data: Vec<u8>,
}

impl ExternalCommand {
    #[inline]
    pub fn command_link(cmd: String, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }
}

impl Command {
    #[inline]
    pub fn command_link(cmd: String, data: Vec<u8>) -> Self {
        Self::External(ExternalCommand::command_link(cmd, data))
    }
}

impl ASTKind {
    #[inline]
    pub fn command_line(cmd: String, data: Vec<u8>, r: Option<OffsetRange>) -> ASTNode {
        Command::command_link(cmd, data).into_node(r)
    }
}
