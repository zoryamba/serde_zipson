use crate::ser::{test_stringify, test_stringify_detect_dates, test_stringify_full_precision};

use indexmap::IndexMap;
use serde_zipson::value::{Number, Value};

#[test]
fn test_empty() {
    test_stringify(Value::Array(vec![]), "|÷");
}

#[test]
fn test_one_null() {
    test_stringify(Value::Array(vec![Value::Null]), "|§÷");
}

#[test]
fn test_one_bool() {
    test_stringify(Value::Array(vec![Value::Bool(true)]), "|»÷");
    test_stringify(Value::Array(vec![Value::Bool(false)]), "|«÷");
}

#[test]
fn test_one_small_integer() {
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-9))]), "|À÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-8))]), "|Á÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-7))]), "|Â÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-6))]), "|Ã÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-5))]), "|Ä÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-4))]), "|Å÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-3))]), "|Æ÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-2))]), "|Ç÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-1))]), "|È÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-0))]), "|É÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(1))]), "|Ê÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(2))]), "|Ë÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(3))]), "|Ì÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(4))]), "|Í÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(5))]), "|Î÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(6))]), "|Ï÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(7))]), "|Ð÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(8))]), "|Ñ÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(9))]), "|Ò÷");
}

#[test]
fn test_one_big_integer() {
    test_stringify(Value::Array(vec![Value::Number(Number::Int(10))]), "|¤A÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(61))]), "|¤z÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(62))]), "|¢10÷");
    test_stringify(
        Value::Array(vec![Value::Number(Number::Int(12301230))]),
        "|¢pc6w÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Int(123012342310))]),
        "|¢2AGxFdG÷",
    );

    test_stringify(Value::Array(vec![Value::Number(Number::Int(-10))]), "|¢-A÷");
    test_stringify(
        Value::Array(vec![Value::Number(Number::Int(-12301230))]),
        "|¢-pc6w÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Int(-123014323230))]),
        "|¢-2AH5Yxa÷",
    );
}

#[test]
fn test_one_float_small() {
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(-0.))]),
        "|£0.0÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.))]),
        "|£0.0÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(-0.0001))]),
        "|£0.0÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.0001))]),
        "|£0.0÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.001))]),
        "|£0.1÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.01))]),
        "|£0.A÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.1))]),
        "|£0.1c÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(0.111))]),
        "|£0.1n÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(-0.111))]),
        "|£0.-1n÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(5.999))]),
        "|£5.G7÷",
    );
    test_stringify(
        Value::Array(vec![Value::Number(Number::Float(-15.555))]),
        "|£-F.-8x÷",
    );
}

#[test]
fn test_one_float_full_precision() {
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(0.))]),
        "|£0,0÷",
    );
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(-0.))]),
        "|£0,0÷",
    );
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(5.9234827938))]),
        "|£5,9234827938÷",
    );
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(-15.552345411))]),
        "|£-F,552345411÷",
    );
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(0.552345411))]),
        "|£0,552345411÷",
    );
    test_stringify_full_precision(
        Value::Array(vec![Value::Number(Number::Float(-0.552345411))]),
        "|£-0,552345411÷",
    );
}

#[test]
fn test_one_empty_string() {
    test_stringify(Value::Array(vec![Value::String("".into())]), "|´´÷");
}

#[test]
fn test_one_short_string() {
    test_stringify(Value::Array(vec![Value::String("x".into())]), "|´x´÷");
}

#[test]
fn test_one_long_string() {
    test_stringify(
        Value::Array(vec![Value::String(
            "aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc"
                .into(),
        )]),
        "|¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨÷",
    );
}

#[test]
fn test_one_date() {
    test_stringify_detect_dates(
        Value::Array(vec![Value::String("2022-02-24T04:31:00.123Z".into())]),
        "|øSyKTET5÷",
    );
}

#[test]
fn test_one_lp_date() {
    test_stringify_detect_dates(
        Value::Array(vec![Value::String("2022-02-24T04:30:00.000Z".into())]),
        "|±1739m÷",
    );
}

#[test]
fn test_repeat_scalar() {
    test_stringify(
        Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
        "|´x´þ÷",
    );
}

#[test]
fn test_repeat_array() {
    test_stringify(
        Value::Array(vec![
            Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
            Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
        ]),
        "||´x´þ÷þ÷",
    );
}

#[test]
fn test_repeat_object() {
    test_stringify(
        Value::Array(vec![
            Value::Object(IndexMap::from([("x".into(), Value::String("x".into()))])),
            Value::Object(IndexMap::from([("x".into(), Value::String("x".into()))])),
        ]),
        "|{´x´´x´}þ÷",
    );
}

#[test]
fn test_repeat_scalar_multiple() {
    test_stringify(
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("y".into()),
            Value::String("z".into()),
            Value::String("z".into()),
        ]),
        "|´x´þþ´y´´z´þ÷",
    );
}

#[test]
fn test_repeat_many() {
    test_stringify(
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
        ]),
        "|´x´þþþ^2÷",
    );
}

#[test]
fn test_repeat_many_with_trailing() {
    test_stringify(
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("y".into()),
        ]),
        "|´x´þþþ^2´y´÷",
    );
}

#[test]
fn test_repeat_indexed() {
    test_stringify(
        Value::Array(vec![
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
        ]),
        "|¨xyz¨ß0þþþ^1÷",
    );
}
