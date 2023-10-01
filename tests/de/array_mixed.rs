use serde_zipson;
use serde_zipson::value::Value;
use crate::de::test_parse;

#[test]
fn test_one_of_each() {
    test_parse("|´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨÷", Value::Array(vec![
        Value::String("".to_string()),
        Value::String("x".to_string()),
        Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".to_string()),
    ]));
}