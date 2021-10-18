use std::{convert::{TryFrom, TryInto}, fmt::Display, str::FromStr};

use serde::Serialize;
use serde_json::Number;
use serde_with::de::DeserializeAsWrap;

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
// #[serde(into = "T")]
// struct PossibleString<T>{
//     value: T
// }

// impl <U: PossibleString<T>, T> From<PossibleString<T>> for T {

// }

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum StringOr<T>{
    String(String),
    Value(T),
}


impl<T> From<T> for StringOr<T> {
    /// Copies `val` into a new `Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// let o: Option<u8> = Option::from(67);
    ///
    /// assert_eq!(Some(67), o);
    /// ```
    fn from(val: T) -> StringOr<T> {
        Self::Value(val)
    }
}

impl <T> Display for StringOr<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringOr::String(x) => x.fmt(f),
            StringOr::Value(x) => x.fmt(f),
        }
    }
}

impl<T> FromStr for StringOr<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::String(s.to_string()))
    }
}


// #[serde_as]
// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
// #[serde(transparent)]
// pub struct ValueOrDisplayFromStr<T> where T:Serialize + serde::Deserialize {
//     #[serde_as(as = "serde_with::PickFirst<(_, serde_with::DisplayFromStr)>")]
//     value: T,
// }

// impl std::ops::Deref for ValueOrDisplayFromStr {
//     type Target = Number;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//         serde_with::DisplayFromStr
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
// #[serde(untagged)]
// enum NumberOrString {
//     Number(Number),
//     String(NumberAsString),
// }

macro_rules! make_as_string {
    ($v:vis $i:ident for $t:ty) => {
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
        #[serde(try_from = "String", into = "String")]
        $v struct $i {
            value: $t,
        }

        impl From<$i> for String {
            fn from(val: $i) -> Self {
                val.to_string()
            }
        }

        impl From<$i> for $t {
            fn from(val: $i) -> Self {
                val.value
            }
        }

        impl From<$t> for $i {
            fn from(val: $t) -> Self {
                Self { value: val }
            }
        }

        impl TryFrom<String> for $i {
            type Error = serde_json::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: serde_json::from_str(&value)?,
                })
            }
        }

        impl std::ops::Deref for $i {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        impl FromStr for $i {
            type Err = serde_json::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self {
                    value: serde_json::from_str(s)?,
                })
            }
        }
    };
}

make_as_string! {pub NumberAsString for Number}
make_as_string! {pub FloatAsString for f64}

#[cfg(test)]
mod test {
    use serde_json::Number;

    use super::NumberAsString;

    #[test]
    fn parse() {
        let value = Number::from_f64(32.2).unwrap();
        let str = value.to_string();
        let nas: NumberAsString = str.parse().unwrap();
        assert_eq!(*nas, value);
    }

    #[test]
    fn deserlize() {
        let value = Number::from_f64(32.2).unwrap();
        let str = value.to_string();

        let json = serde_json::to_string_pretty(&str).unwrap();
        assert_eq!(json, "\"32.2\"");
        let nas: NumberAsString = serde_json::from_str(&json).expect("deserlize failed");
        assert_eq!(*nas, value);
    }

    #[test]
    fn serlize() {
        let value: Number = 20.into();
        let nas: NumberAsString = value.into();
        let json = serde_json::to_string_pretty(&nas).unwrap();
        assert_eq!("\"20\"", json)
    }
}
