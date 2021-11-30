use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while},
    character::complete::{alphanumeric1, char, digit0, one_of},
    combinator::{cut, map, map_res},
    error::{context, ContextError, FromExternalError, ParseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

use crate::{parser::{Parser, ParserError}, variant::{self, Variant}};

struct NomParser;

impl Parser for NomParser {
    fn parse(input: &str) -> Result<Variant, crate::parser::ParserError> {
        Ok(variant(input).map(|result| result.1)?)
    }
}

impl<'l> From<nom::Err<nom::error::Error<&str>>> for ParserError<'l> {
    fn from(result: nom::Err<nom::error::Error<&str>>) -> Self {
        todo!()
    }
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
            alt((
                preceded(
                    char('\"'),
                    cut(terminated(
                        escaped(alphanumeric1, '\\', one_of("\"n\\")),
                        char('\"'),
                    )),
                ),
                escaped(alphanumeric1, '\\', one_of("\"n\\")),
            )),
            |res: &str| Variant::Str(String::from(res)),
        ),
    )(i)
}

fn int16<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "i16",
        map_res(terminated(digit0, tag("i16")), |res: &str| {
            match res.parse::<i16>() {
                Ok(int) => Ok(Variant::Int16(int)),
                Err(e) => Err(e),
            }
        }),
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
        map_res(terminated(digit0, tag("i32")), |res: &str| {
            match res.parse::<i32>() {
                Ok(int) => Ok(Variant::Int32(int)),
                Err(e) => Err(e),
            }
        }),
    )(i)
}

fn int64<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "i64",
        map_res(terminated(digit0, tag("i64")), |res: &str| {
            match res.parse::<i64>() {
                Ok(int) => Ok(Variant::Int64(int)),
                Err(e) => Err(e),
            }
        }),
    )(i)
}

fn word16<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "word16",
        map_res(terminated(digit0, tag("u16")), |res: &str| {
            match res.parse::<u16>() {
                Ok(int) => Ok(Variant::Word16(int)),
                Err(e) => Err(e),
            }
        }),
    )(i)
}

fn word32<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "word32",
        map_res(terminated(digit0, tag("u32")), |res: &str| {
            match res.parse::<u32>() {
                Ok(int) => Ok(Variant::Word32(int)),
                Err(e) => Err(e),
            }
        }),
    )(i)
}

fn word64<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "word64",
        map_res(terminated(digit0, tag("u64")), |res: &str| {
            match res.parse::<u64>() {
                Ok(int) => Ok(Variant::Word64(int)),
                Err(e) => Err(e),
            }
        }),
    )(i)
}

fn f64<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseFloatError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "double",
        map(terminated(double, tag("f64")), |res: f64| {
            Variant::Double(res)
        }),
    )(i)
}

fn array<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>
        + FromExternalError<&'a str, std::num::ParseFloatError>,
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

fn dictionary<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>
        + FromExternalError<&'a str, std::num::ParseFloatError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    context(
        "dictionary",
        map(
            preceded(
                char('{'),
                cut(terminated(
                    separated_list0(preceded(space, char(',')), key_value),
                    preceded(space, char('}')),
                )),
            ),
            |res: Vec<(Variant, Variant)>| Variant::Dictionary(res),
        ),
    )(i)
}

fn key_value<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>
        + FromExternalError<&'a str, std::num::ParseFloatError>,
>(
    i: &'a str,
) -> IResult<&'a str, (Variant, Variant), E> {
    context(
        "key_value",
        separated_pair(
            preceded(space, variant),
            cut(preceded(space, char(':'))),
            variant,
        ),
    )(i)
}

fn variant<
    'a,
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + FromExternalError<&'a str, std::num::ParseIntError>
        + FromExternalError<&'a str, std::num::ParseFloatError>,
>(
    i: &'a str,
) -> IResult<&'a str, Variant, E> {
    preceded(
        space,
        alt((
            array, dictionary, int16, int32, int64, word16, word32, word64, f64, string,
        )),
    )(i)
}

#[cfg(test)]
mod test {
    use std::{any::Any, collections::hash_set::SymmetricDifference, num::ParseIntError};

    use nom::{character::complete::digit1, combinator::map_res};

    use crate::nom_parser::{int32, string, variant, Variant};

    #[test]
    fn test_int32() {
        let val: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = int32("123i32");

        assert_eq!(val, Ok(("", Variant::Int32(123))));
    }

    #[test]
    fn test_string() {
        let val: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = string("\"123\"");

        assert_eq!(val, Ok(("", Variant::Str(String::from("123")))));
    }

    #[test]
    fn test_variant() {
        let str: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("\"123\"");

        let i16: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123i16");
        let i32: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123i32");
        let i64: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123i64");

        let u16: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123u16");
        let u32: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123u32");
        let u64: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123u64");

        let f64: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> = variant("123.12f64");

        let array: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> =
            variant("[12i32, 13i32]");

        let dict: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> =
            variant("{\"first\": 12i32, \"second\": 13i32}");

        let dict2: Result<(&str, Variant), nom::Err<nom::error::Error<&str>>> =
            variant("{ first: 12i32, second: 13i32}");

        assert_eq!(str, Ok(("", Variant::Str(String::from("123")))));

        assert_eq!(i16, Ok(("", Variant::Int16(123))));
        assert_eq!(i32, Ok(("", Variant::Int32(123))));
        assert_eq!(i64, Ok(("", Variant::Int64(123))));

        assert_eq!(u16, Ok(("", Variant::Word16(123))));
        assert_eq!(u32, Ok(("", Variant::Word32(123))));
        assert_eq!(u64, Ok(("", Variant::Word64(123))));

        assert_eq!(f64, Ok(("", Variant::Double(123.12))));

        assert_eq!(
            array,
            Ok((
                "",
                Variant::Array(vec![Variant::Int32(12), Variant::Int32(13i32)])
            ))
        );

        assert_eq!(
            dict,
            Ok((
                "",
                Variant::Dictionary(vec![
                    (Variant::Str("first".into()), Variant::Int32(12)),
                    (Variant::Str("second".into()), Variant::Int32(13))
                ])
            ))
        );

        assert_eq!(
            dict2,
            Ok((
                "",
                Variant::Dictionary(vec![
                    (Variant::Str("first".into()), Variant::Int32(12)),
                    (Variant::Str("second".into()), Variant::Int32(13))
                ])
            ))
        );
    }

    // #[test]
    // fn test_parse() {
    //     let spec = "{i{s(is)}[i]}";
    //     let input = "{ integer: 3, dict: { \"key\": (2, \"test\") }, array: [3:i32, 4:i32] }";
    // }
}
