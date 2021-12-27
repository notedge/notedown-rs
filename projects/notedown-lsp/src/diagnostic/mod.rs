use lspower::lsp::*;

pub fn diagnostics_provider(url: &Url) -> Vec<Diagnostic> {
    let _ = url;

    return vec![
        Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 100)),
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
            data: None,
        },
        Diagnostic {
            range: Range::new(Position::new(1, 0), Position::new(1, 100)),
            severity: Some(DiagnosticSeverity::WARNING),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
            data: None,
        },
        Diagnostic {
            range: Range::new(Position::new(2, 0), Position::new(2, 100)),
            severity: Some(DiagnosticSeverity::INFORMATION),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
            data: None,
        },
        Diagnostic {
            range: Range::new(Position::new(3, 0), Position::new(3, 100)),
            severity: Some(DiagnosticSeverity::HINT),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
            data: None,
        },
        Diagnostic {
            range: Range::new(Position::new(4, 0), Position::new(4, 100)),
            severity: Some(DiagnosticSeverity::HINT),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: Some(vec![DiagnosticTag::UNNECESSARY]),
            data: None,
        },
        Diagnostic {
            range: Range::new(Position::new(5, 0), Position::new(5, 100)),
            severity: Some(DiagnosticSeverity::HINT),
            code: None,
            code_description: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: Some(vec![DiagnosticTag::DEPRECATED]),
            data: None,
        },
    ];
}
