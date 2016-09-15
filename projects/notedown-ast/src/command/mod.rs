mod escaped;
mod external;
mod normal;
mod options;
mod traits;
mod xml;

use self::xml::{XMLCommand, XMLCommandKind};
use crate::{
    nodes::{Array, Literal, Object, OffsetRange},
    ASTKind, ASTNode,
};
pub use options::CommandOptions;
use std::{
    fmt,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    ops::Range,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CommandKind {
    /// ```md
    /// \cmd: args
    /// ```
    Inline,
    /// ```md
    /// \cmd(
    ///     arg = 1
    /// )
    /// ```
    Normal,
    /// ````md
    /// ```cmd(arg=1)
    /// body text
    /// ```
    /// ````
    CodeBlock,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// ```md
    /// \cmd: args
    /// ```
    Normal(NormalCommand),
    Escaped(),
    XML(XMLCommand),
    External(ExternalCommand),
}

/// ```md
/// \cmd[][](): args
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalCommand {
    pub cmd: String,
    pub options: CommandOptions,
    pub rest: Literal<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalCommand {
    cmd: String,
    data: Vec<u8>,
}

impl Command {
    #[inline]
    pub fn is(&self, rhs: impl AsRef<str>) -> bool {
        self.cmd.as_str() == rhs.as_ref()
    }
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::Command(box self), range }
    }
}

impl Command {
    #[inline]
    pub fn command_line(cmd: String, _: String) -> Command {
        Self { cmd, kind: CommandKind::Inline, args: Default::default(), kvs: Default::default() }
    }
}

impl ASTKind {
    #[inline]
    pub fn command_line(cmd: impl Into<String>, content: impl Into<String>, r: Option<OffsetRange>) -> ASTNode {
        Command::command_line(cmd.into(), content.into()).into_node(r)
    }
}
