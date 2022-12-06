use super::*;
use diagnostic_quick::error_3rd::NodeLocation;

/// ````md
/// ```cmd(arg=1)
/// body text
/// ```
/// ````
#[derive(Clone, Eq, PartialEq)]
pub struct EscapedCommand {
    pub cmd: String,
    pub level: u8,
    pub start: usize,
    pub end: usize,
    pub options: CommandArguments,
    pub pattern: LiteralPattern,
    pub body: NodeLocation<String>,
}
