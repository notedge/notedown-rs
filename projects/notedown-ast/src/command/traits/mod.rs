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

impl Hash for CommandArguments {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.positional.hash(state);
        for i in &self.optional {
            i.hash(state)
        }
    }
}
