use std::ops::Deref;

use dbus::{
    Signature as DbusSignature,
};
use itertools::Itertools;

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
    Variant(Box<DBusType>),
}

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
            DBusType::Variant(_) => "v".to_string(),
        }
    }
}

impl From<&str> for DBusType {
    fn from(str: &str) -> Self {
    }
}

#[cfg(test)]
mod test {
    use dbus::Signature as DbusSignature;

    use crate::dbus_type::DBusType;

    #[test]
    fn test_conversions() {
        assert_eq!(Into::<String>::into(&DBusType::String), "s".to_string());

        assert_eq!(
            Into::<String>::into(&DBusType::Variant(Box::new(DBusType::String))),
            "v".to_string()
        );

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
                DBusType::Array{ value_type: DBusType::String.into() }
            ])),
            "(sxas)".to_string()
        );
    }
}