use super::*;
use std::{cmp::Ordering, ops::Deref};

/// Used to represent a node with positions
#[derive(Clone, Eq)]
pub struct Literal<T> {
    /// The actual value
    pub value: T,
    /// The Start offset and end offset
    pub range: MaybeRanged,
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
            w.field("range", s);
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

impl<T> Deref for Literal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
