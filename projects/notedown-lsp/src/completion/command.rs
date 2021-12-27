use super::*;

pub fn build_command(cmd: &str, short: &str, long: &str) -> CompletionItem {
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    CompletionItem {
        label: format!("\\{}", cmd),
        label_details: None,
        kind: Some(CompletionItemKind::FUNCTION),
        detail: Some(String::from(short)),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("0{}", cmd)),
        filter_text: None,
        insert_text: Some(format!("\\{}", cmd)),
        insert_text_format: None,
        insert_text_mode: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
