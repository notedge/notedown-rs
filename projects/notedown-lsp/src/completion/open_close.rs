use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind};

pub fn build_open_close(cmd: &str, short: &str, long: &str) -> CompletionItem {
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    CompletionItem {
        label: format!("<{}>", cmd),
        kind: Some(CompletionItemKind::Class),
        detail: Some(String::from(short)),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: Some(format!("<{cmd}>$0<{cmd}/>$1", cmd = cmd)),
        insert_text_format: Some(InsertTextFormat::Snippet),
        text_edit: None,
        // text_edit: Some(CompletionTextEdit::Edit(TextEdit { range, new_text: "\\comment".to_string() })),
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}
