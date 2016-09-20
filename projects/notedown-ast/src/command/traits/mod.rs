mod show;
use super::*;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

impl Hash for Command {
    /// command with the same name considered to be the same
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.command().hash(state)
    }
}

impl Hash for CommandPattern {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in &self.pts {
            i.value.hash(state)
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
