use crate::{errors::DiagnosticLevel, NoteError};
use std::ops::Range;
use yggdrasil_shared::records::{
    lsp_types::{Diagnostic, DiagnosticSeverity},
    TextIndex,
};

impl DiagnosticLevel {
    pub fn into_severity(self) -> Option<DiagnosticSeverity> {
        match self {
            Self::None => None,
            Self::Error => Some(DiagnosticSeverity(1i32)),
            Self::Warning => Some(DiagnosticSeverity(2i32)),
            Self::Information => Some(DiagnosticSeverity(3i32)),
            Self::Hint => Some(DiagnosticSeverity(4i32)),
        }
    }
}

impl NoteError {
    pub fn build_diag(&self, text: &TextIndex) -> Diagnostic {
        let range = match &self.range {
            None => Default::default(),
            Some(r) => text.get_lsp_range(r.start, r.end),
        };
        let severity = match self.level {
            DiagnosticLevel::None => None,
            DiagnosticLevel::Error => Some(DiagnosticSeverity(1i32)),
            DiagnosticLevel::Warning => Some(DiagnosticSeverity(2i32)),
            DiagnosticLevel::Information => Some(DiagnosticSeverity(3i32)),
            DiagnosticLevel::Hint => Some(DiagnosticSeverity(4i32)),
        };
        Diagnostic {
            range,
            severity: self.level.into_severity(),
            code: None,
            code_description: None,
            source: None,
            message: "".to_string(),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
