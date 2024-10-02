use indexmap::{indexmap, IndexMap};
use serde_zipson::value::{Number, Value};
use crate::ser::{test_stringify};

#[test]
fn test_empty_object() {
    test_stringify(Value::Object(IndexMap::new()), "{}");
}

#[test]
fn test_homogenous() {
    test_stringify(Value::Object(indexmap! {
        "x".into() => Value::Number(Number::Int(1)),
        "y".into() => Value::Number(Number::Int(2)),
        "z".into() => Value::Number(Number::Int(3)),
    }), "{´x´Ê´y´Ë´z´Ì}", );
}

#[test]
fn test_mixed() {
    test_stringify(Value::Object(indexmap! {
        "x".into() => Value::Number(Number::Int(1)),
        "y".into() => Value::Number(Number::Int(212301230)),
        "z".into() => Value::String("asdfioj{{}}".into()),
        "i".into() => Value::String("".into()),
        "longkey".into() => Value::Bool(true),
        "nope".into() => Value::Null,
    }), "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§}");
}

#[test]
fn test_nested() {
    test_stringify(Value::Object(indexmap! {
        "x".into() => Value::Number(Number::Int(1)),
        "y".into() => Value::Number(Number::Int(212301230)),
        "z".into() => Value::String("asdfioj{{}}".into()),
        "i".into() => Value::String("".into()),
        "longkey".into() => Value::Bool(true),
        "nope".into() => Value::Null,
        "float".into() => Value::Number(Number::Float(113123.432)),
        "nest".into() => Value::Object(indexmap!{
            "x".into() => Value::Number(Number::Int(1)),
            "y".into() => Value::Number(Number::Int(212301230)),
            "float".into() => Value::Number(Number::Float(0.312)),
            "z".into() => Value::String("asdfioj{{}}".into()),
            "i".into() => Value::String("".into()),
            "longkey".into() => Value::Bool(true),
            "nope".into() => Value::Null,
            "yep".into() => Value::Object(indexmap!{
                "5".into() => Value::Array(vec![Value::Null]),
                "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
            })
        }),
        "array_nest".into() => Value::Array(vec![
            Value::Object(indexmap!{
                "x".into() => Value::Number(Number::Int(1)),
                "y".into() => Value::Number(Number::Int(212301230)),
                "z".into() => Value::String("asdfioj{{}}".into()),
                "i".into() => Value::String("".into()),
                "longkey".into() => Value::Bool(true),
                "nope".into() => Value::Null,
            }),
        ])
    }), "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨float¨£TQZ.6y¨nest¨{´x´Ê´y´º0ß3£0.52´z´ß0´i´´´ß1»ß2§¨yep¨{´5´|§÷¨string¨¨\"\"asoidj{}sidofj¨}}¨array_nest¨|{´x´Ê´y´º0´z´ß0´i´´´ß1»ß2§}÷}");
}
