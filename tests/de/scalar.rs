use serde_zipson;
use serde_zipson::value::{Number, Value};
use crate::de::test_parse;

#[test]
fn test_null() {
    test_parse("§", Value::Null);
}

#[test]
fn test_bool() {
    test_parse("»", Value::Bool(true));
    test_parse("«", Value::Bool(false));
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
fn test_big_integer() {
    test_parse("¤A", Value::Number(Number::Int(10)));
    test_parse("¤z", Value::Number(Number::Int(61)));
    test_parse("¢10", Value::Number(Number::Int(62)));
    test_parse("¢pc6w", Value::Number(Number::Int(12301230)));
    test_parse("¢2AGxFdG", Value::Number(Number::Int(123012342310)));

    test_parse("¢-A", Value::Number(Number::Int(-10)));
    test_parse("¢-pc6w", Value::Number(Number::Int(-12301230)));
    test_parse("¢-2AH5Yxa", Value::Number(Number::Int(-123014323230)));
}

#[test]
fn test_float_small() {
    test_parse("£0.1n", Value::Number(Number::Float(0.111)));
    test_parse("£0.-1n", Value::Number(Number::Float(-0.111)));
    test_parse("£5.G7", Value::Number(Number::Float(5.999)));
    test_parse("£-F.-8x", Value::Number(Number::Float(-15.555)));
}

#[test]
fn test_float_full_precision() {
    test_parse("£5,9234827938", Value::Number(Number::Float(5.9234827938)));
    test_parse("£-F,552345411", Value::Number(Number::Float(-15.552345411)));
    test_parse("£0,552345411", Value::Number(Number::Float(0.552345411)));
    test_parse("£-0,552345411", Value::Number(Number::Float(-0.552345411)));
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

#[test]
fn test_string_date() {
    test_parse("øSyKTET5", Value::String("2022-02-24T04:31:00.123Z".into()));
}

#[test]
fn test_string_lp_date() {
    test_parse("±1739m", Value::String("2022-02-24T04:30:00.000Z".into()));
}

#[test]
fn test_ref_integer() {
    test_parse("|¢1n¢3aº0º1º0º1÷", Value::Array(vec![
        Value::Number(Number::Int(111)),
        Value::Number(Number::Int(222)),
        Value::Number(Number::Int(111)),
        Value::Number(Number::Int(222)),
        Value::Number(Number::Int(111)),
        Value::Number(Number::Int(222)),
    ]));
}

#[test]
fn test_ref_float() {
    test_parse("|£1n.1c£3a.3EÝ0Ý1Ý0Ý1÷", Value::Array(vec![
        Value::Number(Number::Float(111.1)),
        Value::Number(Number::Float(222.2)),
        Value::Number(Number::Float(111.1)),
        Value::Number(Number::Float(222.2)),
        Value::Number(Number::Float(111.1)),
        Value::Number(Number::Float(222.2)),
    ]));
}

#[test]
fn test_ref_string() {
    test_parse("|¨aaa¨¨bbb¨ß0ß1ß0ß1÷", Value::Array(vec![
        Value::String("aaa".into()),
        Value::String("bbb".into()),
        Value::String("aaa".into()),
        Value::String("bbb".into()),
        Value::String("aaa".into()),
        Value::String("bbb".into()),
    ]));
}

#[test]
fn test_ref_dates() {
    test_parse("|øSyKTEStøSyKTEUg×0×1×0×1÷", Value::Array(vec![
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
    ]));
}

#[test]
fn test_ref_lp_dates() {
    test_parse("|±1739m±1739sü0ü1ü0ü1÷", Value::Array(vec![
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
    ]));
}
