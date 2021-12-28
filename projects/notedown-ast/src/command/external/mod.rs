use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    pub cmd: String,
    pub data: Vec<u8>,
}

impl ExternalCommand {
    #[inline]
    pub fn new(cmd: String, data: Vec<u8>) -> Self {
        Self { cmd, data }
    }
}

impl Command {
    #[inline]
    pub fn external(cmd: String, data: Vec<u8>) -> Self {
        Self::External(ExternalCommand::new(cmd, data))
    }
}

impl ASTKind {
    #[inline]
    pub fn command_external<S: Into<String>>(cmd: S, data: Vec<u8>, r: MaybeRanged) -> ASTNode {
        Command::external(cmd.into(), data).into_node(r)
    }
}
