use super::*;
use diagnostic_quick::error_3rd::NodeLocation;

/// ```md
/// \cmd[][](): args
/// ```
/// ```md
/// \cmd(
///     arg = 1
/// )
/// ```
#[derive(Clone, Eq, PartialEq)]
pub struct NormalCommand {
    /// The name of the command
    ///
    /// ## Example
    ///
    /// ```note
    /// \cmd
    /// ```
    pub cmd: String,
    /// The standard argument of the command
    ///
    /// ## Example
    ///
    /// ```note
    /// \cmd(arguments, options)
    /// ```
    pub options: CommandArguments,
    /// The pattern argument of the command
    ///
    /// ## Example
    ///
    /// ```note
    /// \cmd[p1][p2]
    /// ```
    pub pattern: Vec<NodeLocation<String>>,
    /// The body argument of the command
    ///
    /// ## Example
    ///
    /// ```note
    /// \cmd: body
    /// ```
    pub body: NodeLocation<String>,
}
