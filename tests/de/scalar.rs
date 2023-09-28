use serde_zipson;
use serde_zipson::constants::STRING_TOKEN;
use serde_zipson::value::Value;

#[test]
fn test_empty_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("".into()));
}

#[test]
fn test_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), "string".to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("string".into()));
}