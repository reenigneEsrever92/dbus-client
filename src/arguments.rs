use nom::{
    branch::alt,
    bytes::complete::{escaped, take_while},
    character::complete::{alphanumeric1, char, digit1, one_of},
    combinator::{cut, map, map_res},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};


#[derive(Debug, PartialEq)]
pub enum Variant {
    Str(String),
    I32(i32),
    Array(Vec<Variant>),
}

fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn string<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "string",
        map(
            preceded(
                char('\"'),
                cut(terminated(
                    escaped(alphanumeric1, '\\', one_of("\"n\\")),
                    char('\"'),
                )),
            ),
            |res: &str| Variant::Str(String::from(res)),
        ),
    )(i)
}

fn int32<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "i32",
        map_res(digit1, |res: &str| match res.parse::<i32>() {
            Ok(int) => Ok(Variant::I32(int)),
            Err(e) => Err(e),
        }),
    )(i)
}

fn array<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "array",
        map(
            preceded(
                char('['),
                cut(terminated(
                    separated_list0(preceded(space, char(',')), variant),
                    preceded(space, char(']')),
                )),
            ),
            |res| Variant::Array(Vec::from(res)),
        ),
    )(i)
}

fn variant<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    preceded(space, alt((array, string)))(i)
}

#[cfg(test)]
mod test {
    use std::{any::Any, collections::hash_set::SymmetricDifference, num::ParseIntError};

    use nom::{character::complete::digit1, combinator::map_res};

    use crate::arguments::{int32, string, Variant};

    #[test]
    fn test_i32() {
        let val: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = int32("123");

        assert_eq!(val, Ok(("", Variant::I32(123))));
    }

    #[test]
    fn test_string() {
        let val: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = string("\"123\"");

        assert_eq!(val, Ok(("", Variant::Str(String::from("123")))));
    }

    // #[test]
    // fn test_parse() {
    //     let spec = "{i{s(is)}[i]}";
    //     let input = "{ integer: 3, dict: { \"key\": (2, \"test\") }, array: [3:i32, 4:i32] }";
    // }
}
