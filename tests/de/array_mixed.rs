use serde_zipson;
use serde_zipson::value::{Number, Value};
use crate::de::test_parse;

#[test]
fn test_one_of_each() {
    test_parse("|§É¤A¢pc6w¢-2AH5Yxa´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨÷", Value::Array(vec![
        Value::Null,
        Value::Number(Number::Int(0)),
        Value::Number(Number::Int(10)),
        Value::Number(Number::Int(12301230)),
        Value::Number(Number::Int(-123014323230)),
        Value::String("".to_string()),
        Value::String("x".to_string()),
        Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".to_string()),
    ]));
}