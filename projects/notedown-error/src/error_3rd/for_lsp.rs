use super::*;

impl DiagnosticLevel {
    pub fn into_severity(self) -> Option<DiagnosticSeverity> {
        match self {
            Self::None => None,
            Self::Error => Some(DiagnosticSeverity::ERROR),
            Self::Warning => Some(DiagnosticSeverity::WARNING),
            Self::Information => Some(DiagnosticSeverity::INFORMATION),
            Self::Hint => Some(DiagnosticSeverity::HINT),
        }
    }
}

impl NoteError {
    #[inline]
    pub fn get_lsp_range(&self, text: &TextIndex) -> LSPRange {
        match &self.range {
            None => Default::default(),
            Some(r) => text.get_lsp_range(r.start, r.end),
        }
    }
    #[inline]
    pub fn get_lsp_tags(&self) -> Option<Vec<DiagnosticTag>> {
        let mut tags = vec![];
        if self.is_unnecessary() {
            tags.push(DiagnosticTag::UNNECESSARY)
        }
        if self.is_deprecated() {
            tags.push(DiagnosticTag::DEPRECATED)
        }
        return Some(tags);
    }
    #[inline]
    pub fn build_diagnostic(&self, text: &TextIndex) -> Diagnostic {
        Diagnostic {
            range: self.get_lsp_range(text),
            severity: self.level.into_severity(),
            code: None,
            code_description: None,
            source: None,
            message: "".to_string(),
            related_information: None,
            tags: self.get_lsp_tags(),
            data: None,
        }
    }
}
