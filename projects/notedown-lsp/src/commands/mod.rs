use cli_clipboard::{ClipboardContext, ClipboardProvider};
use serde_json::Value;
use std::{collections::HashSet, lazy::SyncLazy};
use tower_lsp::{lsp_types::*, Client};

pub fn server_commands() -> ExecuteCommandOptions {
    let commands = SERVER_COMMANDS.iter().map(|s| s.to_string()).collect();
    ExecuteCommandOptions { commands, work_done_progress_options: Default::default() }
}

static SERVER_COMMANDS: SyncLazy<HashSet<&'static str>> = SyncLazy::new(|| {
    let mut s = HashSet::new();
    s.insert("vscode-notedown.injectPaste");
    s.insert("vscode-notedown.rawPaste");
    s.insert("vscode-notedown.image.save2local");
    return s;
});

pub async fn command_provider(p: ExecuteCommandParams, c: &Client) -> Option<Value> {
    c.show_message(MessageType::Log, format!("{:#?}", p.command)).await;
    let mut ctx: ClipboardContext = match ClipboardProvider::new() {
        Ok(o) => o,
        Err(e) => {
            c.show_message(MessageType::Error, e).await;
            return None;
        }
    };

    let s = match ctx.get_contents() {
        Ok(o) => o,
        Err(e) => {
            c.show_message(MessageType::Error, e).await;
            return None;
        }
    };
    c.show_message(MessageType::Info, s).await;
    return None;

    // c.apply_edit();
    //
    //
    // WorkspaceEdit {
    // changes: None,
    // document_changes: Some(DocumentChanges::Edits(vec![]) )
    // };
    //
    // TextDocumentEdit {
    // text_document: VersionedTextDocumentIdentifier { uri: (), version: None },
    // edits: vec![
    //
    // ]
    // };
    // TextEdit{
    // range: Default::default(),
    // new_text: "".to_string()
    // };
    //
}
