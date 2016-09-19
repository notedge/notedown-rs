use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct ExternalCommand {
    pub cmd: String,
    pub data: Vec<u8>,
}
