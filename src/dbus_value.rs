use itertools::Itertools;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::dbus_type::DBusType;

#[derive(Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Double(f64),
    String(String),
    Vec(Vec<Value>)
}


#[derive(Parser)]
#[grammar = "dbus_value.pest"]
struct ValueParser;

impl From<&str> for Value {
    fn from(str: &str) -> Self {
        let rule = ValueParser::parse(Rule::dbus_value, str)
            .expect("Invalid Value")
            .next()
            .unwrap();

        convert_rule(rule)
    }
}

impl Value {
    pub fn is_type(&self, typ: DBusType) -> bool {
        match self {
            Value::Boolean(_) => if let DBusType::Boolean = typ { true } else { false },
            Value::Byte(_) => if let DBusType::Byte = typ { true } else { false },
            Value::Int16(_) => if let DBusType::Int16 = typ { true } else { false },
            Value::Int32(_) => if let DBusType::Int32 = typ { true } else { false },
            Value::Int64(_) => if let DBusType::Int64 = typ { true } else { false },
            Value::UInt16(_) => if let DBusType::UInt16 = typ { true } else { false },
            Value::UInt32(_) => if let DBusType::UInt32 = typ { true } else { false },
            Value::UInt64(_) => if let DBusType::UInt64 = typ { true } else { false },
            Value::Double(_) => if let DBusType::Double = typ { true } else { false },
            Value::String(_) => if let DBusType::String = typ { true } else { false },
            Value::Vec(_) => match typ {
                DBusType::Struct(_) => true,
                DBusType::Array { value_type: _ } => true,
                DBusType::Dictionary { key_type: _, value_type: _ } => true,
                _ => false
            },
        }
    }
}

fn convert_rule(rule: Pair<Rule>) -> Value {
    match rule.as_rule() {
        Rule::dbus_value => convert_rule(rule.into_inner().next().unwrap()),
        Rule::array => Value::Vec(
            rule.into_inner().map(|inner_rule| {
                convert_rule(inner_rule)
            })
            .collect_vec()
        ),
        Rule::struct_t => Value::Vec(
            rule.into_inner().map(|inner_rule| {
                convert_rule(inner_rule)
            })
            .collect_vec()
        ),
        Rule::dictionary => Value::Vec(
            rule.into_inner().map(|inner_rule| {
                convert_rule(inner_rule)
            })
            .collect_vec()
        ),
        Rule::BOOLEAN => Value::Boolean(rule.as_str().parse().unwrap()),
        Rule::BYTE => Value::Byte(u8::from_str_radix(rule.as_str().trim_end_matches("y"), 16).unwrap()),
        Rule::INT_16 => Value::Int16(rule.as_str().trim_end_matches("n").parse().unwrap()),
        Rule::INT_32 => Value::Int32(rule.as_str().trim_end_matches("i").parse().unwrap()),
        Rule::INT_64 => Value::Int64(rule.as_str().trim_end_matches("x").parse().unwrap()),
        Rule::U_INT_16 => Value::UInt16(rule.as_str().trim_end_matches("q").parse().unwrap()),
        Rule::U_INT_32 => Value::UInt32(rule.as_str().trim_end_matches("u").parse().unwrap()),
        Rule::U_INT_64 => Value::UInt64(rule.as_str().trim_end_matches("t").parse().unwrap()),
        Rule::DOUBLE => Value::Double(rule.as_str().trim_end_matches("d").parse().unwrap()),
        Rule::STRING => Value::String(rule.as_str().replace("\"", "").to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::dbus_value::Value;

    #[test]
    fn test_parse() {
        let value: Value = "{ \"test\": -8i }".into();
        assert_eq!(value, Value::Vec(vec![Value::String("test".to_string()), Value::Int32(-8)]));

        assert_eq!(Into::<Value>::into("-1.9d"), Value::Double(-1.9));
        assert_eq!(Into::<Value>::into("ffy"), Value::Byte(255u8));
        assert_eq!(Into::<Value>::into("fey"), Value::Byte(254u8));
    }

}