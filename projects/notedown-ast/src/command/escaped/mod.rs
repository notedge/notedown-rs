use super::*;

/// ```md
/// \cmd[][](): args
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct EscapedCommand {
    pub cmd: String,
    pub level: u8,
    pub start: usize,
    pub end: usize,
    pub options: CommandOptions,
    pub pattern: CommandPattern,
    pub body: Literal<String>,
}
