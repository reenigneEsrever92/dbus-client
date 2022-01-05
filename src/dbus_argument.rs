use dbus::arg::messageitem::{MessageItem, MessageItemArray, MessageItemDict};
use itertools::Itertools;

use crate::{dbus_type::DBusType, dbus_value::DBusValue, dbus_error::DBusError};


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

impl<'a> From<DBusArgument<'a>> for MessageItem {
    fn from(arg: DBusArgument) -> Self {
        match arg.dbus_type {
            DBusType::Boolean => {
                if let DBusValue::Boolean(value) = arg.dbus_value {
                    MessageItem::Bool(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Byte => {
                if let DBusValue::Byte(value) = arg.dbus_value {
                    MessageItem::Byte(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int16 => {
                if let DBusValue::Int16(value) = arg.dbus_value {
                    MessageItem::Int16(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int32 => {
                if let DBusValue::Int32(value) = arg.dbus_value {
                    MessageItem::Int32(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Int64 => {
                if let DBusValue::Int64(value) = arg.dbus_value {
                    MessageItem::Int64(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt16 => {
                if let DBusValue::UInt16(value) = arg.dbus_value {
                    MessageItem::UInt16(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt32 => {
                if let DBusValue::UInt32(value) = arg.dbus_value {
                    MessageItem::UInt32(*value)
                } else {
                    panic!()
                }
            }
            DBusType::UInt64 => {
                if let DBusValue::UInt64(value) = arg.dbus_value {
                    MessageItem::UInt64(*value)
                } else {
                    panic!()
                }
            }
            DBusType::Double => {
                if let DBusValue::Double(value) = arg.dbus_value {
                    MessageItem::Double(*value)
                } else {
                    panic!()
                }
            }
            DBusType::String => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::Str(value.clone())
                } else {
                    panic!()
                }
            }
            DBusType::ObjPath => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::ObjectPath(value.clone().into())
                } else {
                    panic!()
                }
            }
            DBusType::Signature => {
                if let DBusValue::String(value) = arg.dbus_value {
                    MessageItem::Signature(value.clone().into())
                } else {
                    panic!()
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
                    MessageItem::Struct(
                        types
                            .iter()
                            .zip(values.iter())
                            .map(|pair| {
                                DBusArgument {
                                    dbus_type: pair.0,
                                    dbus_value: pair.1,
                                }
                                .into()
                            })
                            .collect_vec(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Array { value_type } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    MessageItem::Array(
                        MessageItemArray::new(
                            values
                                .iter()
                                .map(|value| {
                                    DBusArgument {
                                        dbus_type: value_type,
                                        dbus_value: value,
                                    }
                                    .into()
                                })
                                .collect_vec(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Dictionary {
                key_type,
                value_type,
            } => {
                if let DBusValue::Vec(values) = arg.dbus_value {
                    MessageItem::Dict(
                        MessageItemDict::new(
                            values
                                .iter()
                                .step_by(2)
                                .zip(values.iter().skip(1).step_by(2))
                                .map(|entry| {
                                    (
                                        DBusArgument {
                                            dbus_type: key_type,
                                            dbus_value: entry.0,
                                        }
                                        .into(),
                                        DBusArgument {
                                            dbus_type: value_type,
                                            dbus_value: entry.1,
                                        }
                                        .into(),
                                    )
                                })
                                .collect_vec(),
                            Into::<String>::into(key_type.as_ref()).into(),
                            Into::<String>::into(value_type.as_ref()).into(),
                        )
                        .unwrap(),
                    )
                } else {
                    panic!()
                }
            }
            DBusType::Variant => todo!(),
        }
    }
}