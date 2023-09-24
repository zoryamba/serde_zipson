use serde_zipson;
use serde_zipson::value::Value;

#[test]
fn test_string() {
    let res = serde_zipson::de::from_str::<Value>("¨string¨");
    assert_eq!(res.unwrap(), Value::String("string".to_string()));
}

#[test]
fn test_empty_string() {
    let res = serde_zipson::de::from_str::<Value>("¨¨");
    assert_eq!(res.unwrap(), Value::String("".to_string()));
}