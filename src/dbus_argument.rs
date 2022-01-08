use dbus::arg::messageitem::{MessageItem, MessageItemArray, MessageItemDict};
use itertools::Itertools;

use crate::{dbus_error::DBusError, dbus_type::DBusType, dbus_value::DBusValue};

pub struct DBusArgument<'a> {
    pub dbus_type: &'a DBusType,
    pub dbus_value: &'a DBusValue,
}

impl<'a> DBusArgument<'a> {
    pub fn validate(self) -> Result<DBusArgument<'a>, DBusError> {
        self.dbus_type
            .is_valid_value(&self.dbus_value)
            .map(|_| self)
    }
}

impl<'a> From<DBusArgument<'a>> for Option<MessageItem> {
    fn from(arg: DBusArgument) -> Self {
        match arg.dbus_type {
            DBusType::Boolean => {
                if let DBusValue::Boolean(value) = arg.dbus_value {
                    Some(MessageItem::Bool(*value))
                } else {
                    panic!("Expected argument of type Boolean got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Byte => {
                if let DBusValue::Byte(value) = arg.dbus_value {
                    Some(MessageItem::Byte(*value))
                } else {
                    panic!("Expected argument of type Byte got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Int16 => {
                if let DBusValue::Int16(value) = arg.dbus_value {
                    Some(MessageItem::Int16(*value))
                } else {
                    panic!("Expected argument of type Int16 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Int32 => {
                if let DBusValue::Int32(value) = arg.dbus_value {
                    Some(MessageItem::Int32(*value))
                } else {
                    panic!("Expected argument of type Int32 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Int64 => {
                if let DBusValue::Int64(value) = arg.dbus_value {
                    Some(MessageItem::Int64(*value))
                } else {
                    panic!("Expected argument of type Int64 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::UInt16 => {
                if let DBusValue::UInt16(value) = arg.dbus_value {
                    Some(MessageItem::UInt16(*value))
                } else {
                    panic!("Expected argument of type UInt16 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::UInt32 => {
                if let DBusValue::UInt32(value) = arg.dbus_value {
                    Some(MessageItem::UInt32(*value))
                } else {
                    panic!("Expected argument of type UInt32 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::UInt64 => {
                if let DBusValue::UInt64(value) = arg.dbus_value {
                    Some(MessageItem::UInt64(*value))
                } else {
                    panic!("Expected argument of type UInt64 got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Double => {
                if let DBusValue::Double(value) = arg.dbus_value {
                    Some(MessageItem::Double(*value))
                } else {
                    panic!("Expected argument of type Double got: {:?}", arg.dbus_value)
                }
            }
            DBusType::String => {
                if let DBusValue::String(value) = arg.dbus_value {
                    Some(MessageItem::Str(value.clone()))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::ObjPath => {
                if let DBusValue::String(value) = arg.dbus_value {
                    Some(MessageItem::ObjectPath(value.clone().into()))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Signature => {
                if let DBusValue::String(value) = arg.dbus_value {
                    Some(MessageItem::Signature(value.clone().into()))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::FileDescriptor => {
                if let DBusValue::String(_) = arg.dbus_value {
                    todo!()
                } else {
                    panic!()
                }
            }
            DBusType::Struct(types) => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    Some(MessageItem::Struct(
                        types
                            .iter()
                            .zip(values.iter())
                            .map(|pair| {
                                Into::<Option<MessageItem>>::into(DBusArgument {
                                    dbus_type: pair.0,
                                    dbus_value: pair.1,
                                })
                                .unwrap()
                            })
                            .collect_vec(),
                    ))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Array { value_type } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    Some(MessageItem::Array(
                        MessageItemArray::new(
                            values
                                .iter()
                                .map(|value| {
                                    Into::<Option<MessageItem>>::into(DBusArgument {
                                        dbus_type: value_type,
                                        dbus_value: value,
                                    })
                                    .unwrap()
                                })
                                .collect_vec(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    ))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Dictionary {
                key_type,
                value_type,
            } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    Some(MessageItem::Dict(
                        MessageItemDict::new(
                            values
                                .iter()
                                .step_by(2)
                                .zip(values.iter().skip(1).step_by(2))
                                .map(|entry| {
                                    (
                                        Into::<Option<MessageItem>>::into(DBusArgument {
                                            dbus_type: key_type,
                                            dbus_value: entry.0,
                                        })
                                        .unwrap(),
                                        Into::<Option<MessageItem>>::into(DBusArgument {
                                            dbus_type: value_type,
                                            dbus_value: entry.1,
                                        })
                                        .unwrap(),
                                    )
                                })
                                .collect_vec(),
                            Into::<String>::into(key_type.as_ref()).into(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    ))
                } else {
                    panic!("Expected argument of type String got: {:?}", arg.dbus_value)
                }
            }
            DBusType::Variant => todo!(),
            DBusType::Unit => None,
        }
    }
}
