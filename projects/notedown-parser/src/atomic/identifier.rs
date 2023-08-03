use super::*;

#[rustfmt::skip]
pub static IDENTIFIER: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r"^(?x)(
    \p{XID_START}[\p{XID_Continue}&&[^_＿]]+
)").unwrap()});

impl NoteParser for IdentifierNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&IDENTIFIER, "IDENTIFIER")?;
        let id = IdentifierNode::new(m.as_str(), get_span(input, state));
        state.finish(id)
    }
}

#[rustfmt::skip]
pub static LIGATURE: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r"^(?x)(
    != # ≠
|   >= # ⩾
|   <= # ⩽
|   \+- # ±
|   -\+ # ∓
|   -> # →
|   => # ⇒
|   ->> # ↠
|   >-> # ↣
|   \|-> # ↦
|   ~>> # ⇝
|   ~> # \leadsto
)").unwrap()});

impl NoteParser for LigatureNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, m) = input.match_regex(&LIGATURE, "LIGATURE")?;
        let id = LigatureNode::new(m.as_str(), get_span(input, state));
        state.finish(id)
    }
}
