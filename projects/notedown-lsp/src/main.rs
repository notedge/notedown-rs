use crate::completion::{complete_commands, list_completion_kinds};
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

mod completion;

#[derive(Debug)]
struct Backend {
    client: Client,
}

fn c2() -> CompletionItem {
    CompletionItem {
        label: "<comment>".to_string(),
        kind: Some(CompletionItemKind::Class),
        detail: Some("Some comment text will not appear in the rendering result".to_string()),
        documentation: Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: "`\\comment: something will not shown`
                    `\\comment[some tips not shown]`\
                    "
                .to_string(),
        })),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: Some("<comment>$0<comment/>".to_string()),
        insert_text_format: Some(InsertTextFormat::Snippet),
        text_edit: None,
        // text_edit: Some(CompletionTextEdit::Edit(TextEdit { range, new_text: "\\comment".to_string() })),
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let server_info = ServerInfo { name: "Notedown LSP".to_string(), version: Some("V1".to_string()) };
        let completion_trigger = vec![".", "\\", "[", "<"];
        let completion = CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(completion_trigger.iter().map(ToString::to_string).collect()),
            work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(true) },
        };
        let ws = WorkspaceCapability {
            workspace_folders: Some(WorkspaceFolderCapability {
                supported: Some(true),
                change_notifications: Some(WorkspaceFolderCapabilityChangeNotifications::Bool(true)),
            }),
        };

        let init = InitializeResult {
            server_info: Some(server_info),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::Incremental)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(completion),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: None,
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                document_highlight_provider: Some(false),
                workspace_symbol_provider: Some(true),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(ws),
                ..ServerCapabilities::default()
            },
        };
        return Ok(init);
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::Info, "Notedown server initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, cp: CompletionParams) -> Result<Option<CompletionResponse>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", cp)).await;
        let mut items = vec![];

        match cp.context {
            None => (),
            Some(s) => match s.trigger_kind {
                CompletionTriggerKind::Invoked => (),
                CompletionTriggerKind::TriggerCharacter => match s.trigger_character.unwrap().as_str() {
                    "\\" => items = complete_commands(),
                    "<" => items = vec![c2()],
                    "." => items = list_completion_kinds(),
                    _ => (),
                },
                CompletionTriggerKind::TriggerForIncompleteCompletions => (),
            },
        };
        // const lineTextBefore = document.lineAt(position.line).text.substring(0, position.character);
        // const lineTextAfter = document.lineAt(position.line).text.substring(position.character);
        Ok(Some(CompletionResponse::Array(items)))
    }

    // async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
    //    Ok(params)
    //}

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover { contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())), range: None }))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout).interleave(messages).serve(service).await;
}
