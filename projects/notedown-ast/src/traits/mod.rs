#[cfg(feature = "html-ast")]
pub mod html;

use crate::NotedownAST;
use notedown_error::Validation;
use crate::hir::NotedownHIR;

pub trait NoteOptimizer {
    fn optimize(&mut self, info: &NotedownHIR) -> Validation<NotedownHIR>;
}

pub trait NoteGenerator {
    type Output;
    fn generate(&mut self, info: &NotedownHIR) -> Validation<Self::Output>;
}
