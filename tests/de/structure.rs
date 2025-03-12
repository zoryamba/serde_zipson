use crate::de::test_parse;
use indexmap::{indexmap, IndexMap};

use serde::Deserialize;
use serde_zipson::value::{Number, Value};

#[test]
fn test_empty() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Object {}
    test_parse("{}", Object {});
}

#[test]
fn test_homogenous() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Object {
        x: i64,
        y: i64,
        z: i64,
    }

    test_parse("{´x´Ê´y´Ë´z´Ì}", Object { x: 1, y: 2, z: 3 });
}

#[test]
fn test_mixed() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Object {
        x: i64,
        y: i64,
        z: String,
        i: String,
        longkey: bool,
        nope: (),
    }

    test_parse(
        "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§}",
        Object {
            x: 1,
            y: 212301230,
            z: "asdfioj{{}}".into(),
            i: "".into(),
            longkey: true,
            nope: (),
        },
    );
}

#[test]
fn test_nested() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Object {
        x: i64,
        y: i64,
        z: String,
        i: String,
        longkey: bool,
        nope: (),
        float: f64,
        nest: NestedObject,
        array_nest: Vec<IndexMap<String, Value>>,
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct NestedObject {
        x: i64,
        y: i64,
        float: f64,
        z: String,
        i: String,
        longkey: bool,
        nope: Option<()>,
        yep: Option<Value>,
    }
    test_parse(
        "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨float¨£TQZ.6y¨nest¨{´x´Ê´y´º0ß3£0.52´z´ß0´i´´´ß1»ß2§¨yep¨{´5´|§÷¨string¨¨\"\"asoidj{}sidofj¨}}¨array_nest¨|{´x´Ê´y´º0´z´ß0´i´´´ß1»ß2§}÷}",
        Object {
            x: 1,
            y: 212301230,
            z: "asdfioj{{}}".into(),
            i: "".into(),
            longkey: true,
            nope: (),
            float: 113123.432,
            nest: NestedObject {
                x: 1,
                y: 212301230,
                float: 0.312,
                z: "asdfioj{{}}".into(),
                i: "".into(),
                longkey: true,
                nope: None,
                yep: Some(Value::Object(indexmap! {
                    "5".into() => Value::Array(vec![Value::Null]),
                    "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
                })),
            },
            array_nest: vec![
                indexmap! {
                    "x".into() => Value::Number(Number::Int(1)),
                    "y".into() => Value::Number(Number::Int(212301230)),
                    "z".into() => Value::String("asdfioj{{}}".into()),
                    "i".into() => Value::String("".into()),
                    "longkey".into() => Value::Bool(true),
                    "nope".into() => Value::Null,
                },
            ],
        }
    );
}

#[test]
fn nest_newtype() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct StringNewType(String);

    test_parse("¨string¨", StringNewType("string".into()));

    #[derive(Deserialize, PartialEq, Debug)]
    struct IntNewType(i64);

    test_parse("¢1z", IntNewType(123));
}

#[test]
fn nest_newtype_nested() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct ObjectNewType(NestedObject);

    #[derive(Deserialize, PartialEq, Debug)]
    struct NestedObject {
        x: i64,
        y: i64,
        float: f64,
        z: String,
        i: String,
        longkey: bool,
        nope: Option<()>,
        yep: Option<Value>,
    }

    test_parse(
        "{´x´Ê´y´¢EMnFO¨float¨£0.52´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨yep¨{´5´|§÷¨string¨¨\"\"asoidj{}sidofj¨}}",
        ObjectNewType(NestedObject {
            x: 1,
            y: 212301230,
            float: 0.312,
            z: "asdfioj{{}}".into(),
            i: "".into(),
            longkey: true,
            nope: None,
            yep: Some(Value::Object(indexmap! {
                "5".into() => Value::Array(vec![Value::Null]),
                "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
            })),
        }),
    );
}

#[test]
fn test_unit() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct UnitStruct(());

    test_parse("§", UnitStruct(()));
}

#[test]
fn test_tuple() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct TupleStruct(String, i64, UnitStruct, NestedObject);

    #[derive(Deserialize, PartialEq, Debug)]
    struct UnitStruct(());

    #[derive(Deserialize, PartialEq, Debug)]
    struct NestedObject {
        x: i64,
        y: i64,
        float: f64,
        z: String,
        i: String,
        longkey: bool,
        nope: Option<()>,
        yep: Value,
    }

    test_parse(
        "|¨string¨¢EMnFO§{´x´Ê´y´º0¨float¨£0.52´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨yep¨{´5´|§÷ß0¨\"\"asoidj{}sidofj¨}}÷",
        TupleStruct(
            "string".into(),
            212301230,
            UnitStruct(()),
            NestedObject {
                x: 1,
                y: 212301230,
                float: 0.312,
                z: "asdfioj{{}}".into(),
                i: "".into(),
                longkey: true,
                nope: None,
                yep: Value::Object(indexmap! {
                    "5".into() => Value::Array(vec![Value::Null]),
                    "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
                }),
            }
        ),
    );
}
