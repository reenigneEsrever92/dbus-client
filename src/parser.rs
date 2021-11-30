use crate::variant::Variant;

pub trait Parser {
    fn parse(input: &str) -> Result<Variant, ParserError>;
}

pub struct ParserError<'l> {
    errors: Vec<Error<'l>>
}

pub struct Error<'l> {
    line: i32,
    column: i32,
    error: &'l str
}