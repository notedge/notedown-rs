use crate::{Result, Value};
use indexmap::IndexMap;

/// The global function type definition
pub trait Function: Sync + Send {
    /// The global function type definition
    fn apply(&self, args: &[Value], options: &IndexMap<String, Value>) -> Result<Value>;
}

impl<F> Function for F
where
    F: Fn(&[Value], &IndexMap<String, Value>) -> Result<Value> + Sync + Send,
{
    fn apply(&self, args: &[Value], options: &IndexMap<String, Value>) -> Result<Value> {
        self(args, options)
    }
}

#[allow(dead_code)]
fn print_input(args: &[Value], options: &IndexMap<String, Value>) -> Result<Value> {
    println!("{:?}", args);
    println!("{:?}", options);
    Ok(Value::Null)
}

#[test]
fn test_apply() {
    let arg = vec![Value::Null];
    let mut options = IndexMap::new();
    options.insert(String::from("first"), Value::from(1));
    print_input.apply(&arg, &options).unwrap();
}
