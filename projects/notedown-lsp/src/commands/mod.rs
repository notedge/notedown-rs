use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use tower_lsp::lsp_types::*;
use serde_json::Value;


pub fn command_provider(p: ExecuteCommandParams) -> Option<Value> {
    let _ = p;
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if let Ok(s) = ctx.get_contents() {
        return Some(Value::String(s))
    }
    return None
}
