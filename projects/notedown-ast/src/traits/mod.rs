use crate::NotedownAST;
use notedown_error::Validation;

pub trait NoteOptimizer {
    fn optimize(&mut self, info: &NotedownAST) -> Validation<NotedownAST>;
}

pub trait NoteGenerator {
    type Output;
    fn generate(&mut self, info: &NotedownAST) -> Validation<Self::Output>;
}
