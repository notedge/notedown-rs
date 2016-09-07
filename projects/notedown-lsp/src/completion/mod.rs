use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};
mod command;
mod open_close;
mod self_close;

pub use command::complete_commands;

pub fn list_completion_kinds() -> Vec<CompletionItem> {
    use tower_lsp::lsp_types::CompletionItemKind::*;
    fn item(e: CompletionItemKind) -> CompletionItem {
        CompletionItem { label: format!("{:?}", e), kind: Some(e), ..CompletionItem::default() }
    }

    vec![
        item(Text),
        item(Method),
        item(Function),
        item(Constructor),
        item(Field),
        item(Variable),
        item(Class),
        item(Interface),
        item(Module),
        item(Property),
        item(Unit),
        item(Value),
        item(Enum),
        item(Keyword),
        item(Snippet),
        item(Color),
        item(File),
        item(Reference),
        item(Folder),
        item(EnumMember),
        item(Constant),
        item(Struct),
        item(Event),
        item(Operator),
        item(TypeParameter),
    ]
}
