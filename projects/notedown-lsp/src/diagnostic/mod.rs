mod toc;
pub use toc::ToToc;

use crate::{io::read_url, Backend};
use tower_lsp::lsp_types::*;

impl Backend {
    pub async fn check_the_file(&self, url: Url) {
        self.client.publish_diagnostics(url, comma_problems(&url), None).await
    }
}

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
