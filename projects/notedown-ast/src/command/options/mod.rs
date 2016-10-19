use super::*;

impl CommandPattern {
    #[inline]
    pub fn get_view(&self) -> Vec<String> {
        self.pts.iter().cloned().map(|s| s.value).collect()
    }
    #[inline]
    pub fn get_length(&self) -> usize {
        self.pts.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pts.is_empty()
    }
}
