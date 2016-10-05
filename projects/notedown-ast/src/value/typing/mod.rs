use super::*;

/// Type of [`Value`]
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ValueType {
    /// It doesn't look like anything to me
    Null,
    /// `true` or `false`
    Boolean,
    /// Arbitrarily large integer
    Integer,
    /// 128-bit fixed point decimal
    Decimal,
    /// A UTF-8–encoded string
    String,
    /// Ordered set of values
    Set(BTreeSet<ValueType>),
    /// Array of values
    List(BTreeSet<ValueType>),
    /// Ordered map of key value pairs
    Object(BTreeMap<String, ValueType>),
}

impl Value {
    /// get type of the value
    pub fn get_type(&self) -> ValueType {
        match self {
            Self::Null => ValueType::Null,
            Self::Boolean(_) => ValueType::Boolean,
            Self::Integer(_) => ValueType::Integer,
            Self::Decimal(_) => ValueType::Decimal,
            Self::String(_) => ValueType::String,
            Self::Set(v) => self.check_set_type(v),
            Self::Array(v) => self.check_list_type(v),
            Self::Object(v) => self.check_dict_type(v),
        }
    }
    /// get type name of the value
    pub fn get_type_name(&self) -> String {
        self.get_type().to_string()
    }
    fn check_set_type(&self, input: &OrderedSet) -> ValueType {
        let mut count = BTreeSet::new();
        for v in input {
            count.insert(v.value.get_type());
        }
        ValueType::Set(count)
    }
    fn check_list_type(&self, input: &SparseArray) -> ValueType {
        let mut count = BTreeSet::new();
        for v in input.values() {
            count.insert(v.value.get_type());
        }
        ValueType::List(count)
    }
    fn check_dict_type(&self, input: &OrderedMap) -> ValueType {
        // let input: BTreeMap<String, Literal<Value>>;
        let mut count = BTreeMap::new();
        for (k, v) in input {
            count.insert(k.to_owned(), v.value.get_type());
        }
        ValueType::Object(count)
    }
}
