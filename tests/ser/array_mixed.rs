use crate::ser::{test_stringify, test_stringify_detect_dates, test_stringify_full_precision};

use serde_zipson::value::{Number, Value};

#[test]
fn test_one_of_each() {
    test_stringify(Value::Array(vec![
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
    ]), "|§»«É¤A¢pc6w¢-2AH5Yxa£0.-1n£0.-8u´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨¨2022-02-24T04:31:00.123Z¨¨2022-02-24T04:30:00.000Z¨÷");
}

#[test]
fn test_one_of_each_full_precision() {
    test_stringify_full_precision(Value::Array(vec![
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
    ]), "|§»«É¤A¢pc6w¢-2AH5Yxa£-0,111£-0,552345411´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨¨2022-02-24T04:31:00.123Z¨¨2022-02-24T04:30:00.000Z¨÷");
}

#[test]
fn test_one_of_each_detect_dates() {
    test_stringify_detect_dates(Value::Array(vec![
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
    ]), "|§»«É¤A¢pc6w¢-2AH5Yxa£0.-1n£0.-8u´´´x´¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨øSyKTET5±1739m÷");
}
