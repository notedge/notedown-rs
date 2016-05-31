use crate::{ast::highlighter::Highlighter, Command, CommandKind, Value};
use std::collections::HashMap;

impl From<Highlighter> for Command {
    fn from(h: Highlighter) -> Self {
        let mut kvs: HashMap<&str, Value> = Default::default();
        kvs.insert("body", Value::String(h.code.into()));
        Command { cmd: h.lang, args: vec![], kvs, kind: CommandKind::SmartLink }
    }
}
