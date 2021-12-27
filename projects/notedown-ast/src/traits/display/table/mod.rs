use super::*;

impl Debug for SimpleTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let w = &mut f.debug_struct("Table");
        todo!();
        w.finish()
    }
}

impl Display for SimpleTable {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
