#[cfg(feature = "html-ast")]
pub mod html;

use crate::{hir::NotedownHIR, NotedownAST};
use notedown_error::Validation;

/// Apply IR -> IR transformation
pub trait NoteOptimizer {
    /// Optimize the given notedown AST
    fn optimize(&mut self, ir: &NotedownHIR) -> Validation<NotedownHIR>;
}
/// Convert notedown to other formats
pub trait NoteGenerator {
    /// The output format
    type Output;
    /// Generate the output with the given notedown AST
    fn generate(&mut self, ir: &NotedownHIR) -> Validation<Self::Output>;
}
