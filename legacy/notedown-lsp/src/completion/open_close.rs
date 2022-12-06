use super::*;

pub fn build_open_close(cmd: &str, short: &str, long: &str) -> CompletionItem {
    let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    CompletionItem {
        label: format!("<{}>", cmd),
        label_details: None,
        kind: Some(CompletionItemKind::CLASS),
        detail: Some(String::from(short)),
        documentation: Some(Documentation::MarkupContent(doc)),
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("0{}", cmd)),
        filter_text: None,
        insert_text: Some(format!("{cmd}>$1<{cmd}/>$0", cmd = cmd)),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        insert_text_mode: None,
        text_edit: None,
        // text_edit: Some(CompletionTextEdit::Edit(TextEdit { range, new_text: "\\comment".to_string() })),
        additional_text_edits: None,
        command: None,
        commit_characters: None,
        data: None,
        tags: None,
    }
}
