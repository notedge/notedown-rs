use super::*;
use std::cmp::Ordering;

#[derive(Clone, Eq, Ord)]
pub struct Literal<T> {
    ///
    pub value: T,
    //
    pub range: Option<(u32, u32)>,
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

impl<T: PartialOrd> PartialOrd for Literal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

fn test() {
    use lsp_document::{TextMap, TextAdapter, Pos, IndexedText};
    use lsp_types::Position;

// Character width
// U16:     1111111111111 1111111111 1 11 1 1 111111111 21
// U8:      1111111111111 1222122221 1 13 3 3 111111111 41
// U8 offset
//          0         1       2      3       4          5
//          0123456789012 3468013579 0 12 5 8 123456789 04
    let text = "Hello, world!\nКак дела?\r\n做得好\nThis is 💣!";
    let text = IndexedText::new(text);
//
// Examples of using TextMap methods
//
// Pos of 💣 from its offset
    assert_eq!(text.offset_to_pos(50).unwrap(), Pos::new(3, 8));
// Raw line range info
    assert_eq!(text.line_range(2).unwrap(), Pos::new(2, 0)..Pos::new(2, 10));
// Extracting part of text between two positions
    assert_eq!(text.substr(Pos::new(1, 7)..Pos::new(1, 15)).unwrap(), "дела");

//
// Example of using TextAdapter methods
//
// Pos of `!` after 💣
    assert_eq!(text.lsp_pos_to_pos(&Position::new(3, 10)).unwrap(), Pos::new(3, 12));
    assert_eq!(text.pos_to_lsp_pos(&Pos::new(3, 12)).unwrap(), Position::new(3, 10));
}