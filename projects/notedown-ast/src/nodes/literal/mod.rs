use super::*;

#[derive(Clone, Eq)]
pub struct Literal<T> {
    ///
    pub(crate) value: T,
    //
    pub(crate) range: Option<(u32, u32)>,
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
        if let Some(s) = self.range {
            w.field("range", &format!("{}-{}", s.0, s.1));
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

impl<T> Literal<T> {
    #[inline]
    pub fn new(value: T, range: Option<(u32, u32)>) -> Self {
        Self { value, range }
    }
    #[inline]
    pub fn unwrap(&self) -> &T {
        &self.value
    }

    #[inline]
    pub fn range(&self) -> Option<(u32, u32)> {
        self.range
    }
}
