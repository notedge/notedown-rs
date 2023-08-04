use crate::NotedownAST;

pub trait NoteOptimizer {
    fn optimize(&mut self, info: &NotedownAST) -> Result<NotedownAST>;
}

pub trait NoteGenerator {
    type Output;
    fn generate(&mut self, info: &NotedownAST) -> Result<Self::Output>;
}
