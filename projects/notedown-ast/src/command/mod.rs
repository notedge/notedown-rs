mod escaped;
mod external;
mod normal;
mod options;
mod traits;
mod xml;

use self::xml::{XMLCommand, XMLCommandKind};
use crate::{
    command::{escaped::EscapedCommand, external::ExternalCommand, normal::NormalCommand},
    nodes::{Literal, OffsetRange},
    ASTKind, ASTNode,
};
pub use options::{CommandOptions, CommandPattern};
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

#[derive(Clone, Eq, PartialEq)]
pub enum Command {
    /// ```md
    /// \cmd: args
    /// ```
    Normal(NormalCommand),
    Escaped(EscapedCommand),
    XML(XMLCommand),
    External(ExternalCommand),
}

impl Command {
    #[inline]
    pub fn is(&self, rhs: impl AsRef<str>) -> bool {
        self.command().eq(rhs.as_ref())
    }
    #[inline]
    pub fn command(&self) -> &str {
        match self {
            Self::Normal(v) => v.cmd.as_str(),
            Self::Escaped(v) => v.cmd.as_str(),
            Self::XML(v) => v.cmd.as_str(),
            Self::External(v) => v.cmd.as_str(),
        }
    }
    #[inline]
    pub fn into_node(self, range: Option<OffsetRange>) -> ASTNode {
        ASTNode { value: ASTKind::Command(box self), range }
    }
}
