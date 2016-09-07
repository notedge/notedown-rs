use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, InsertTextFormat, MarkupContent, MarkupKind};

pub fn build_open_close() -> CompletionItem {
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
