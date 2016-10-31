use super::*;

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
