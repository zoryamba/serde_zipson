use serde_zipson::value::Value;

pub mod scalar;
pub mod array;
pub mod array_mixed;

fn test_stringify(value: Value, expected: &str) {
    let res = serde_zipson::ser::to_string::<Value>(&value, false, false);
    assert_eq!(res.unwrap(), expected);
}

fn test_stringify_full_precision(value: Value, expected: &str) {
    let res = serde_zipson::ser::to_string::<Value>(&value, true, false);
    assert_eq!(res.unwrap(), expected);
}

fn test_stringify_detect_dates(value: Value, expected: &str) {
    let res = serde_zipson::ser::to_string::<Value>(&value, false, true);
    assert_eq!(res.unwrap(), expected);
}