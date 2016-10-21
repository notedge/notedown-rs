use super::*;

pub fn complete_table() -> Vec<CompletionItem> {
    (1..=9).flat_map(|i| (1..=9).map(move |j| build_table(i, j))).collect()
}

fn build_table(a: usize, b: usize) -> CompletionItem {
    // let doc = MarkupContent { kind: MarkupKind::Markdown, value: String::from(long) };
    let short = format!("build a {} row {} column table", a, b);
    CompletionItem {
        label: format!("Table {} × {}", a, b),
        kind: Some(CompletionItemKind::Keyword),
        detail: Some(short),
        // documentation: Some(Documentation::MarkupContent(doc)),
        documentation: None,
        deprecated: None,
        preselect: None,
        sort_text: Some(format!("9{}{}", a, b)),
        filter_text: Some(format!("\\table{}x{}",a,b)),
        insert_text: Some("|".repeat(a)),
        insert_text_format: None,
        text_edit: None,
        additional_text_edits: None,
        command: None,
        data: None,
        tags: None,
    }
}
