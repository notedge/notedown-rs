use notedown_ast::ASTNode;

pub struct CommandDefinition {
    name: String,
}

#[test]
fn test() {
    let _ = CommandDefinition { name: "sort".to_string() };
}
