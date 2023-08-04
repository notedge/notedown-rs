use crate::NotedownAST;
use notedown_error::Validation;

pub trait NoteOptimizer {
    fn optimize(&mut self, info: &NotedownAST) -> Validation<NotedownAST>;
}

pub trait NoteGenerator {
    type Output;
    fn generate(&mut self, info: &NotedownAST) -> Validation<Self::Output>;
}

pub struct MarkdownRenderer {}

use std::fmt::Write;

impl NoteGenerator for MarkdownRenderer {
    type Output = String;

    fn generate(&mut self, info: &NotedownAST) -> Validation<Self::Output> {
        let mut ouput = String::new();
        write!(ouput, "{}", info)?;
        todo!()
    }
}
