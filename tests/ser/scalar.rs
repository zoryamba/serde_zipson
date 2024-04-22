use serde_zipson::value::{Number::{Float, Int}, Value};
use serde_zipson::ser::to_string;
use crate::ser::{test_stringify, test_stringify_detect_dates, test_stringify_full_precision};

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
    test_stringify(Value::Number(Int(-9)), "À");
    test_stringify(Value::Number(Int(-8)), "Á");
    test_stringify(Value::Number(Int(-7)), "Â");
    test_stringify(Value::Number(Int(-6)), "Ã");
    test_stringify(Value::Number(Int(-5)), "Ä");
    test_stringify(Value::Number(Int(-4)), "Å");
    test_stringify(Value::Number(Int(-3)), "Æ");
    test_stringify(Value::Number(Int(-2)), "Ç");
    test_stringify(Value::Number(Int(-1)), "È");
    test_stringify(Value::Number(Int(-0)), "É");
    test_stringify(Value::Number(Int(0)), "É");
    test_stringify(Value::Number(Int(1)), "Ê");
    test_stringify(Value::Number(Int(2)), "Ë");
    test_stringify(Value::Number(Int(3)), "Ì");
    test_stringify(Value::Number(Int(4)), "Í");
    test_stringify(Value::Number(Int(5)), "Î");
    test_stringify(Value::Number(Int(6)), "Ï");
    test_stringify(Value::Number(Int(7)), "Ð");
    test_stringify(Value::Number(Int(8)), "Ñ");
    test_stringify(Value::Number(Int(9)), "Ò");
}

#[test]
fn test_big_integer() {
    test_stringify(Value::Number(Int(10)), "¤A");
    test_stringify(Value::Number(Int(61)), "¤z");
    test_stringify(Value::Number(Int(62)), "¢10");
    test_stringify(Value::Number(Int(12301230)), "¢pc6w");
    test_stringify(Value::Number(Int(123012342310)), "¢2AGxFdG");

    test_stringify(Value::Number(Int(-10)), "¢-A");
    test_stringify(Value::Number(Int(-12301230)), "¢-pc6w");
    test_stringify(Value::Number(Int(-123014323230)), "¢-2AH5Yxa");
}

#[test]
fn test_unreferenced_integer() {
    test_stringify(Value::Array(vec![
        Value::Number(Int(61)),
        Value::Number(Int(62)),
        Value::Number(Int(61)),
    ]), "|¤z¢10¤z÷");
}

#[test]
fn test_ref_integer() {
    test_stringify(Value::Array(vec![
        Value::Number(Int(111)),
        Value::Number(Int(222)),
        Value::Number(Int(111)),
        Value::Number(Int(222)),
        Value::Number(Int(111)),
        Value::Number(Int(222)),
    ]), "|¢1n¢3aº0º1º0º1÷");
}

#[test]
fn test_float_small() {
    test_stringify(Value::Number(Float(-0.)), "£0.0");
    test_stringify(Value::Number(Float(0.)), "£0.0");
    test_stringify(Value::Number(Float(-0.0001)), "£0.0");
    test_stringify(Value::Number(Float(0.0001)), "£0.0");
    test_stringify(Value::Number(Float(0.001)), "£0.1");
    test_stringify(Value::Number(Float(0.01)), "£0.A");
    test_stringify(Value::Number(Float(0.1)), "£0.1c");
    test_stringify(Value::Number(Float(0.111)), "£0.1n");
    test_stringify(Value::Number(Float(-0.111)), "£0.-1n");
    test_stringify(Value::Number(Float(5.999)), "£5.G7");
    test_stringify(Value::Number(Float(-15.555)), "£-F.-8x");
}

#[test]
fn test_float_full_precision() {
    test_stringify_full_precision(Value::Number(Float(0.)), "£0,0");
    test_stringify_full_precision(Value::Number(Float(-0.)), "£0,0");
    test_stringify_full_precision(Value::Number(Float(5.9234827938)), "£5,9234827938");
    test_stringify_full_precision(Value::Number(Float(-15.552345411)), "£-F,552345411");
    test_stringify_full_precision(Value::Number(Float(0.552345411)), "£0,552345411");
    test_stringify_full_precision(Value::Number(Float(-0.552345411)), "£-0,552345411");
}

