// use std::ops::{Range, RangeFrom, RangeInclusive};

// use nom::{
//     branch::{alt, permutation},
//     bytes::complete::{escaped, tag, take_while},
//     character::complete::{alpha1, alphanumeric1, char, digit0, digit1, one_of},
//     combinator::{cut, map, map_res},
//     error::{context, ContextError, ErrorKind, FromExternalError, ParseError},
//     multi::{fold_many0, separated_list0},
//     number::complete::double,
//     sequence::{self, preceded, separated_pair, terminated},
//     AsChar, IResult, InputLength, InputTakeAtPosition, Slice,
// };
// use regex::internal::Char;

// use crate::{
//     parser::{Parser, ParserError},
//     signature::{self, DBusType},
// };

// pub struct NomParser;

// impl Parser for NomParser {
//     fn parse(input: &str) -> Result<DBusType, ParserError> {
//         Ok(variant(input).map(|result| result.1)?)
//     }
// }

// impl<'l> From<nom::Err<nom::error::Error<&str>>> for ParserError<'l> {
//     fn from(error: nom::Err<nom::error::Error<&str>>) -> Self {
//         panic!("{}", error);
//     }
// }

// fn space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
//     let chars = " \t\r\n";

//     // nom combinators like `take_while` return a function. That function is the
//     // parser,to which we can pass the input
//     take_while(move |c| chars.contains(c))(i)
// }

// fn string<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "string",
//         map(
//             alt((preceded(char('\"'), cut(terminated(str, char('\"')))), str)),
//             |res: &str| DBusType::Str(String::from(res)),
//         ),
//     )(i)
// }

// fn str<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, &str, E> {
//     context(
//         "str",
//         escaped(
//             concat_orderless(
//                 alphanumeric1,
//                 concat_orderless(tag("-"), concat_orderless(tag("."), tag("@"))),
//             ),
//             '\\',
//             one_of("\"n\\"),
//         ),
//     )(i)
// }

// pub fn concat_orderless<T, E: ParseError<T>, T1, T2>(
//     mut left: T1,
//     mut right: T2,
// ) -> impl FnMut(T) -> IResult<T, T, E>
// where
//     T: Copy + Clone + Slice<Range<usize>> + InputLength,
//     T1: FnMut(T) -> IResult<T, T, E>,
//     T2: FnMut(T) -> IResult<T, T, E>,
// {
//     move |input: T| {
//         match left(input.clone()) {
//             Ok(v1) => match right(v1.0) {
//                 Ok(v2) => Ok((v2.0, input.slice(0..v1.1.input_len() + v2.1.input_len()))),
//                 Err(_) => Ok((v1.0, v1.1)),
//             },
//             Err(_) => {
//                 match right(input.clone()) {
//                     Ok(v2) => Ok((v2.0, v2.1)),
//                     Err(e) => Err(e), // TODO better errors
//                 }
//             }
//         }
//     }
// }

// fn int16<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "i16",
//         map_res(terminated(digit0, tag("i16")), |res: &str| {
//             match res.parse::<i16>() {
//                 Ok(int) => Ok(DBusType::Int16(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn int32<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "i32",
//         map_res(terminated(digit0, tag("i32")), |res: &str| {
//             match res.parse::<i32>() {
//                 Ok(int) => Ok(DBusType::Int32(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn int64<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "i64",
//         map_res(terminated(digit0, tag("i64")), |res: &str| {
//             match res.parse::<i64>() {
//                 Ok(int) => Ok(DBusType::Int64(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn word16<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "word16",
//         map_res(terminated(digit0, tag("u16")), |res: &str| {
//             match res.parse::<u16>() {
//                 Ok(int) => Ok(DBusType::UInt16(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn word32<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "word32",
//         map_res(terminated(digit0, tag("u32")), |res: &str| {
//             match res.parse::<u32>() {
//                 Ok(int) => Ok(DBusType::UInt32(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn word64<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "word64",
//         map_res(terminated(digit0, tag("u64")), |res: &str| {
//             match res.parse::<u64>() {
//                 Ok(int) => Ok(DBusType::UInt64(int)),
//                 Err(e) => Err(e),
//             }
//         }),
//     )(i)
// }

// fn f64<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "double",
//         map(terminated(double, tag("f64")), |res: f64| {
//             DBusType::Double(res)
//         }),
//     )(i)
// }

// fn array<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "array",
//         map(
//             preceded(
//                 char('['),
//                 cut(terminated(
//                     separated_list0(preceded(space, char(',')), variant),
//                     preceded(space, char(']')),
//                 )),
//             ),
//             |res| DBusType::Array(Vec::from(res)),
//         ),
//     )(i)
// }

// fn tuple<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "tuple",
//         map(
//             preceded(
//                 char('('),
//                 cut(terminated(
//                     separated_list0(preceded(space, char(',')), variant),
//                     preceded(space, char(')')),
//                 )),
//             ),
//             |res| DBusType::Struct(Vec::from(res)),
//         ),
//     )(i)
// }

