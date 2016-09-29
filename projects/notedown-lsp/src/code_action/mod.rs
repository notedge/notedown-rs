use tower_lsp::lsp_types::*;

pub fn code_action_provider(p: CodeActionParams) -> CodeActionResponse {
    let _ = p;
    return vec![]
}