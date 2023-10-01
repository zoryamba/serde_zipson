use serde_zipson;
use serde_zipson::value::Value;
use crate::de::test_parse;

#[test]
fn test_empty_array() {
    test_parse("|÷", Value::Array(vec![]));
}

#[test]
fn test_one_empty_string() {
    test_parse("|¨¨÷", Value::Array(vec![Value::String("".into())]));
}

#[test]
fn test_one_short_string() {
    test_parse("|´x´÷", Value::Array(vec![Value::String("x".into())]));
}

#[test]
fn test_one_long_string() {
    test_parse("|¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨÷", Value::Array(vec![Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".into())]));
}
