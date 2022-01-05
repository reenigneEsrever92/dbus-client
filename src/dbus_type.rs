use std::ops::Deref;

use dbus::Signature as DbusSignature;
use itertools::Itertools;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{dbus_value::DBusValue, dbus_error::DBusError};

#[derive(Debug, PartialEq)]
pub enum DBusType {
    Boolean,
    Byte,
    Int16,
    Int32,
    Int64,
    UInt16,
    UInt32,
    UInt64,
    Double,
    String,
    ObjPath,
    Signature,
    FileDescriptor,
    Struct(Vec<DBusType>),
    Array {
        value_type: Box<DBusType>,
    },
    Dictionary {
        key_type: Box<DBusType>,
        value_type: Box<DBusType>,
    },
    Variant,
}

impl DBusType {
    pub fn is_valid_value(&self, val: &DBusValue) -> Result<(), DBusError> {
        match self {
            DBusType::Boolean => {
                if let DBusValue::Boolean(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected boolean got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Byte => {
                if let DBusValue::Byte(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected byte got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Int16 => {
                if let DBusValue::Int16(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected int16 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Int32 => {
                if let DBusValue::Int32(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected int32 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Int64 => {
                if let DBusValue::Int64(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected int64 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::UInt16 => {
                if let DBusValue::UInt16(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected uint16 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::UInt32 => {
                if let DBusValue::UInt32(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected uint32 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::UInt64 => {
                if let DBusValue::UInt64(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected uint64 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Double => {
                if let DBusValue::Double(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected double got: {:?}",
                        val
                    )))
                }
            }
            DBusType::String => {
                if let DBusValue::String(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected string got: {:?}",
                        val
                    )))
                }
            }
            DBusType::ObjPath => {
                if let DBusValue::String(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected string got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Signature => {
                if let DBusValue::String(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected string got: {:?}",
                        val
                    )))
                }
            }
            DBusType::FileDescriptor => {
                if let DBusValue::UInt32(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected uint32 got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Struct(_) => {
                if let DBusValue::Vec(_) = val {
                    Ok(())
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected vec got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Array { value_type: _ } => {
                if let DBusValue::Vec(_vec) = val {
                    todo!("Check that all values have the same type")
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected vec got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Dictionary {
                key_type,
                value_type,
            } => {
                if let DBusValue::Vec(vec) = val {
                    vec.iter()
                        .step_by(2)
                        .map(|inner_val| key_type.is_valid_value(inner_val))
                        .find(|inner_val| *inner_val != Ok(()))
                        .map_or(Ok(()), |error| Err(DBusError::InvalidValue(format!("Wrong dictionary key"))))
                } else {
                    Err(DBusError::InvalidValue(format!(
                        "Expected vec got: {:?}",
                        val
                    )))
                }
            }
            DBusType::Variant => Ok(()),
        }
    }
}

#[derive(Parser)]
#[grammar = "dbus_type.pest"]
pub struct DBusTypeParser;

impl<'a> From<&DBusType> for DbusSignature<'a> {
    fn from(variant: &DBusType) -> Self {
        DbusSignature::new(Into::<String>::into(variant)).unwrap()
    }
}

impl From<&DBusType> for String {
    fn from(variant: &DBusType) -> Self {
        match variant {
            DBusType::Boolean => "b".to_string(),
            DBusType::Byte => "y".to_string(),
            DBusType::Int16 => "n".to_string(),
            DBusType::Int32 => "i".to_string(),
            DBusType::Int64 => "x".to_string(),
            DBusType::UInt16 => "q".to_string(),
            DBusType::UInt32 => "u".to_string(),
            DBusType::UInt64 => "t".to_string(),
            DBusType::Double => "d".to_string(),
            DBusType::String => "s".to_string(),
            DBusType::ObjPath => "o".to_string(),
            DBusType::Signature => "g".to_string(),
            DBusType::FileDescriptor => "h".to_string(),
            DBusType::Variant => "v".to_string(),
            DBusType::Struct(value_types) => format!(
                "({})",
                value_types.iter().map(|v| Into::<String>::into(v)).join("")
            ),
            DBusType::Array { value_type } => {
                format!("a{}", Into::<String>::into(value_type.deref()))
            }
            DBusType::Dictionary {
                key_type,
                value_type,
            } => {
                format!(
                    "a{{{}{}}}",
                    Into::<String>::into(key_type.deref()),
                    Into::<String>::into(value_type.deref())
                )
            }
        }
    }
}

impl From<&str> for DBusType {
    fn from(str: &str) -> DBusType {
        let ast = DBusTypeParser::parse(Rule::dbus_type, str)
            .expect("Invalid Signature")
            .next()
            .unwrap();

        convert_rule(ast.into_inner().next().unwrap())
    }
}

fn convert_rule(rule: Pair<Rule>) -> DBusType {
    match rule.as_rule() {
        Rule::dbus_type => convert_rule(rule.into_inner().next().unwrap()),
        Rule::array => DBusType::Array {
            value_type: Box::new(convert_rule(rule.into_inner().next().unwrap())),
        },
        Rule::struct_t => DBusType::Struct(
            rule.into_inner()
                .map(|inner_rule| convert_rule(inner_rule))
                .collect_vec(),
        ),
        Rule::dictionary => {
            let mut inner_rule = rule.into_inner();

            DBusType::Dictionary {
                key_type: Box::new(convert_rule(inner_rule.next().unwrap())),
                value_type: Box::new(convert_rule(inner_rule.next().unwrap())),
            }
        }
        Rule::BOOLEAN => DBusType::Boolean,
        Rule::BYTE => DBusType::Byte,
        Rule::INT_16 => DBusType::Int16,
        Rule::INT_32 => DBusType::Int32,
        Rule::INT_64 => DBusType::Int64,
        Rule::U_INT_16 => DBusType::UInt16,
        Rule::U_INT_32 => DBusType::UInt32,
        Rule::U_INT_64 => DBusType::UInt64,
        Rule::DOUBLE => DBusType::Double,
        Rule::STRING => DBusType::String,
        Rule::OBJ_PATH => DBusType::ObjPath,
        Rule::SIGNATURE => DBusType::Signature,
        Rule::FILE_DESCRIPTOR => DBusType::FileDescriptor,
        Rule::VARIANT => DBusType::Variant,
    }
}

#[cfg(test)]
mod test {
    use dbus::Signature as DbusSignature;

    use crate::dbus_type::DBusType;

    #[test]
    fn test_conversions() {
        assert_eq!(Into::<String>::into(&DBusType::String), "s".to_string());

        assert_eq!(Into::<String>::into(&DBusType::Variant), "v".to_string());

        assert_eq!(
            Into::<String>::into(&DBusType::Array {
                value_type: Box::new(DBusType::String)
            }),
            "as".to_string()
        );

        assert_eq!(
            Into::<String>::into(&DBusType::Struct(vec![
                DBusType::String,
                DBusType::Int64,
                DBusType::Array {
                    value_type: DBusType::String.into()
                }
            ])),
            "(sxas)".to_string()
        );
    }

    #[test]
    fn test_invert() {
        assert_eq!(
            Into::<DBusType>::into("(si)"),
            DBusType::Struct(vec![DBusType::String, DBusType::Int32])
        );
    }
}
