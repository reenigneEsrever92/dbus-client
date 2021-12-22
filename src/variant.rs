use dbus::arg::messageitem::MessageItem;

use crate::{dbus_type::DBusType, value::Value};

struct Variant{
    signature: Box<DBusType>, // heap allocated as signatures have unknwon size
    value: Value
}

// impl From<&Variant> for MessageItem {
//     fn from(variant: &Variant) -> Self {
//         match variant {

//         }
//     }
// }