use serde_zipson::value::Value;

pub mod array;
pub mod array_mixed;
pub mod object;
pub mod scalar;

fn test_parse(str: &str, expected: Value) {
    let res = serde_zipson::de::from_str::<Value>(str);
    assert_eq!(res.unwrap(), expected);
}
