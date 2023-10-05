use serde_zipson;
use serde_zipson::value::{Number, Value};
use crate::de::test_parse;

#[test]
fn test_null() {
    test_parse("§", Value::Null);
}

#[test]
fn test_small_integer() {
    test_parse("À", Value::Number(Number::Int(-9)));
    test_parse("Á", Value::Number(Number::Int(-8)));
    test_parse("Â", Value::Number(Number::Int(-7)));
    test_parse("Ã", Value::Number(Number::Int(-6)));
    test_parse("Ä", Value::Number(Number::Int(-5)));
    test_parse("Å", Value::Number(Number::Int(-4)));
    test_parse("Æ", Value::Number(Number::Int(-3)));
    test_parse("Ç", Value::Number(Number::Int(-2)));
    test_parse("È", Value::Number(Number::Int(-1)));
    test_parse("É", Value::Number(Number::Int(-0)));
    test_parse("Ê", Value::Number(Number::Int(1)));
    test_parse("Ë", Value::Number(Number::Int(2)));
    test_parse("Ì", Value::Number(Number::Int(3)));
    test_parse("Í", Value::Number(Number::Int(4)));
    test_parse("Î", Value::Number(Number::Int(5)));
    test_parse("Ï", Value::Number(Number::Int(6)));
    test_parse("Ð", Value::Number(Number::Int(7)));
    test_parse("Ñ", Value::Number(Number::Int(8)));
    test_parse("Ò", Value::Number(Number::Int(9)));
}

#[test]
fn test_empty_string() {
    test_parse("¨¨", Value::String("".into()));
}

#[test]
fn test_short_string() {
    test_parse("´x´", Value::String("x".into()));
}

#[test]
fn test_short_string_single_quote() {
    test_parse("´'´", Value::String("'".into()));
}

#[test]
fn test_short_string_double_quote() {
    test_parse("´\"´", Value::String("\"".into()));
}

#[test]
fn test_short_string_string_token() {
    test_parse("¨\\¨¨", Value::String("¨".into()));
}

#[test]
fn test_short_string_unreferenced_string_token() {
    test_parse("´\\´´", Value::String("´".into()));
}

#[test]
fn test_long_string() {
    test_parse("¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨", Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".into()));
}

#[test]
fn test_long_string_single_quote() {
    test_parse("¨'aoasdfjalisruhgals'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc'¨", Value::String("'aoasdfjalisruhgals'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc'".into()));
}

#[test]
fn test_long_string_double_quote() {
    test_parse("¨\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"¨", Value::String("\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"".into()));
}

#[test]
fn test_long_string_string_token() {
    test_parse("¨\\¨aoasdfjalisruhgals\\¨iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\\¨¨", Value::String("¨aoasdfjalisruhgals¨iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨".into()));
}

#[test]
fn test_long_string_unreferenced_string_token() {
    test_parse("¨´aoasdfjalisruhgals´iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc´¨", Value::String("´aoasdfjalisruhgals´iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc´".into()));
}