use super::*;
use std::{cmp::Ordering, ops::Range};

#[derive(Clone, Eq)]
pub struct Literal<T> {
    ///
    pub value: T,
    //
    pub range: Option<Range<usize>>,
}

impl<T: Default> Default for Literal<T> {
    fn default() -> Self {
        Self { value: Default::default(), range: None }
    }
}

impl<T: Debug> Debug for Literal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("ASTNode");
        w.field("kind", &self.value);
        if let Some(s) = &self.range {
            w.field("range", &format!("{}-{}", s.start, s.end));
        }
        w.finish()
    }
}

impl<T: Hash> Hash for Literal<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T: PartialEq> PartialEq for Literal<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T: PartialOrd> PartialOrd for Literal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Literal<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
