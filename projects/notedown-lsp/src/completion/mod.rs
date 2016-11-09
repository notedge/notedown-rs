mod command;
mod open_close;
mod self_close;
mod structural;

use crate::{completion::structural::complete_table, io::FILE_STORAGE};
use command::build_command;
use open_close::build_open_close;
use self_close::build_self_close;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, lazy::SyncLazy};
use tower_lsp::lsp_types::{
    CompletionItem,
    CompletionItemKind::{self, *},
    CompletionOptions, CompletionParams, CompletionResponse, Documentation, InsertTextFormat, MarkupContent, MarkupKind,
    Position, WorkDoneProgressOptions,
};
use unicode_xid::UnicodeXID;

pub static COMPLETION_OPTIONS: SyncLazy<CompletionOptions> = SyncLazy::new(|| {
    let completion_trigger = vec!['.', '\\', '[', '<'];
    CompletionOptions {
        resolve_provider: Some(false),
        trigger_characters: Some(completion_trigger.iter().map(ToString::to_string).collect()),
        work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(false) },
    }
});

pub async fn completion_provider(p: CompletionParams) -> Option<CompletionResponse> {
    let text = FILE_STORAGE.get().read().await.read(&p.text_document_position.text_document.uri);
    match text {
        Some(s) => completion_provider_dynamic(s, p.text_document_position.position),
        None => {
            let c = p.context.and_then(|e| e.trigger_character).and_then(|e| e.chars().next());
            completion_provider_static(c)
        }
    }
}

fn completion_provider_static(c: Option<char>) -> Option<CompletionResponse> {
    let mut items = vec![];
    match c {
        Some('<') => items = COMPLETE_COMPONENTS.to_owned(),
        Some('\\') => {
            items.extend(COMPLETE_COMMANDS.to_owned());
            items.extend(complete_table());
        }
        _ => return None,
    };
    Some(CompletionResponse::Array(items))
}

fn completion_provider_dynamic(text: String, position: Position) -> Option<CompletionResponse> {
    let word = get_completion_word(text, position);
    completion_provider_static(word.chars().nth(0))
}

pub fn get_completion_word(text: String, tp: Position) -> String {
    let line = tp.line as usize;
    let num = tp.character as usize;
    text.lines().nth(line).map(|e| get_word(e, num)).unwrap_or_default()
}

fn get_word(line: &str, index: usize) -> String {
    // FIXME: panic when dis-sync!!!
    let num = index.min(line.len());
    let (f, e) = (&line[..num], &line[num..]);
    let mut v = VecDeque::new();
    for c in f.chars().rev() {
        if c.is_xid_continue() || c == '\\' || c == '<' { v.push_front(c) } else { break }
    }
    for c in e.chars() {
        if c.is_xid_continue() { v.push_back(c) } else { break }
    }
    return v.iter().collect();
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

pub static COMPLETE_COMMANDS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| {
    let parsed = load_md_doc(include_str!("command.md"));
    parsed.iter().map(|doc| doc.command()).collect()
});

pub static COMPLETE_COMPONENTS: SyncLazy<Vec<CompletionItem>> = SyncLazy::new(|| {
    let open_close = load_md_doc(include_str!("open_close.md"));
    let self_close = load_md_doc(include_str!("self_close.md"));
    open_close.iter().map(|doc| doc.open_close()).chain(self_close.iter().map(|doc| doc.self_close())).collect()
});

#[allow(dead_code)]
fn list_completion_kinds() -> Vec<CompletionItem> {
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
