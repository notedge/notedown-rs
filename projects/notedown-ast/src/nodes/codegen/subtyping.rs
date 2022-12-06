use super::*;

impl IntoNotedown for TableView {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        NotedownKind::TableView(self).into_node(span, file)
    }
}

impl IntoNotedown for SimpleTable {
    #[inline]
    fn into_node(self, span: &Span, file: &FileID) -> NotedownNode {
        TableView::SimpleTable(Box::new(self)).into_node(span, file)
    }
}

impl NotedownKind {
    /// Constructor of [`TableView`]
    #[inline]
    pub fn table_view(value: TableView, span: &Span, file: &FileID) -> NotedownNode {
        NotedownKind::TableView(value).into_node(span, file)
    }
    /// Getter of [`TableView`]
    pub fn get_table_view(&self) -> Option<&TableView> {
        match self {
            NotedownKind::TableView(v) => Some(v),
            _ => None,
        }
    }
    /// Mutable getter of [`TableView`]
    pub fn mut_table_view(&mut self) -> Option<&mut TableView> {
        match self {
            NotedownKind::TableView(v) => Some(v),
            _ => None,
        }
    }

    /// Constructor of [`SimpleTable`]
    #[inline]
    pub fn table_simple(value: SimpleTable, span: &Span, file: &FileID) -> NotedownNode {
        TableView::SimpleTable(Box::new(value)).into_node(span, file)
    }
    /// Getter of [`SimpleTable`]
    pub fn get_table_simple(&self) -> Option<&SimpleTable> {
        match self.get_table_view()? {
            TableView::SimpleTable(v) => Some(v),
            _ => None,
        }
    }
    /// Mutable getter of [`SimpleTable`]
    pub fn mut_table_simple(&mut self) -> Option<&mut SimpleTable> {
        match self.mut_table_view()? {
            TableView::SimpleTable(v) => Some(v),
            _ => None,
        }
    }
}
