use itertools::Itertools;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Word16(u16),
    Word32(u32),
    Word64(u64),
    Double(f64),
    Str(String),
    Vec(Vec<Value>)
}


#[derive(Parser)]
#[grammar = "argument.pest"]
struct ValueParser;

impl From<&str> for Value {
    fn from(str: &str) -> Self {
        let rule = ValueParser::parse(Rule::argument, str)
            .expect("Invalid Value")
            .next()
            .unwrap();

        convert_rule(rule)
    }
}

fn convert_rule(rule: Pair<Rule>) -> Value {
    match rule.as_rule() {
        Rule::argument => convert_rule(rule.into_inner().next().unwrap()),
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
        Rule::value => convert_rule(rule.into_inner().next().unwrap()),
        Rule::BOOLEAN => Value::Boolean(rule.as_str().parse().unwrap()),
        Rule::BYTE => Value::Byte(u8::from_str_radix(rule.as_str().trim_end_matches("y"), 16).unwrap()),
        Rule::INT_16 => Value::Int16(rule.as_str().trim_end_matches("n").parse().unwrap()),
        Rule::INT_32 => Value::Int32(rule.as_str().trim_end_matches("i").parse().unwrap()),
        Rule::INT_64 => Value::Int64(rule.as_str().trim_end_matches("x").parse().unwrap()),
        Rule::U_INT_16 => Value::Word16(rule.as_str().trim_end_matches("q").parse().unwrap()),
        Rule::U_INT_32 => Value::Word32(rule.as_str().trim_end_matches("u").parse().unwrap()),
        Rule::U_INT_64 => Value::Word64(rule.as_str().trim_end_matches("t").parse().unwrap()),
        Rule::DOUBLE => Value::Double(rule.as_str().trim_end_matches("d").parse().unwrap()),
        Rule::STRING => Value::Str(rule.as_str().replace("\"", "").to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::value::Value;

    #[test]
    fn test_parse() {
        let value: Value = "{ \"test\": -8i }".into();
        assert_eq!(value, Value::Vec(vec![Value::Str("test".to_string()), Value::Int32(-8)]));

        assert_eq!(Into::<Value>::into("-1.9d"), Value::Double(-1.9));
        assert_eq!(Into::<Value>::into("ffy"), Value::Byte(255u8));
        assert_eq!(Into::<Value>::into("fey"), Value::Byte(254u8));
    }

}