// fn dictionary<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     context(
//         "dictionary",
//         map(
//             preceded(
//                 char('{'),
//                 cut(terminated(
//                     separated_list0(preceded(space, char(',')), key_value),
//                     preceded(space, char('}')),
//                 )),
//             ),
//             |res: Vec<(DBusType, DBusType)>| DBusType::Dictionary(res),
//         ),
//     )(i)
// }

// fn key_value<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, (DBusType, DBusType), E> {
//     context(
//         "key_value",
//         separated_pair(
//             preceded(space, variant),
//             cut(preceded(space, char(':'))),
//             variant,
//         ),
//     )(i)
// }

// fn variant<
//     'a,
//     E: ParseError<&'a str>
//         + ContextError<&'a str>
//         + FromExternalError<&'a str, std::num::ParseIntError>
//         + FromExternalError<&'a str, std::num::ParseFloatError>,
// >(
//     i: &'a str,
// ) -> IResult<&'a str, DBusType, E> {
//     preceded(
//         space,
//         alt((
//             tuple, array, dictionary, int16, int32, int64, word16, word32, word64, f64, string,
//         )),
//     )(i)
// }

// #[cfg(test)]
// mod test {
//     use std::{any::Any, collections::hash_set::SymmetricDifference, num::ParseIntError};

//     use nom::{character::complete::digit1, combinator::map_res};

//     use crate::nom_parser::{int32, string, variant, DBusType};

//     #[test]
//     fn test_int32() {
//         let val: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = int32("123i32");

//         assert_eq!(val, Ok(("", DBusType::Int32(123))));
//     }

//     #[test]
//     fn test_string() {
//         let val: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = string("\"123\"");

//         assert_eq!(val, Ok(("", DBusType::Str(String::from("123")))));
//     }

//     #[test]
//     fn test_variant() {
//         let str: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("\"123\"");

//         let i16: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123i16");
//         let i32: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123i32");
//         let i64: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123i64");

//         let u16: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123u16");
//         let u32: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123u32");
//         let u64: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123u64");

//         let f64: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123.12f64");
//         let variant: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> = variant("123.12v");

//         let array: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> =
//             variant("[12i32, 13i32]");

//         let array2: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> =
//             variant("[abc, xyz]");

//         let dict: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> =
//             variant("{\"first\": 12i32, \"second\": 13i32}");

//         let dict2: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> =
//             variant("{ first: 12i32, second: apps-menu@gnome-shell-extensions.gcampax.github.com}");

//         let tuple: Result<(&str, DBusType), nom::Err<nom::error::Error<&str>>> =
//             variant("(string, 12i32, {second: apps-menu@gnome-shell-extensions.gcampax.github.com})");

//         assert_eq!(str, Ok(("", DBusType::Str(String::from("123")))));

//         assert_eq!(i16, Ok(("", DBusType::Int16(123))));
//         assert_eq!(i32, Ok(("", DBusType::Int32(123))));
//         assert_eq!(i64, Ok(("", DBusType::Int64(123))));

//         assert_eq!(u16, Ok(("", DBusType::UInt16(123))));
//         assert_eq!(u32, Ok(("", DBusType::UInt32(123))));
//         assert_eq!(u64, Ok(("", DBusType::UInt64(123))));

//         assert_eq!(f64, Ok(("", DBusType::Double(123.12))));

//         assert_eq!(
//             array,
//             Ok((
//                 "",
//                 DBusType::Array(vec![DBusType::Int32(12), DBusType::Int32(13i32)])
//             ))
//         );

//         assert_eq!(
//             array2,
//             Ok((
//                 "",
//                 DBusType::Array(vec![
//                     DBusType::Str("abc".to_string()),
//                     DBusType::Str("xyz".to_string())
//                 ])
//             ))
//         );

//         assert_eq!(
//             dict,
//             Ok((
//                 "",
//                 DBusType::Dictionary(vec![
//                     (DBusType::Str("first".into()), DBusType::Int32(12)),
//                     (DBusType::Str("second".into()), DBusType::Int32(13))
//                 ])
//             ))
//         );

//         assert_eq!(
//             dict2,
//             Ok((
//                 "",
//                 DBusType::Dictionary(vec![
//                     (DBusType::Str("first".into()), DBusType::Int32(12)),
//                     (
//                         DBusType::Str("second".into()),
//                         DBusType::Str(String::from(
//                             "apps-menu@gnome-shell-extensions.gcampax.github.com"
//                         ))
//                     )
//                 ])
//             ))
//         );

//         assert_eq!(
//             tuple,
//             Ok((
//                 "",
//                 DBusType::Struct(vec![
//                     DBusType::Str("string".into()), 
//                     DBusType::Int32(12),
//                     DBusType::Dictionary(
//                         vec![(
//                             DBusType::Str("second".to_string()),
//                             DBusType::Str("apps-menu@gnome-shell-extensions.gcampax.github.com".to_string())
//                         )]
//                     )
//                 ])
//             ))
//         );
//     }

//     // #[test]
//     // fn test_parse() {
//     //     let spec = "{i{s(is)}[i]}";
//     //     let input = "{ integer: 3, dict: { \"key\": (2, \"test\") }, array: [3:i32, 4:i32] }";
//     // }
// }
