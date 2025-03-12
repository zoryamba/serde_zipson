use serde::Deserialize;
use std::fmt::Debug;

pub mod array;
pub mod array_mixed;
pub mod object;
pub mod scalar;
pub mod structure;

fn test_parse<'de, T: Deserialize<'de> + PartialEq + Debug>(str: &'de str, expected: T) {
    let res = serde_zipson::de::from_str::<T>(str);
    assert_eq!(res.unwrap(), expected);
}
