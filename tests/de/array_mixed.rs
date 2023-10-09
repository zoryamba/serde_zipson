use serde_zipson;
use serde_zipson::value::{Number, Value};
use crate::de::test_parse;

#[test]
fn test_one_of_each() {
    test_parse("|§»«É¤A¢pc6w¢-2AH5Yxa£0.-1n£-0,552345411´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨øSyKTET5±1739m÷", Value::Array(vec![
        Value::Null,
        Value::Bool(true),
        Value::Bool(false),
        Value::Number(Number::Int(0)),
        Value::Number(Number::Int(10)),
        Value::Number(Number::Int(12301230)),
        Value::Number(Number::Int(-123014323230)),
        Value::Number(Number::Float(-0.111)),
        Value::Number(Number::Float(-0.552345411)),
        Value::String("".to_string()),
        Value::String("x".to_string()),
        Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".to_string()),
        Value::String("2022-02-24T04:31:00.123Z".to_string()),
        Value::String("2022-02-24T04:30:00.000Z".to_string()),
    ]));
}