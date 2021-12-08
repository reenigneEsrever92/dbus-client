use std::fmt::Display;

use dbus::{
    arg::messageitem::{MessageItem, MessageItemArray, MessageItemDict},
    Signature, Message,
};
use itertools::Itertools;

use crate::{nom_parser::NomParser, parser::Parser};

#[derive(Debug, PartialEq)]
pub enum Variant {
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
    ObjPath(String),
    Signature(String),
    Array(Vec<Variant>),
    Dictionary(Vec<(Variant, Variant)>),
    FileDescriptor(u32),
}

impl From<&Variant> for MessageItem {
    fn from(variant: &Variant) -> Self {
        match variant {
            Variant::Boolean(value) => MessageItem::Bool(*value),
            Variant::Byte(value) => MessageItem::Byte(*value),
            Variant::Int16(value) => MessageItem::Int16(*value),
            Variant::Int32(value) => MessageItem::Int32(*value),
            Variant::Int64(value) => MessageItem::Int64(*value),
            Variant::Word16(value) => MessageItem::UInt16(*value),
            Variant::Word32(value) => MessageItem::UInt32(*value),
            Variant::Word64(value) => MessageItem::UInt64(*value),
            Variant::Double(value) => MessageItem::Double(*value),
            Variant::Str(value) => MessageItem::Str(value.clone()),
            Variant::ObjPath(_) => todo!(),
            Variant::Signature(_) => todo!(),
            Variant::Array(value) => {
                let items = value
                    .into_iter()
                    .map(|variant| variant.into())
                    .collect_vec();

                MessageItem::Array(
                    MessageItemArray::new(items, variant.into()).unwrap(),
                )
            }
            Variant::Dictionary(value) => {
                let items = value
                    .into_iter()
                    .map(|(key, val)| (key.into(), val.into()))
                    .collect_vec();

                MessageItem::Dict(
                    MessageItemDict::new(
                        items,
                        (&value.first().unwrap().0).into(),
                        (&value.first().unwrap().1).into(),
                    )
                    .unwrap(),
                )
            }
            Variant::FileDescriptor(_) => todo!(),
        }
    }
}

impl<'a> From<&Variant> for Signature<'a> {
    fn from(variant: &Variant) -> Self {
        Signature::new(Into::<String>::into(variant)).unwrap()
    }
}

impl From<&Variant> for String {
    fn from(variant: &Variant) -> Self {
        match variant {
            Variant::Boolean(_) => "b".to_string(),
            Variant::Byte(_) => "y".to_string(),
            Variant::Int16(_) => "n".to_string(),
            Variant::Int32(_) => "i".to_string(),
            Variant::Int64(_) => "x".to_string(),
            Variant::Word16(_) => "q".to_string(),
            Variant::Word32(_) => "u".to_string(),
            Variant::Word64(_) => "t".to_string(),
            Variant::Double(_) => "d".to_string(),
            Variant::Str(_) => "s".to_string(),
            Variant::ObjPath(_) => "o".to_string(),
            Variant::Signature(_) => "g".to_string(),
            Variant::Array(value) => format!("a{}", Into::<String>::into(value.first().unwrap())),
            Variant::Dictionary(value) => {
                format!(
                    "a{{{}{}}}",
                    Into::<String>::into(&value.first().unwrap().0),
                    Into::<String>::into(&value.first().unwrap().1)
                )
            }
            Variant::FileDescriptor(_) => "h".to_string(),
        }
    }
}

impl From<&str> for Variant {
    fn from(str: &str) -> Self {
        NomParser::parse(str).unwrap()
    }
}

#[cfg(test)]
mod test {
    use dbus::Signature;

    use crate::variant::Variant;

    #[test]
    fn test_conversions() {
        assert_eq!(
            Into::<Signature>::into(&Variant::from("[test, test2]")),
            Signature::new(String::from("as")).unwrap()
        );
        assert_eq!(
            Into::<Signature>::into(&Variant::from("{test: 2i32}")),
            Signature::new(String::from("a{si}")).unwrap()
        );
    }
}
