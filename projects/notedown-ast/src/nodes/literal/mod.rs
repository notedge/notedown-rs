use super::*;

#[derive(Clone, Eq)]
pub struct Literal<T> {
    ///
    pub(crate) value: T,
    // pub range: Option<(u32, u32)>,
    pub(crate) range: u64,
}

impl<T: Default> Default for Literal<T> {
    fn default() -> Self {
        Self { value: Default::default(), range: 0 }
    }
}

impl<T: Debug> Debug for Literal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("ASTNode");
        w.field("kind", &self.value);
        if self.range != 0 {
            unsafe {
                let s = transmute::<u64, (u32, u32)>(self.range);
                w.field("range", &format!("{}-{}", s.0, s.1));
            };
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
    pub fn new(value: T, range: Option<(u32, u32)>) -> Self {
        Self { value, range: Self::range_from(range) }
    }

    pub fn unwrap(self) -> T {
        self.value
    }

    pub fn range(&self) -> Option<(u32, u32)> {
        Self::range_into(self.range)
    }

    pub fn range_into(u: u64) -> Option<(u32, u32)> {
        if u == 0 {
            return None;
        }
        unsafe {
            let s = transmute::<u64, (u32, u32)>(u);
            return Some(s);
        };
    }
    pub fn range_from(r: Option<(u32, u32)>) -> u64 {
        match r {
            None => 0,
            Some(s) => unsafe { transmute::<(u32, u32), u64>(s) },
        }
    }
}
