use crate::parser::ASTNode;
use notedown_ast::{nodes::ListItem, ASTNodes};

/// ident:usize
/// kind: &str
/// term: ASTNodes
pub fn regroup_list_view(lists: Vec<(usize, &str, ASTNodes)>) -> Vec<ASTNode> {
    println!("{:#?}", lists);
    ListItem { prefix: Default::default(), rest: vec![] };

    todo!()
}

pub fn regroup_table_view(table: &[Vec<Vec<ASTNode>>]) -> Vec<ASTNode> {
    for line in table {
        for item in line {
            println!("{:?}", item)
        }
    }
    todo!()
}
