use core::slice;
use std::{vec::IntoIter, iter::Zip, marker::PhantomData};

use crate::{dbus_type::DBusType, dbus_value::Value};

pub struct Argument<'a> {
    pub dbus_type: Box<DBusType>, // heap allocated as signatures have unknwon size
    pub dbus_value: Value,
    phantom: PhantomData<&'a DBusType>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DBusError {
    InvalidSignature,
    InvalidValue(String),
}

impl<'a> Argument<'a> {
    pub fn new(dbus_type: DBusType, dbus_value: Value) -> Self {
        Self {
            dbus_type: Box::new(dbus_type),
            dbus_value,
            phantom: PhantomData
        }
    }

    pub fn validate(self) -> Result<Argument<'a>, DBusError> {
        self.dbus_type.is_valid_value(&self.dbus_value).map(|_| self)
    }
}

impl<'a> IntoIterator for Argument<'a> {
    type Item = (&'a DBusType, &'a Value);

    type IntoIter = slice::Iter<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match *self.dbus_type {
            DBusType::Struct(type_vec) => if let Value::Vec(value_vec) = self.dbus_value {
                type_vec.iter().zip(value_vec.iter()).into_iter()
            } else {
                panic!("");
            },
            DBusType::Array { value_type } => todo!(),
            DBusType::Dictionary { key_type, value_type } => todo!(),
            _ => (self.dbus_type, self.dbus_value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{argument::Argument, dbus_value::Value};

    #[test]
    fn test_arguments() {
        let argument = Argument::new(
            "a{si}".into(),
            Value::Vec(vec![Value::String("test".into()), Value::Int32(32)]),
        );

        assert_eq!(argument.validate(), Ok(()));

        assert!(Argument::new("i".into(), Value::Int16(16)).validate().is_err())
    }
}

// impl From<&Variant> for MessageItem {
//     fn from(variant: &Variant) -> Self {
//         match variant {

//         }
//     }
// }
