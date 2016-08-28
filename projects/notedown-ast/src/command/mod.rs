mod options;

pub use options::CommandOptions;

pub struct Command {
    namespace: Vec<String>,
    name: String,
}
