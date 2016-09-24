use notedown_parser::{notedown_ast::utils::TOC, AST};
use tower_lsp::lsp_types::{DocumentSymbol, Position, Range, SymbolKind};

pub trait ToToc {
    fn to_toc(&self) -> DocumentSymbol;
}

impl ToToc for AST {
    fn to_toc(&self) -> DocumentSymbol {
        self.toc(9).to_toc()
    }
}

impl ToToc for TOC {
    fn to_toc(&self) -> DocumentSymbol {
        let (a, b, x, y) = self.range.as_tuple();
        let start = Position { line: a, character: b };
        let end = Position { line: x, character: y };
        let children = match self.children.len() {
            0 => None,
            _ => Some(self.children.iter().map(ToToc::to_toc).collect()),
        };
        DocumentSymbol {
            name: String::from("TOC"),
            detail: Some(self.detail.to_owned()),
            kind: SymbolKind::Namespace,
            deprecated: None,
            range: Range { start, end },
            selection_range: Range { start, end: start },
            children,
        }
    }
}
