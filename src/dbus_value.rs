use itertools::Itertools;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::dbus_type::DBusType;

#[derive(Debug, PartialEq)]
pub enum DBusValue {
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
    Vec(Vec<DBusValue>),
    Unit,
}

#[derive(Parser)]
#[grammar = "dbus_value.pest"]
struct ValueParser;

impl From<&str> for DBusValue {
    fn from(str: &str) -> Self {
        if str.is_empty() {
            DBusValue::Unit
        } else {
            let rule = ValueParser::parse(Rule::dbus_value, str)
                .expect("Invalid Value")
                .next()
                .unwrap();

            convert_rule(rule)
        }
    }
}

impl DBusValue {
    pub fn is_type(&self, typ: DBusType) -> bool {
        match self {
            DBusValue::Boolean(_) => {
                if let DBusType::Boolean = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Byte(_) => {
                if let DBusType::Byte = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Int16(_) => {
                if let DBusType::Int16 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Int32(_) => {
                if let DBusType::Int32 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Int64(_) => {
                if let DBusType::Int64 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::UInt16(_) => {
                if let DBusType::UInt16 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::UInt32(_) => {
                if let DBusType::UInt32 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::UInt64(_) => {
                if let DBusType::UInt64 = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Double(_) => {
                if let DBusType::Double = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::String(_) => {
                if let DBusType::String = typ {
                    true
                } else {
                    false
                }
            }
            DBusValue::Vec(_) => match typ {
                DBusType::Struct(_) => true,
                DBusType::Array { value_type: _ } => true,
                DBusType::Dictionary {
                    key_type: _,
                    value_type: _,
                } => true,
                _ => false,
            },
            DBusValue::Unit => {
                if let DBusType::Unit = typ {
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn convert_rule(rule: Pair<Rule>) -> DBusValue {
    match rule.as_rule() {
        Rule::dbus_value => convert_rule(rule.into_inner().next().unwrap()),
        Rule::array => DBusValue::Vec(
            rule.into_inner()
                .map(|inner_rule| convert_rule(inner_rule))
                .collect_vec(),
        ),
        Rule::struct_t => DBusValue::Vec(
            rule.into_inner()
                .map(|inner_rule| convert_rule(inner_rule))
                .collect_vec(),
        ),
        Rule::dictionary => DBusValue::Vec(
            rule.into_inner()
                .map(|inner_rule| convert_rule(inner_rule))
                .collect_vec(),
        ),
        Rule::BOOLEAN => DBusValue::Boolean(rule.as_str().parse().unwrap()),
        Rule::BYTE => {
            DBusValue::Byte(u8::from_str_radix(rule.as_str().trim_end_matches("y"), 16).unwrap())
        }
        Rule::INT_16 => DBusValue::Int16(rule.as_str().trim_end_matches("n").parse().unwrap()),
        Rule::INT_32 => DBusValue::Int32(rule.as_str().trim_end_matches("i").parse().unwrap()),
        Rule::INT_64 => DBusValue::Int64(rule.as_str().trim_end_matches("x").parse().unwrap()),
        Rule::U_INT_16 => DBusValue::UInt16(rule.as_str().trim_end_matches("q").parse().unwrap()),
        Rule::U_INT_32 => DBusValue::UInt32(rule.as_str().trim_end_matches("u").parse().unwrap()),
        Rule::U_INT_64 => DBusValue::UInt64(rule.as_str().trim_end_matches("t").parse().unwrap()),
        Rule::DOUBLE => DBusValue::Double(rule.as_str().trim_end_matches("d").parse().unwrap()),
        Rule::STRING => DBusValue::String(rule.as_str().replace("\"", "").to_string()),
    }
}

#[cfg(test)]
mod test {
    use crate::dbus_value::DBusValue;

    #[test]
    fn test_parse() {
        let value: DBusValue = "{ \"test\": -8i }".into();
        assert_eq!(
            value,
            DBusValue::Vec(vec![
                DBusValue::String("test".to_string()),
                DBusValue::Int32(-8)
            ])
        );

        assert_eq!(Into::<DBusValue>::into("-1.9d"), DBusValue::Double(-1.9));
        assert_eq!(Into::<DBusValue>::into("ffy"), DBusValue::Byte(255u8));
        assert_eq!(Into::<DBusValue>::into("fey"), DBusValue::Byte(254u8));
        assert_eq!(
            Into::<DBusValue>::into(
                "launch-new-instance@gnome-shell-extensions.gcampax.github.com"
            ),
            DBusValue::String(
                "launch-new-instance@gnome-shell-extensions.gcampax.github.com".into()
            )
        );
        assert_eq!(
            Into::<DBusValue>::into(
                "(8i,some@string)"
            ),
            DBusValue::Vec(
                vec![
                    DBusValue::Int32(8),
                    DBusValue::String("some@string".into())
                ]
            )
        );
    }
}
