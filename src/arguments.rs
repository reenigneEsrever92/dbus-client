use std::{collections::HashMap, iter::Map};

use itertools::PadUsing;
use regex::Regex;

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
    Integer32(Value<String>),
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


struct Dictionary {
    val: HashMap<Variant, Variant>,
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


#[cfg(test)]
mod test {
    use std::any::Any;

    use super::{Dictionary, ParsingError, Val, Value};


    #[test]
    fn test_simple() {
        let dict: Dictionary = Dictionary::try_from("{si}").unwrap();
        dict.takes("[{ \"key\": 4 }]");
    }

    #[test]
    fn test_parse() {
        let spec = "{i{s(is)},[i]}";
        let input = "{ integer: 3, dict: { \"key\": (2, \"test\") }, array: [3:i32, 4:i32] }";
    }

}