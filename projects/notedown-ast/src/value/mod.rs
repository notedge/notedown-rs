use num::BigInt;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use std::cmp::Ordering;
use num::BigUint;

mod debug;
mod from;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PrimitiveValue {
    Null,
    Boolean(bool),
    Integer(BigInt),
    Decimal([u8; 8]),
    String(String),
}

#[derive(Clone, Debug)]
pub enum Value {
    Atom(Literal<PrimitiveValue>),
    Set(BTreeSet<Literal<Value>>),
    List(BTreeMap<usize, Literal<Value>>),
    Dict(BTreeMap<Literal<String>, Literal<Value>>),
}

#[derive(Clone, Debug)]
pub struct Literal<T> {
    value: T,
    range: Option<(usize, usize)>,
}


impl<T: Hash> Hash for Literal<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}


