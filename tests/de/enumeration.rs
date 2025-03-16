use crate::de::test_parse;

use serde::{Deserialize, Serialize};

#[test]
fn test_unit_like() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum UnitLike {
        String,
        Int,
        Float,
    }

    test_parse("¨String¨", UnitLike::String);
    test_parse("¨Int¨", UnitLike::Int);
    test_parse("¨Float¨", UnitLike::Float);
}

#[test]
fn test_tuple_like() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum TupleLike {
        Value(String, i64, f64),
        Unit(()),
        Empty(),
    }

    test_parse("{¨Unit¨§}", TupleLike::Unit(()));
    test_parse("{¨Empty¨|÷}", TupleLike::Empty());
    test_parse(
        "{¨Value¨|¨string¨¢EMnFO£0.52÷}",
        TupleLike::Value("string".to_string(), 212301230, 0.312),
    );
}

#[test]
fn test_struct_like() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum StructLike {
        Value {
            string: String,
            int: i64,
            float: f64,
        },
        Empty {},
    }
    test_parse("{¨Empty¨{}}", StructLike::Empty {});
    test_parse(
        "{¨Value¨{¨string¨ß1¨int¨¢EMnFO¨float¨£0.52}}",
        StructLike::Value {
            string: "string".to_string(),
            int: 212301230,
            float: 0.312,
        },
    );
}

#[test]
fn test_nested() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum StructLike {
        Value {
            string: String,
            int: i64,
            float: f64,
            nested: TupleLike,
        },
        None,
    }
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum TupleLike {
        Value(String, i64, f64),
        None,
    }

    test_parse("¨None¨", StructLike::None);
    test_parse(
        "{¨Value¨{¨string¨ß1¨int¨¢EMnFO¨float¨£0.52¨nested¨{ß0|ß1º0Ý0÷}}}",
        StructLike::Value {
            string: "string".to_string(),
            int: 212301230,
            float: 0.312,
            nested: TupleLike::Value("string".to_string(), 212301230, 0.312),
        },
    );
}
