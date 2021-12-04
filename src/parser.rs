use crate::variant::Variant;

pub trait Parser {
    fn parse(input: &str) -> Result<Variant, ParserError>;
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