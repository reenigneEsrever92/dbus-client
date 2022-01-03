use core::slice;
use std::{vec::IntoIter, iter::Zip, marker::PhantomData};

use dbus::arg::messageitem::{MessageItem, MessageItemDict};
use itertools::Itertools;

use crate::{dbus_type::DBusType, dbus_value::DBusValue};

pub struct Argument {
    pub dbus_type: Box<DBusType>, // heap allocated as signatures have unknwon size
    pub dbus_value: DBusValue
}

pub enum Arg {

}

#[derive(Debug, PartialEq, Eq)]
pub enum DBusError {
    InvalidSignature,
    InvalidValue(String),
}

impl Argument {
    pub fn new(dbus_type: DBusType, dbus_value: DBusValue) -> Self {
        Self {
            dbus_type: Box::new(dbus_type),
            dbus_value,
        }
    }

    pub fn validate(self) -> Result<Argument, DBusError> {
        self.dbus_type.is_valid_value(&self.dbus_value).map(|_| self)
    }
}

#[cfg(test)]
mod test {
    use crate::{argument::Argument, dbus_value::DBusValue};

    #[test]
    fn test_arguments() {
        let argument = Argument::new(
            "a{si}".into(),
            DBusValue::Vec(vec![DBusValue::String("test".into()), DBusValue::Int32(32)]),
        );

        assert!(Argument::new("i".into(), DBusValue::Int16(16)).validate().is_err())
    }
}

// impl From<&Variant> for MessageItem {
//     fn from(variant: &Variant) -> Self {
//         match variant {

//         }
//     }
// }
