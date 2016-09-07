use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind};

pub fn complete_commands() -> Vec<CompletionItem> {
    vec![
        build_command(
            "comment",
            "Some comment text will not appear in the rendering result",
            "`\\comment: something will not shown`
               `\\comment[some tips not shown]`",
        ),
        build_command(
            "img",
            "Some comment text will not appear in the rendering result",
            "`\\img: something will not shown`
               `\\img[some tips not shown]`",
        ),
    ]
}

pub fn build_command(cmd: &str, short: &str, long: &str) -> CompletionItem {
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    CompletionItem {
        label: format!("\\{}", cmd),
        kind: Some(CompletionItemKind::Function),
        detail: Some(String::from(short)),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: None,
        filter_text: None,
        insert_text: Some(String::from(cmd)),
        insert_text_format: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}
