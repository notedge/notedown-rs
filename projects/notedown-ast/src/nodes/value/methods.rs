use super::*;
use itertools::Itertools;


impl Value {
    pub fn type_name(&self) -> String {
        match self {
            Value::Null => {String::from("Null")}
            Value::Boolean(_) => {String::from("Boolean")}
            Value::Integer(_) => {String::from("Integer")}
            Value::Decimal(_) => {String::from("Decimal")}
            Value::String(_) => {String::from("String")}
            Value::Set(v) => {self.check_set_type(v)}
            Value::Array(v) => {self.check_list_type(v)}
            Value::Object(v) => {self.check_dict_type(v)}
        }
    }
    fn check_set_type(&self, input: &Set<Literal<Value>>) -> String {
        let mut count = Set::new();
        for v in input {
            count.insert(v.type_name())
        }
        format!("Set<{}>", count.join(" | "))
    }
    fn check_list_type(&self, input: &Map<Literal<BigUint>, Literal<Value>>) {

    }
    fn check_dict_type(&self, input: &Map<Literal<String>, Literal<Value>>) {

    }
}