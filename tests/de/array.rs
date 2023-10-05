use serde_zipson;
use serde_zipson::value::{Number, Value};
use crate::de::test_parse;

#[test]
fn test_empty_array() {
    test_parse("|÷", Value::Array(vec![]));
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

#[test]
fn test_one_null() {
    test_parse("|§÷", Value::Array(vec![Value::Null]));
}
