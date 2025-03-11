use crate::de::test_parse;

use indexmap::IndexMap;
use serde_zipson::value::{Number, Value};

#[test]
fn test_empty_array() {
    test_parse("|÷", Value::Array(vec![]));
}

#[test]
fn test_one_null() {
    test_parse("|§÷", Value::Array(vec![Value::Null]));
}

#[test]
fn test_one_bool() {
    test_parse("|»÷", Value::Array(vec![Value::Bool(true)]));
    test_parse("|«÷", Value::Array(vec![Value::Bool(false)]));
}

#[test]
fn test_one_small_integer() {
    test_parse("|À÷", Value::Array(vec![Value::Number(Number::Int(-9))]));
    test_parse("|Á÷", Value::Array(vec![Value::Number(Number::Int(-8))]));
    test_parse("|Â÷", Value::Array(vec![Value::Number(Number::Int(-7))]));
    test_parse("|Ã÷", Value::Array(vec![Value::Number(Number::Int(-6))]));
    test_parse("|Ä÷", Value::Array(vec![Value::Number(Number::Int(-5))]));
    test_parse("|Å÷", Value::Array(vec![Value::Number(Number::Int(-4))]));
    test_parse("|Æ÷", Value::Array(vec![Value::Number(Number::Int(-3))]));
    test_parse("|Ç÷", Value::Array(vec![Value::Number(Number::Int(-2))]));
    test_parse("|È÷", Value::Array(vec![Value::Number(Number::Int(-1))]));
    test_parse("|É÷", Value::Array(vec![Value::Number(Number::Int(-0))]));
    test_parse("|Ê÷", Value::Array(vec![Value::Number(Number::Int(1))]));
    test_parse("|Ë÷", Value::Array(vec![Value::Number(Number::Int(2))]));
    test_parse("|Ì÷", Value::Array(vec![Value::Number(Number::Int(3))]));
    test_parse("|Í÷", Value::Array(vec![Value::Number(Number::Int(4))]));
    test_parse("|Î÷", Value::Array(vec![Value::Number(Number::Int(5))]));
    test_parse("|Ï÷", Value::Array(vec![Value::Number(Number::Int(6))]));
    test_parse("|Ð÷", Value::Array(vec![Value::Number(Number::Int(7))]));
    test_parse("|Ñ÷", Value::Array(vec![Value::Number(Number::Int(8))]));
    test_parse("|Ò÷", Value::Array(vec![Value::Number(Number::Int(9))]));
}

#[test]
fn test_one_big_integer() {
    test_parse("|¤A÷", Value::Array(vec![Value::Number(Number::Int(10))]));
    test_parse("|¤z÷", Value::Array(vec![Value::Number(Number::Int(61))]));
    test_parse("|¢10÷", Value::Array(vec![Value::Number(Number::Int(62))]));
    test_parse(
        "|¢pc6w÷",
        Value::Array(vec![Value::Number(Number::Int(12301230))]),
    );
    test_parse(
        "|¢2AGxFdG÷",
        Value::Array(vec![Value::Number(Number::Int(123012342310))]),
    );

    test_parse("|¢-A÷", Value::Array(vec![Value::Number(Number::Int(-10))]));
    test_parse(
        "|¢-pc6w÷",
        Value::Array(vec![Value::Number(Number::Int(-12301230))]),
    );
    test_parse(
        "|¢-2AH5Yxa÷",
        Value::Array(vec![Value::Number(Number::Int(-123014323230))]),
    );
}

#[test]
fn test_one_float_small() {
    test_parse(
        "|£0.0÷",
        Value::Array(vec![Value::Number(Number::Float(0.))]),
    );
    test_parse(
        "|£0.1÷",
        Value::Array(vec![Value::Number(Number::Float(0.001))]),
    );
    test_parse(
        "|£0.A÷",
        Value::Array(vec![Value::Number(Number::Float(0.01))]),
    );
    test_parse(
        "|£0.1c÷",
        Value::Array(vec![Value::Number(Number::Float(0.1))]),
    );
    test_parse(
        "|£0.1n÷",
        Value::Array(vec![Value::Number(Number::Float(0.111))]),
    );
    test_parse(
        "|£0.-1n÷",
        Value::Array(vec![Value::Number(Number::Float(-0.111))]),
    );
    test_parse(
        "|£5.G7÷",
        Value::Array(vec![Value::Number(Number::Float(5.999))]),
    );
    test_parse(
        "|£-F.-8x÷",
        Value::Array(vec![Value::Number(Number::Float(-15.555))]),
    );
}

#[test]
fn test_one_float_full_precision() {
    test_parse(
        "|£0,0÷",
        Value::Array(vec![Value::Number(Number::Float(0.))]),
    );
    test_parse(
        "|£5,9234827938÷",
        Value::Array(vec![Value::Number(Number::Float(5.9234827938))]),
    );
    test_parse(
        "|£-F,552345411÷",
        Value::Array(vec![Value::Number(Number::Float(-15.552345411))]),
    );
    test_parse(
        "|£0,552345411÷",
        Value::Array(vec![Value::Number(Number::Float(0.552345411))]),
    );
    test_parse(
        "|£-0,552345411÷",
        Value::Array(vec![Value::Number(Number::Float(-0.552345411))]),
    );
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
    test_parse(
        "|¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨÷",
        Value::Array(vec![Value::String(
            "aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc"
                .into(),
        )]),
    );
}

#[test]
fn test_one_date() {
    test_parse(
        "|øSyKTET5÷",
        Value::Array(vec![Value::String("2022-02-24T04:31:00.123Z".into())]),
    );
}

#[test]
fn test_one_lp_date() {
    test_parse(
        "|±1739m÷",
        Value::Array(vec![Value::String("2022-02-24T04:30:00.000Z".into())]),
    );
}

#[test]
fn test_repeat_scalar() {
    test_parse(
        "|´x´þ÷",
        Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
    );
}

#[test]
fn test_repeat_array() {
    test_parse(
        "||´x´þ÷þ÷",
        Value::Array(vec![
            Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
            Value::Array(vec![Value::String("x".into()), Value::String("x".into())]),
        ]),
    );
}

#[test]
fn test_repeat_object() {
    test_parse(
        "|{´x´´x´}þ÷",
        Value::Array(vec![
            Value::Object(IndexMap::from([("x".into(), Value::String("x".into()))])),
            Value::Object(IndexMap::from([("x".into(), Value::String("x".into()))])),
        ]),
    );
}

#[test]
fn test_repeat_scalar_multiple() {
    test_parse(
        "|´x´þþ´y´´z´þ÷",
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("y".into()),
            Value::String("z".into()),
            Value::String("z".into()),
        ]),
    );
}

#[test]
fn test_repeat_many() {
    test_parse(
        "|´x´þ^2þþ÷",
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
        ]),
    );
}

#[test]
fn test_repeat_many_with_trailing() {
    test_parse(
        "|´x´þ^2þþ´y´÷",
        Value::Array(vec![
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("x".into()),
            Value::String("y".into()),
        ]),
    );
}

#[test]
fn test_repeat_indexed() {
    test_parse(
        "|¨xyz¨ß0þþþ^1÷",
        Value::Array(vec![
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
            Value::String("xyz".into()),
        ]),
    );
}