#[test]
fn test_unreferenced_float() {
    let mut values = vec![];
    for i in 0..62 * 62 {
        values.push(Value::Number(Float(0.111 + 0.01 * i as f64)));
    }

    values.push(Value::Number(Float(0.001)));
    values.push(Value::Number(Float(111.111)));
    values.push(Value::Number(Float(0.001)));

    let res = to_string::<Value>(&Value::Array(values), false, false);

    let re = regex::Regex::new(r"¥0\.1£1n\.1n¥0\.1÷$").unwrap();
    assert!(re.is_match(&res.unwrap()));
}

#[test]
fn test_ref_float() {
    test_stringify(Value::Array(vec![
        Value::Number(Float(111.1)),
        Value::Number(Float(222.2)),
        Value::Number(Float(111.1)),
        Value::Number(Float(222.2)),
        Value::Number(Float(111.1)),
        Value::Number(Float(222.2)),
    ]), "|£1n.1c£3a.3EÝ0Ý1Ý0Ý1÷");
}

#[test]
fn test_empty_string() {
    test_stringify(Value::String("".into()), "´´");
}


#[test]
fn test_short_string() {
    test_stringify(Value::String("x".into()), "´x´");
}

#[test]
fn test_short_string_single_quote() {
    test_stringify(Value::String("'".into()), "´'´");
}

#[test]
fn test_short_string_double_quote() {
    test_stringify(Value::String("\"".into()), "´\"´");
}

#[test]
fn test_short_string_string_token() {
    test_stringify(Value::String("¨".into()), "¨\\¨¨");
}

#[test]
fn test_short_string_unreferenced_string_token() {
    test_stringify(Value::String("´".into()), "´\\´´");
}

#[test]
fn test_short_string_escape_token() {
    test_stringify(Value::String("\\".into()), "¨\\\\¨");
}

#[test]
fn test_long_string() {
    test_stringify(Value::String("aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".into()), "¨aoasdfjalisruhgalsiuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨");
}

#[test]
fn test_long_string_single_quote() {
    test_stringify(Value::String("'aoasdfjalisruhgals'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc'".into()), "¨'aoasdfjalisruhgals'iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc'¨");
}

#[test]
fn test_long_string_double_quote() {
    test_stringify(Value::String("\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"".into()), "¨\"aoasdfjalisruhgals\"iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\"¨");
}

#[test]
fn test_long_string_string_token() {
    test_stringify(Value::String("¨aoasdfjalisruhgals¨iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨".into()), "¨\\¨aoasdfjalisruhgals\\¨iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc\\¨¨");
}

#[test]
fn test_long_string_unreferenced_string_token() {
    test_stringify(Value::String("´aoasdfjalisruhgals´iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc´".into()), "¨´aoasdfjalisruhgals´iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc´¨");
}

#[test]
fn test_long_string_escape_token() {
    test_stringify(Value::String("\\aoasdfjalisruhgals\\\\iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc".into()), "¨\\\\aoasdfjalisruhgals\\\\\\\\iuhfdlsajdlifuashrlifuhsaildjfsalkhglasurflasjdfklsandfasurliausnlc¨");
}

#[test]
fn test_string_date() {
    test_stringify_detect_dates(Value::String("2022-02-24T04:31:00.123Z".into()), "øSyKTET5");
}

#[test]
fn test_unreferenced_string() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("x".into()),
        Value::String("aaa".into()),
        Value::String("x".into()),
    ]), "|´x´¨aaa¨´x´÷");
}

#[test]
fn test_ref_string() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("aaa".into()),
        Value::String("bbb".into()),
        Value::String("aaa".into()),
        Value::String("bbb".into()),
        Value::String("aaa".into()),
        Value::String("bbb".into()),
    ]), "|¨aaa¨¨bbb¨ß0ß1ß0ß1÷");
}

#[test]
fn test_unreferenced_date() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("1970-01-01T00:00:00.001Z".into()),
    ]), "|¿1÷");
}

#[test]
fn test_ref_date() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
        Value::String("2022-02-24T04:31:00.111Z".into()),
        Value::String("2022-02-24T04:31:00.222Z".into()),
    ]), "|øSyKTEStøSyKTEUg×0×1×0×1÷");
}

#[test]
fn test_string_lp_date() {
    test_stringify_detect_dates(Value::String("2022-02-24T04:30:00.000Z".into()), "±1739m");
}

#[test]
fn test_unreferenced_lp_date() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("1970-01-01T00:10:00.000Z".into()),
    ]), "|ÿ6÷");
}

#[test]
fn test_ref_lp_date() {
    test_stringify_detect_dates(Value::Array(vec![
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
        Value::String("2022-02-24T04:30:00.000Z".into()),
        Value::String("2022-02-24T04:40:00.000Z".into()),
    ]), "|±1739m±1739sü0ü1ü0ü1÷");
}
