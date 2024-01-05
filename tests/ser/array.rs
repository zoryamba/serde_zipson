use serde_zipson::value::{Number, Value};
use crate::ser::test_stringify;

#[test]
fn test_empty_array() {
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
    test_stringify(Value::Array(vec![Value::Number(Number::Int(12301230))]), "|¢pc6w÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(123012342310))]), "|¢2AGxFdG÷");

    test_stringify(Value::Array(vec![Value::Number(Number::Int(-10))]), "|¢-A÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-12301230))]), "|¢-pc6w÷");
    test_stringify(Value::Array(vec![Value::Number(Number::Int(-123014323230))]), "|¢-2AH5Yxa÷");
}
