use serde_zipson::value::{Number, Value};
use crate::ser::{test_stringify, test_stringify_full_precision};

#[test]
fn test_null() {
    test_stringify(Value::Null, "§");
}

#[test]
fn test_bool() {
    test_stringify(Value::Bool(true), "»");
    test_stringify(Value::Bool(false), "«");
}

#[test]
fn test_small_integer() {
    test_stringify(Value::Number(Number::Int(-9)), "À");
    test_stringify(Value::Number(Number::Int(-8)), "Á");
    test_stringify(Value::Number(Number::Int(-7)), "Â");
    test_stringify(Value::Number(Number::Int(-6)), "Ã");
    test_stringify(Value::Number(Number::Int(-5)), "Ä");
    test_stringify(Value::Number(Number::Int(-4)), "Å");
    test_stringify(Value::Number(Number::Int(-3)), "Æ");
    test_stringify(Value::Number(Number::Int(-2)), "Ç");
    test_stringify(Value::Number(Number::Int(-1)), "È");
    test_stringify(Value::Number(Number::Int(-0)), "É");
    test_stringify(Value::Number(Number::Int(0)), "É");
    test_stringify(Value::Number(Number::Int(1)), "Ê");
    test_stringify(Value::Number(Number::Int(2)), "Ë");
    test_stringify(Value::Number(Number::Int(3)), "Ì");
    test_stringify(Value::Number(Number::Int(4)), "Í");
    test_stringify(Value::Number(Number::Int(5)), "Î");
    test_stringify(Value::Number(Number::Int(6)), "Ï");
    test_stringify(Value::Number(Number::Int(7)), "Ð");
    test_stringify(Value::Number(Number::Int(8)), "Ñ");
    test_stringify(Value::Number(Number::Int(9)), "Ò");
}

#[test]
fn test_big_integer() {
    test_stringify(Value::Number(Number::Int(10)), "¤A");
    test_stringify(Value::Number(Number::Int(61)), "¤z");
    test_stringify(Value::Number(Number::Int(62)), "¢10");
    test_stringify(Value::Number(Number::Int(12301230)), "¢pc6w");
    test_stringify(Value::Number(Number::Int(123012342310)), "¢2AGxFdG");

    test_stringify(Value::Number(Number::Int(-10)), "¢-A");
    test_stringify(Value::Number(Number::Int(-12301230)), "¢-pc6w");
    test_stringify(Value::Number(Number::Int(-123014323230)), "¢-2AH5Yxa");
}

#[test]
fn test_float_small() {
    test_stringify(Value::Number(Number::Float(-0.)), "£0.0");
    test_stringify(Value::Number(Number::Float(0.)), "£0.0");
    test_stringify(Value::Number(Number::Float(-0.0001)), "£0.0");
    test_stringify(Value::Number(Number::Float(0.0001)), "£0.0");
    test_stringify(Value::Number(Number::Float(0.001)), "£0.1");
    test_stringify(Value::Number(Number::Float(0.01)), "£0.A");
    test_stringify(Value::Number(Number::Float(0.1)), "£0.1c");
    test_stringify(Value::Number(Number::Float(0.111)), "£0.1n");
    test_stringify(Value::Number(Number::Float(-0.111)), "£0.-1n");
    test_stringify(Value::Number(Number::Float(5.999)), "£5.G7");
    test_stringify(Value::Number(Number::Float(-15.555)), "£-F.-8x");
}

#[test]
fn test_float_full_precision() {
    test_stringify_full_precision(Value::Number(Number::Float(0.)), "£0,0");
    test_stringify_full_precision(Value::Number(Number::Float(-0.)), "£0,0");
    test_stringify_full_precision(Value::Number(Number::Float(5.9234827938)), "£5,9234827938");
    test_stringify_full_precision(Value::Number(Number::Float(-15.552345411)), "£-F,552345411");
    test_stringify_full_precision(Value::Number(Number::Float(0.552345411)), "£0,552345411");
    test_stringify_full_precision(Value::Number(Number::Float(-0.552345411)), "£-0,552345411");
}