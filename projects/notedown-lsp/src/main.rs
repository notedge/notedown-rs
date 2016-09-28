use crate::{
    completion::{complete_commands, complete_components, list_completion_kinds}
};
use serde_json::Value;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};
use crate::diagnostic::{document_symbol_provider, diagnostics_provider};

mod completion;
mod diagnostic;
mod io;
mod code_action;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let server_info = ServerInfo { name: String::from("Notedown LSP"), version: Some("V1".to_string()) };
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
                code_action_provider: Some(
                    CodeActionProviderCapability::Simple(true)
                ),
                code_lens_provider:Some(
                    CodeLensOptions {
                        resolve_provider: None
                    }
                ),
                document_highlight_provider: Some(false),
                // semantic_highlighting: None,
                document_symbol_provider: Some(true),
                document_formatting_provider: Some(true),
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
    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
    }
    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
    async fn did_open(&self, p: DidOpenTextDocumentParams) {
        self.check_the_file(p.text_document.uri).await
    }

    // 不要把东西都做这里面, 太卡了
    async fn did_change(&self, p: DidChangeTextDocumentParams) {
        self.client.log_message(MessageType::Info, format!("{:#?}", p)).await;
    }
    // 所有的检查在保存之后做
    async fn did_save(&self, p: DidSaveTextDocumentParams) {
        self.check_the_file(p.text_document.uri).await
    }
    async fn did_close(&self, p: DidCloseTextDocumentParams) {
        self.check_the_file(p.text_document.uri).await
    }
    async fn completion(&self, cp: CompletionParams) -> Result<Option<CompletionResponse>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", cp)).await;
        let mut items = vec![];
        if let Some(s) = cp.context {
            match s.trigger_kind {
                CompletionTriggerKind::Invoked => (),
                CompletionTriggerKind::TriggerCharacter => match s.trigger_character.unwrap().as_str() {
                    "\\" => items = complete_commands(),
                    "<" => items = complete_components(),
                    "[" => items = list_completion_kinds(),
                    _ => (),
                },
                CompletionTriggerKind::TriggerForIncompleteCompletions => (),
            }
        };
        Ok(Some(CompletionResponse::Array(items)))
    }
    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        Ok(params)
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        //self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(Some(Hover { contents: HoverContents::Scalar(MarkedString::LanguageString(
            LanguageString {
                language: "yaml".to_string(),
                value: format!("{:#?}", params)
            }
        )), range: None }))
    }
    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }

    /// 当光标在位置 x 时, 哪些内容要被选中
    async fn document_highlight(&self, _: DocumentHighlightParams) -> Result<Option<Vec<DocumentHighlight>>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", hp)).await;
        Ok(None)
    }

    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", sp)).await;
        Ok(document_symbol_provider(params))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        let act = CodeActionOrCommand::CodeAction(
            CodeAction {
                title: "GG1".to_string(),
                kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
                diagnostics: None,
                edit: None,
                command: None,
                is_preferred: Some(true)
            }
        );
        let cmd = CodeActionOrCommand::Command(
            Command {
                title: format!("{:#?}", params),
                command: "fffff".to_string(),
                arguments: None
            }
        );
        Ok(Some(vec![
            act,cmd
        ]))
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        let len = CodeLens{
            range: Range{
                start: Position {
                    line: 0,
                    character: 0
                },
                end: Position {
                    line: 1,
                    character: 1
                }
            },
            command: Some( Command {
                title: format!("{:#?}", params),
                command: "GGGGGGGGGGGGg".to_string(),
                arguments: None
            }),
            data: Some(Value::String("lens".to_string()))
        };

        Ok(Some(vec![len]))
    }

    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(vec![])
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
}

impl Backend {
    pub async fn check_the_file(&self, url: Url) {
        self.client.publish_diagnostics(url.clone(), diagnostics_provider(&url), None).await
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout).interleave(messages).serve(service).await;
}
