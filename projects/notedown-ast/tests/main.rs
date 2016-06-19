mod fast_test;
#[test]
fn ready() {
    println!("it, works!")
}

use notedown_ast::{SmartLink, TextRange, AST};
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
    show_size!(AST);
    show_size!(TextRange);
    show_size!(SmartLink);
    show_size!(TextRange);
}
