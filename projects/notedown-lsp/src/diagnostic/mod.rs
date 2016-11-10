mod toc;

pub use toc::ToToc;

use crate::io::read_url;
use tower_lsp::lsp_types::*;

pub fn diagnostics_provider(url: &Url) -> Vec<Diagnostic> {
    let _ = url;

    return vec![
        Diagnostic {
            range: Range::new(Position::new(0, 0), Position::new(0, 100)),
            severity: Some(DiagnosticSeverity::Error),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
        },
        Diagnostic {
            range: Range::new(Position::new(1, 0), Position::new(1, 100)),
            severity: Some(DiagnosticSeverity::Warning),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
        },
        Diagnostic {
            range: Range::new(Position::new(2, 0), Position::new(2, 100)),
            severity: Some(DiagnosticSeverity::Information),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
        },
        Diagnostic {
            range: Range::new(Position::new(3, 0), Position::new(3, 100)),
            severity: Some(DiagnosticSeverity::Hint),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: None,
        },
        Diagnostic {
            range: Range::new(Position::new(4, 0), Position::new(4, 100)),
            severity: Some(DiagnosticSeverity::Hint),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: Some(vec![DiagnosticTag::Unnecessary]),
        },
        Diagnostic {
            range: Range::new(Position::new(5, 0), Position::new(5, 100)),
            severity: Some(DiagnosticSeverity::Hint),
            code: None,
            source: Some("sourceeee ".to_string()),
            message: "messsssage ".to_string(),
            related_information: None,
            tags: Some(vec![DiagnosticTag::Deprecated]),
        },
    ];
}

#[allow(dead_code)]
pub fn comma_problems(url: &Url) -> Vec<Diagnostic> {
    let mut out = vec![];
    for (line, s) in read_url(&url).lines().enumerate() {
        let mut chars = s.chars().enumerate();
        while let Some((pos, c)) = chars.next() {
            match c {
                ',' | '.' | ':' | '!' | '?' => {
                    if let Some(c) = chars.next() {
                        if c.1 != ' ' {
                            let d = Diagnostic {
                                range: Range::new(
                                    Position::new(line as u64, pos as u64),
                                    Position::new(line as u64, pos as u64 + 2),
                                ),
                                severity: Some(DiagnosticSeverity::Error),
                                code: None,
                                source: Some("sourceeee ".to_string()),
                                message: "messsssage ".to_string(),
                                related_information: None,
                                tags: None,
                            };
                            out.push(d)
                        }
                    }
                }
                _ => {}
            }
        }
    }
    return out;
}
