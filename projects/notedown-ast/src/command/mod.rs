#![doc = include_str!("readme.md")]
mod escaped;
mod external;
mod normal;
mod options;
mod traits;
mod xml;
pub use self::xml::{XMLCommand, XMLCommandMarks};
use crate::{
    command::{escaped::EscapedCommand, external::ExternalCommand, normal::NormalCommand},
    nodes::Literal,
    traits::IntoASTNode,
    value::*,
    ASTKind, ASTNode,
};
use notedown_error::MaybeRanged;
use std::ops::Range;

/// Aka. Macro.
/// It can be understood as a mark that attempts to expand in a given context
/// until the position cannot be expanded
#[derive(Clone, Eq, PartialEq)]
pub enum Command {
    /// Command in normal form
    Normal(NormalCommand),
    /// Command in escape form
    Escaped(EscapedCommand),
    /// Command in XML form
    XML(XMLCommand),
    /// Command in external form
    External(ExternalCommand),
}

/// Available arguments for the command
/// - positional: `\cmd(a, b, c)`
/// - optional: `\cmd(a = 1, b = 2)`
/// - pattern: `\cmd[a][b]`
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandArguments {
    /// Arguments which depends on position
    pub positional: SparseArray,
    /// Arguments forms of key-value pairs
    pub optional: OrderedMap,
    /// Arguments short string pattern
    pub pattern: LiteralPattern,
}

impl Command {
    /// Check if the command is some given name
    #[inline]
    pub fn is(&self, rhs: impl AsRef<str>) -> bool {
        self.command().eq(rhs.as_ref())
    }
    /// Get the name of this command
    #[inline]
    pub fn command(&self) -> &str {
        match self {
            Self::Normal(v) => v.cmd.as_str(),
            Self::Escaped(v) => v.cmd.as_str(),
            Self::XML(v) => v.cmd.as_str(),
            Self::External(v) => v.cmd.as_str(),
        }
    }
}

impl CommandArguments {
    /// Returns true if theres no arguments given
    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.optional.is_empty() && self.pattern.is_empty()
    }
}
