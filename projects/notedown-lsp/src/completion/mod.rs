mod command;
mod open_close;
mod self_close;
mod structural;

use command::build_command;
use open_close::build_open_close;
use self_close::build_self_close;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, lazy::SyncLazy};
use tower_lsp::lsp_types::{
    CompletionContext, CompletionItem,
    CompletionItemKind::{self, *},
    CompletionList, CompletionOptions, CompletionParams, CompletionResponse, CompletionTriggerKind, Documentation,
    InsertTextFormat, MarkupContent, MarkupKind, WorkDoneProgressOptions,
};

pub static COMPLETION_OPTIONS: SyncLazy<CompletionOptions> = SyncLazy::new(|| {
    let completion_trigger = vec![".", "\\", "[", "<", "a", "b", "c", "t"];
    CompletionOptions {
        resolve_provider: Some(true),
        trigger_characters: Some(completion_trigger.iter().map(ToString::to_string).collect()),
        work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(true) },
    }
});

pub fn completion_provider(p: CompletionParams) -> Option<CompletionResponse> {
    let mut items = vec![];
    // let s = p.context.and_then(|e| e.trigger_character).and_then(|e| e.chars().next());

    match p.context {
        None => (),
        Some(s) => {
            let c = s.trigger_character.and_then(|e| e.chars().next()).unwrap_or_default();
            match s.trigger_kind {
                CompletionTriggerKind::Invoked => (),
                CompletionTriggerKind::TriggerCharacter => {
                    match c {
                        '\\' => items = complete_commands(),
                        '<' => items = complete_components(),
                        '[' => items = list_completion_kinds(),
                        _ => (),
                    };
                    return Some(CompletionResponse::List(CompletionList { is_incomplete: true, items }));
                }
                CompletionTriggerKind::TriggerForIncompleteCompletions => {
                    match c {
                        '\\' => items = complete_commands(),
                        '<' => items = complete_components(),
                        '[' => items = list_completion_kinds(),
                        _ => (),
                    };
                    return Some(CompletionResponse::List(CompletionList { is_incomplete: true, items }));
                }
            }
        }
    }

    // Some(CompletionResponse::Array(items))
    return None;
}

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
