mod command;
mod open_close;
mod self_close;

use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind::{self, *}};
use command::build_command;
use self_close::build_self_close;
use open_close::build_open_close;

pub struct DocumentString {
    cmd: String,
    short: String,
    long: String,
}

impl DocumentString {
    pub fn command(&self) -> CompletionItem {
        build_command(&self.cmd, &self.short, &self.long)
    }
    pub fn open_close(&self) -> CompletionItem {
        build_open_close(&self.cmd, &self.short, &self.long)
    }
    pub fn self_close(&self) -> CompletionItem {
        build_self_close(&self.cmd, &self.short, &self.long)
    }
}

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


pub fn complete_self_close() -> Vec<CompletionItem> {
    vec![
        build_self_close(
            "img",
            "Some comment text will not appear in the rendering result",
            "`\\img: something will not shown`
               `\\img[some tips not shown]`",
        )
    ]
}


pub fn complete_open_close() -> Vec<CompletionItem> {
    vec![
        build_open_close(
            "comment",
            "Some comment text will not appear in the rendering result",
            "`\\img: something will not shown`
               `\\img[some tips not shown]`",
        )
    ]
}


pub fn list_completion_kinds() -> Vec<CompletionItem> {
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
