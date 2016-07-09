use notedown_ast::{ASTKind, ASTNode, ListView, SmartLink, TableView};
use std::mem::size_of;

macro_rules! show_size {
    () => {
        println!("{:<22} {:>4}    {}", "Type", "T", "Option<T>");
    };
    ($t:ty) => {
        println!("{:<22} {:4} {:>12}", stringify!($t), size_of::<$t>(), size_of::<Option<$t>>())
    };
}

#[test]
fn get_size() {
    show_size!();
    show_size!(ASTNode<()>);
    show_size!(ASTNode<()>);
    show_size!(ASTKind<ASTNode<()>>);
    show_size!(ListView<()>);
    show_size!(TableView<()>);
    show_size!(SmartLink);

    assert_eq!(size_of::<ASTNode<()>>(), 32);
    assert_eq!(size_of::<ASTKind<ASTNode<()>>>(), 32);
}
