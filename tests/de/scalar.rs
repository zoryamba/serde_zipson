use serde_zipson;
use serde_zipson::constants::{STRING_TOKEN, UNREFERENCED_STRING_TOKEN};
use serde_zipson::value::Value;

#[test]
fn test_empty_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("".into()));
}

#[test]
fn test_short_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![UNREFERENCED_STRING_TOKEN.to_string(), 'a'.to_string(), UNREFERENCED_STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("a".into()));
}

#[test]
fn test_short_string_single_quote() {
    let res = serde_zipson::de::from_str::<Value>(vec![UNREFERENCED_STRING_TOKEN.to_string(), "'".to_string(), UNREFERENCED_STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("'".into()));
}

#[test]
fn test_short_string_double_quote() {
    let res = serde_zipson::de::from_str::<Value>(vec![UNREFERENCED_STRING_TOKEN.to_string(), '"'.to_string(), UNREFERENCED_STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String('"'.into()));
}

#[test]
fn test_short_string_string_token() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), '\\'.to_string(), STRING_TOKEN.to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String(STRING_TOKEN.to_string()));
}

#[test]
fn test_short_string_unreferenced_string_token() {
    let res = serde_zipson::de::from_str::<Value>(vec![UNREFERENCED_STRING_TOKEN.to_string(), '\\'.to_string(), UNREFERENCED_STRING_TOKEN.to_string(), UNREFERENCED_STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String(UNREFERENCED_STRING_TOKEN.to_string()));
}

#[test]
fn test_long_string() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), "aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".into()));
}

#[test]
fn test_long_string_single_quote() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), "\'aoasdfjalisruhgals\'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\'".to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("\'aoasdfjalisruhgals\'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\'".into()));
}

#[test]
fn test_long_string_double_quote() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), "\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"".to_string(), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String("\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"".into()));
}

#[test]
fn test_long_string_string_token() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), format!("\\{}aoasdfjalisruhgals\\{}iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\\{}", STRING_TOKEN, STRING_TOKEN, STRING_TOKEN), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String(format!("{}aoasdfjalisruhgals{}iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc{}", STRING_TOKEN, STRING_TOKEN, STRING_TOKEN)));
}

#[test]
fn test_long_string_unreferenced_string_token() {
    let res = serde_zipson::de::from_str::<Value>(vec![STRING_TOKEN.to_string(), format!("{}aoasdfjalisruhgals{}iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc{}", UNREFERENCED_STRING_TOKEN, UNREFERENCED_STRING_TOKEN, UNREFERENCED_STRING_TOKEN), STRING_TOKEN.to_string()].join("").as_str());
    assert_eq!(res.unwrap(), Value::String(format!("{}aoasdfjalisruhgals{}iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc{}", UNREFERENCED_STRING_TOKEN, UNREFERENCED_STRING_TOKEN, UNREFERENCED_STRING_TOKEN)));
}