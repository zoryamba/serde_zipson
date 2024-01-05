use serde_zipson::value::Value;

pub mod scalar;
pub mod array;

fn test_stringify(value: Value, expected: &str) {
    let res = serde_zipson::ser::to_string::<Value>(&value);
    assert_eq!(res.unwrap(), expected);
}