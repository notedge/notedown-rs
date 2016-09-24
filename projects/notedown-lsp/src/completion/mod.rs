mod command;
mod open_close;
mod self_close;

use command::build_command;
use open_close::build_open_close;
use self_close::build_self_close;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tower_lsp::lsp_types::{
    CompletionItem,
    CompletionItemKind::{self, *},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentString {
    cmd: String,
    short: String,
    long: String,
}

impl DocumentString {
    pub fn new(cmd: &str, short: &str, long: &str) -> DocumentString {
        Self { cmd: String::from(cmd.trim()), short: String::from(short.trim()), long: String::from(long.trim()) }
    }

    pub fn command(&self) -> CompletionItem {
        build_command(&self.cmd, &self.short, &self.long)
    }
    pub fn open_close(&self) -> CompletionItem {
        build_open_close(&self.cmd, &self.short, &self.long)
    }
    pub fn self_close(&self) -> CompletionItem {
        build_self_close(&self.cmd, &self.short, &self.long)
    }
}

fn load_md_doc(input: &str) -> Vec<DocumentString> {
    let mut out = VecDeque::new();
    let mut cmd = "";
    let mut short = "";
    let mut long = String::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.starts_with("# ") {
            out.push_back(DocumentString::new(cmd, short, &long));
            cmd = &line[2..line.len()];
            short = lines.next().unwrap();
            long = String::new()
        }
        else {
            long.push_str(line);
            long.push('\n');
        }
    }
    out.push_back(DocumentString::new(cmd, short, &long));
    out.pop_front();
    return Vec::from(out);
}

pub fn complete_commands() -> Vec<CompletionItem> {
    let parsed = load_md_doc(include_str!("command.md"));
    parsed.iter().map(|doc| doc.command()).collect()
}

pub fn complete_components() -> Vec<CompletionItem> {
    let open_close = load_md_doc(include_str!("open_close.md"));
    let self_close = load_md_doc(include_str!("self_close.md"));
    open_close.iter().map(|doc| doc.open_close()).chain(self_close.iter().map(|doc| doc.self_close())).collect()
}

pub fn list_completion_kinds() -> Vec<CompletionItem> {
    fn item(e: CompletionItemKind) -> CompletionItem {
        CompletionItem { label: format!("{:?}", e), kind: Some(e), ..CompletionItem::default() }
    }

    vec![
        item(Text),
        item(Method),
        item(Function),
        item(Constructor),
        item(Field),
        item(Variable),
        item(Class),
        item(Interface),
        item(Module),
        item(Property),
        item(Unit),
        item(Value),
        item(Enum),
        item(Keyword),
        item(Snippet),
        item(Color),
        item(File),
        item(Reference),
        item(Folder),
        item(EnumMember),
        item(Constant),
        item(Struct),
        item(Event),
        item(Operator),
        item(TypeParameter),
    ]
}

#[test]
fn check_yaml() {
    println!("{:#?}", load_md_doc(include_str!("command.md")));
    println!("{:#?}", load_md_doc(include_str!("open_close.md")));
    println!("{:#?}", load_md_doc(include_str!("self_close.md")));
}
