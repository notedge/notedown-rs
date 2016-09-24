use super::*;

impl Display for QuoteBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "< quote>")?;
        writeln!(f, "</quote>")
    }
}
