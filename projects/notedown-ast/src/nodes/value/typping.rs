use super::*;



#[derive(Clone, Debug)]
pub enum ValueType {
    /// - `None`: It doesn't look like anything to me
    Null,
    Boolean,
    Integer,
    Decimal,
    String,
    Set(Set<ValueType>),
    Array(Set<ValueType>),
    Object(Map<Literal<String>, Literal<Value>>),
}

impl From<Value> for ValueType {
    fn from(_: Value) -> Self {
        todo!()
    }
}