use std::mem::take;
use notedown_error::{NoteError, Validation};
use crate::hir::{HeadingNode, NotedownHIR, NotedownNode, ParagraphNode};
use crate::NoteGenerator;

#[derive(Default)]
pub struct HtmlBuffer {
    buffer: String,
}

impl NoteGenerator for HtmlBuffer {
    type Output = String;

    fn generate(&mut self, info: &NotedownHIR) -> Validation<Self::Output> {
        let mut errors = vec![];
        match info.write_html(self, &mut errors) {
            Ok(_) => {
                Validation::Success { value: take(&mut self.buffer), diagnostics: errors }
            }
            Err(e) => Validation::Failure {
                fatal: e,
                diagnostics: errors,
            }
        }
    }
}

trait WriteHtml {
    fn write_html(&self, buffer: &mut HtmlBuffer, errors: &mut Vec<NoteError>) -> Result<(), NoteError>;
}


impl WriteHtml for NotedownHIR {
    fn write_html(&self, buffer: &mut HtmlBuffer, errors: &mut Vec<NoteError>) -> Result<(), NoteError> {
        for node in &self.node {
            node.write_html(buffer, errors)?;
        }
        Ok(())
    }
}

impl WriteHtml for NotedownNode {
    fn write_html(&self, buffer: &mut HtmlBuffer, errors: &mut Vec<NoteError>) -> Result<(), NoteError> {
        match self {
            NotedownNode::Heading(v) => {
                v.write_html(buffer, errors)
            }
            NotedownNode::Paragraph(v) => {
                v.write_html(buffer, errors)
            }
        }
    }
}

impl WriteHtml for HeadingNode {
    fn write_html(&self, buffer: &mut HtmlBuffer, errors: &mut Vec<NoteError>) -> Result<(), NoteError> {
        todo!()
    }
}

impl WriteHtml for ParagraphNode {
    fn write_html(&self, buffer: &mut HtmlBuffer, errors: &mut Vec<NoteError>) -> Result<(), NoteError> {
        todo!()
    }
}