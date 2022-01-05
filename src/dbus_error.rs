
#[derive(Debug, PartialEq, Eq)]
pub enum DBusError {
    InvalidSignature,
    InvalidValue(String),
}