

use std::{collections::HashMap, iter::Map, process::Output, str::FromStr};

use itertools::PadUsing;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DICTIONARY_RE: Regex = Regex::new(r"{(\w+: *.+)(, *\w+: *.+)*}").unwrap();
}

struct Spec {
    value: String
}

// struct Instance<T: Value<T>> {
//     value: Value<Map<String, T>>
// }

// struct Dictionary<K, T> {

// }

// enum Value {
//     Dictionary(HashMap<Value, Value>),
//     Array(Vec<Value>),
//     Tuple(Vec<Value>)
// }

enum Variant {
    Dictionary(HashMap<Variant, Variant>),
    Array(Vec<Variant>),
    Tuple(Vec<Variant>),
    String(Value<String>),
    Integer32(Value<i32>),
    Integer16(Value<String>),
    Integer8(Value<String>),
    Float64(Value<String>),
}

struct Value<T: Val<T>> {
    value: T
}

trait Val<T> {
    fn takes(&self, value: &str) -> bool;
    fn take(&self, value: &str) -> Result<(), ParsingError>;
    fn get_val(&self) -> T;
    fn get_mnemonic() -> &'static str;
}

impl Val<i32> for Variant {
    fn takes(&self, value: &str) -> bool {
        if let Self::Integer32(value) = self {
            if true {// TODO check parsable
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn take(&self, value: &str)-> Result<(), ParsingError> {
        todo!()
    }

    fn get_val(&self) -> i32 {
        todo!()
    }

    fn get_mnemonic() -> &'static str {
        todo!()
    }
}

impl<T> Val<Value<T>> for Value<T> where T: Val<T> {
    fn takes(&self, value: &str) -> bool {
        todo!()
    }

    fn take(&self, value: &str)-> Result<(), ParsingError> {
        todo!()
    }

    fn get_val(&self) -> Value<T> {
        todo!()
    }

    fn get_mnemonic() -> &'static str {
        todo!()
    }
}

impl Val<String> for String {
    fn takes(&self, value: &str) -> bool {
        todo!()
    }

    fn take(&self, value: &str)-> Result<(), ParsingError> {
        todo!()
    }

    fn get_val(&self) -> String {
        todo!()
    }

    fn get_mnemonic() -> &'static str {
        todo!()
    }
}

impl Val<i32> for i32 {
    fn takes(&self, value: &str) -> bool {
        todo!()
    }

    fn take(&self, value: &str)-> Result<(), ParsingError> {
        todo!()
    }

    fn get_val(&self) -> i32 {
        todo!()
    }

    fn get_mnemonic() -> &'static str {
        todo!()
    }
}

trait Parsable {
    fn parsable(arg: &str) -> bool;
    fn try_parse(arg: &str) -> Result<Box<Self>, ParsingError>;
}

#[derive(Default)]
struct Dictionary {
    val: HashMap<Variant, Variant>,
}

struct Array {
    val: Vec<Variant>
}

impl Parsable for Dictionary {

    fn parsable(arg: &str) -> bool {
        DICTIONARY_RE.is_match(arg)
    }

    fn try_parse(arg: &str) -> Result<Box<Self>, ParsingError> {
        Ok(Box::new(Dictionary::default()))
    }
}

impl TryFrom<&str> for Dictionary {
    type Error = ParsingError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let reg = Regex::new(r"\{[si][si]\}").unwrap();
        
        if reg.is_match(value) {
            Ok(Dictionary { val: HashMap::new() })
        } else {
            Err(ParsingError {})
        }
    }
}


#[derive(Debug)]
struct ParsingError {

}

impl Val<HashMap<Variant, Variant>> for Dictionary {
    fn takes(&self, value: &str) -> bool {
        todo!()
    }

    fn take(&self, value: &str) -> Result<(), ParsingError> {
        todo!()
    }

    fn get_val(&self) -> HashMap<Variant, Variant> {
        todo!()
    }

    fn get_mnemonic() -> &'static str {
        todo!()
    }
}

enum Arguments {
    Dictionary
    Array
}


#[cfg(test)]
mod test {
    use std::{any::Any, collections::hash_set::SymmetricDifference};

    use super::{Dictionary, Parsable, ParsingError, Val, Value, Variant};


    #[test]
    fn test_simple() {
        let dict = Dictionary::try_parse("{ \"key\": 4 }");

        assert_eq!(dict.is_ok(), true);
    }

    #[test]
    fn test_simple_positional() {
        let dict: Dictionary = Dictionary::try_from("{si}").unwrap();
        dict.takes("{ \"key\": 4 }");
    }

    #[test]
    fn test_parse() {
        let spec = "{i{s(is)}[i]}";
        let input = "{ integer: 3, dict: { \"key\": (2, \"test\") }, array: [3:i32, 4:i32] }";
    }

}