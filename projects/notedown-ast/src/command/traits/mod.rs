mod show;
use super::*;
use std::fmt::{Debug, Write};

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Normal(v) => v.hash(state),
            Self::Escaped(v) => v.hash(state),
            Self::XML(v) => v.hash(state),
            Self::External(v) => v.hash(state),
        }
    }
}

impl Hash for CommandOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.args.hash(state);
        for i in &self.kvs {
            i.hash(state)
        }
    }
}
