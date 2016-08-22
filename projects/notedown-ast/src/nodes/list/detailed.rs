use super::*;
/// ## Details List
/// ```note
/// >- summary part
/// >- sum
/// >  part2
/// >  part3
/// ```
///
/// ```html
/// <details open>
/// <summary>Want to ruin the surprise?</summary>
/// <br>
/// Well, you asked for it!
/// </details>
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ListDetailedNode {
    is_open: bool,
    summary: Literal<ListItem>,
    body: Vec<Literal<ListItem>>,
}
