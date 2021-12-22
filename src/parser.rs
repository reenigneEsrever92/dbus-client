use crate::dbus_type::DBusType;

pub trait Parser {
    fn parse(input: &str) -> Result<DBusType, ParserError>;
}

#[derive(Debug)]
pub struct ParserError<'l> {
    errors: Vec<Error<'l>>
}

#[derive(Debug)]
pub struct Error<'l> {
    line: i32,
    column: i32,
    error: &'l str
}