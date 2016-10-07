use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use tower_lsp::lsp_types::*;
use serde_json::Value;
use std::collections::HashSet;
use tower_lsp::Client;


pub fn server_commands() -> ExecuteCommandOptions {
    let commands = get_all().iter().map(|s|s.to_string()).collect();
    ExecuteCommandOptions {
        commands,
        work_done_progress_options: Default::default(),
    }
}

fn get_all() -> HashSet<&'static str> {
    let mut s = HashSet::new();
    s.insert("vscode-notedown.injectPaste");
    s.insert("vscode-notedown.rawPaste");
    s.insert("vscode-notedown.image.save2local");
    return s;
}



pub fn command_provider(p: ExecuteCommandParams,c: &Client) -> Option<Value> {
    let _ = p;
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if let Ok(s) = ctx.get_contents() {
        c.show_message(MessageType::Info, s);
        // return Some(Value::String(s))
    }
/*
    c.apply_edit();


    WorkspaceEdit {
        changes: None,
        document_changes: Some(DocumentChanges::Edits(vec![]) )
    };

    TextDocumentEdit {
        text_document: VersionedTextDocumentIdentifier { uri: (), version: None },
        edits: vec![

        ]
    };
    TextEdit{
        range: Default::default(),
        new_text: "".to_string()
    };
*/


    return None
}
