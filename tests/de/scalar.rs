use serde_zipson;
use serde_zipson::value::Value;
use crate::de::test_parse;

#[test]
fn test_null() {
    test_parse("§", Value::Null);
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