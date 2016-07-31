mod arith;
mod display;
mod slugify;
mod context;

pub use context::ContextKind;

pub trait Slugify {
    fn slugify(&self) -> String;
}

pub trait ContextAware {
    fn context_aware(&self, offset: u32) -> ContextKind;
}