use serde_zipson;
use serde_zipson::constants::{ARRAY_END_TOKEN, ARRAY_START_TOKEN, STRING_TOKEN};
use serde_zipson::value::Value;

#[test]
fn test_empty_array() {
    let res = serde_zipson::de::from_str::<Value>(vec![ARRAY_START_TOKEN.to_string(), ARRAY_END_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::Array(vec![]));
}

#[test]
fn test_one_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![ARRAY_START_TOKEN.to_string(), STRING_TOKEN.to_string(), "string".to_string(), STRING_TOKEN.to_string(), ARRAY_END_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::Array(vec![Value::String("string".into())]));
}

#[test]
fn test_two_strings() {
    let res = serde_zipson::de::from_str::<Value>(vec![ARRAY_START_TOKEN.to_string(), STRING_TOKEN.to_string(), "string1".to_string(), STRING_TOKEN.to_string(), STRING_TOKEN.to_string(), "string2".to_string(), STRING_TOKEN.to_string(), ARRAY_END_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::Array(vec![Value::String("string1".into()), Value::String("string2".into())]));
}
