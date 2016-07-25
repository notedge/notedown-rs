use super::*;



impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Null => { f.write_str("Null") }
            ValueType::Boolean => { f.write_str("Boolean") }
            ValueType::Integer => { f.write_str("Integer") }
            ValueType::Decimal => { f.write_str("Decimal") }
            ValueType::String => { f.write_str("String") }
            ValueType::Set(v) => {
                if v.is_empty() {
                    return f.write_str("Set<?>");
                }
                let t = v.iter().map(|e| e.to_string()).collect_vec().join(" | ");
                write!(f, "Set<{}, {}>", t, v.len())
            }
            ValueType::List(v) => {
                if v.is_empty() {
                    return f.write_str("List<?>");
                }
                let t = v.iter().map(|e| e.to_string()).join(" | ");
                write!(f, "List<{}, {}>", t, v.len())
            }
            ValueType::Object(_) => { unimplemented!() }
        }
    }
}