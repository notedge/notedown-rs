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

    /// `[]`
    SmartLink,
    /// ````md
    /// ```cmd(arg=1)
    /// body text
    /// ```
    /// ````
    CodeBlock,
    /// ```md
    /// <cmd arg=1>body text</cmd>
    /// ```
    OpenClose,
    /// ```md
    /// `<cmd arg=1/>`
    /// ```
    SelfClose,
}
// impl Display for Command {
// fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// let a = self.args.iter().map(|v| format!("{}", v));
// let kv = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v));
//
// write!(f, "\\{}({})", self.cmd, a.chain(kv).collect::<Vec<_>>().join(", "))
// }
// }
