use dbus::arg::messageitem::MessageItem;

use crate::{dbus_type::DBusType, value::Value};

pub struct Argument {
    signature: Box<DBusType>, // heap allocated as signatures have unknwon size
    value: Value,
}

pub enum ArgumentError {
    InvalidSignature,
}

impl Argument {
    pub fn new(signature: DBusType, value: Value) -> Self {
        Self { signature: Box::new(signature), value }
    }

    pub fn validate(&self) -> Result<(), ArgumentError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{argument::Argument, value::Value};

    // #[test]
    // fn test_arguments() {
    //     Argument::new("a{si}".to_string().into(), Value::Vec(vec![Value::Str("test".into()), Value::Int32(32)]));
    // }
}

// impl From<&Variant> for MessageItem {
//     fn from(variant: &Variant) -> Self {
//         match variant {

//         }
//     }
// }
