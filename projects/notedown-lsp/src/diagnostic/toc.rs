use notedown_parser::{utils::TOC, AST};
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
        let start = Position { line: a.saturating_sub(1), character: b.saturating_sub(1) };
        let end = Position { line: x.saturating_sub(1), character: y.saturating_sub(1) };
        let children = match self.children.len() {
            0 => None,
            _ => Some(self.children.iter().map(ToToc::to_toc).collect()),
        };
        #[allow(deprecated)]
        DocumentSymbol {
            name: self.detail.to_owned(),
            detail: Some(format!("H{}", self.children.len())),
            kind: SymbolKind::Number,
            deprecated: None,
            range: Range { start, end },
            selection_range: Range { start, end },
            children,
        }
    }
}
