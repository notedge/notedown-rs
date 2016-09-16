mod command;
mod open_close;
mod self_close;

use command::build_command;
use open_close::build_open_close;
use self_close::build_self_close;
use serde::{Deserialize, Serialize};
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

pub fn complete_commands() -> Vec<CompletionItem> {
    let raw = include_str!("command.yaml");
    let parsed: Vec<DocumentString> = serde_yaml::from_str(raw).unwrap();
    parsed.iter().map(|doc| doc.command()).collect()
}

pub fn complete_components() -> Vec<CompletionItem> {
    let open_close: Vec<DocumentString> = serde_yaml::from_str(include_str!("open_close.yaml")).unwrap();
    let self_close: Vec<DocumentString> = serde_yaml::from_str(include_str!("self_close.yaml")).unwrap();
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
    let command: Vec<DocumentString> = serde_yaml::from_str(include_str!("command.yaml")).unwrap();
    let open_close: Vec<DocumentString> = serde_yaml::from_str(include_str!("open_close.yaml")).unwrap();
    let self_close: Vec<DocumentString> = serde_yaml::from_str(include_str!("self_close.yaml")).unwrap();

    println!("{:#?}", command);
    println!("{:#?}", open_close);
    println!("{:#?}", self_close);
}