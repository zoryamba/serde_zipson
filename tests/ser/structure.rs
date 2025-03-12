use crate::ser::test_stringify;

use indexmap::{indexmap, IndexMap};
use serde::Serialize;
use serde_zipson::value::{Number, Value};

#[test]
fn test_empty() {
    #[derive(Serialize)]
    struct Object {}

    test_stringify(Object {}, "{}");
}

#[test]
fn test_homogenous() {
    #[derive(Serialize)]
    struct Object {
        x: i64,
        y: i64,
        z: i64,
    }

    test_stringify(Object { x: 1, y: 2, z: 3 }, "{´x´Ê´y´Ë´z´Ì}");
}

#[test]
fn test_mixed() {
    #[derive(Serialize)]
    struct Object {
        x: i64,
        y: i64,
        z: String,
        i: String,
        longkey: bool,
        nope: (),
    }

    test_stringify(
        Object {
            x: 1,
            y: 212301230,
            z: "asdfioj{{}}".into(),
            i: "".into(),
            longkey: true,
            nope: (),
        },
        "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§}",
    );
}

#[test]
fn test_nested() {
    #[derive(Serialize)]
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

    #[derive(Serialize)]
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

    test_stringify(Object {
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
            yep: Value::Object(indexmap! {
                "5".into() => Value::Array(vec![Value::Null]),
                "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
            }),
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
    }, "{´x´Ê´y´¢EMnFO´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨float¨£TQZ.6y¨nest¨{´x´Ê´y´º0ß3£0.52´z´ß0´i´´´ß1»ß2§¨yep¨{´5´|§÷¨string¨¨\"\"asoidj{}sidofj¨}}¨array_nest¨|{´x´Ê´y´º0´z´ß0´i´´´ß1»ß2§}÷}");
}

#[test]
fn nest_newtype() {
    #[derive(Serialize)]
    struct StringNewType(String);

    test_stringify(StringNewType("string".into()), "¨string¨");

    #[derive(Serialize)]
    struct IntNewType(i64);

    test_stringify(IntNewType(123), "¢1z");
}

#[test]
fn nest_newtype_nested() {
    #[derive(Serialize)]
    struct ObjectNewType(NestedObject);

    #[derive(Serialize)]
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

    test_stringify(
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
        "{´x´Ê´y´¢EMnFO¨float¨£0.52´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨yep¨{´5´|§÷¨string¨¨\"\"asoidj{}sidofj¨}}",
    );
}

#[test]
fn test_unit() {
    #[derive(Serialize)]
    struct UnitStruct(());

    test_stringify(UnitStruct(()), "§");
}

#[test]
fn test_tuple() {
    #[derive(Serialize)]
    struct TupleStruct(String, i64, UnitStruct, NestedObject);

    #[derive(Serialize)]
    struct UnitStruct(());

    #[derive(Serialize)]
    struct NestedObject {
        x: i64,
        y: i64,
        float: f64,
        z: String,
        i: String,
        longkey: bool,
        nope: (),
        yep: Value,
    }

    test_stringify(
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
                nope: (),
                yep: Value::Object(indexmap! {
                    "5".into() => Value::Array(vec![Value::Null]),
                    "string".into() => Value::String("\"\"asoidj{}sidofj".into()),
                }),
            }
        ),
        "|¨string¨¢EMnFO§{´x´Ê´y´º0¨float¨£0.52´z´¨asdfioj{{}}¨´i´´´¨longkey¨»¨nope¨§¨yep¨{´5´|§÷ß0¨\"\"asoidj{}sidofj¨}}÷",
    );
}
