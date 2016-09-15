use super::*;
use std::fmt::Debug;

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let a = self.args.values().map(|v| format!("{}", v));
        let kv = self.kvs.iter().map(|(k, v)| format!("{} = {}", k, v));
        write!(f, "\\{}({})", self.cmd, a.chain(kv).collect::<Vec<_>>().join(", "))
    }
}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cmd.hash(state);
        todo!()
    }
}

impl Debug for ExternalCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Command");
        w.field("kind", "External");
        w.field("name", &self.cmd);
        w.field("size", &self.data.len());
        w.finish()
    }
}
