use crate::ser::test_stringify;

use serde::Serialize;

#[test]
fn test_unit_like() {
    #[derive(Serialize)]
    enum UnitLike {
        String,
        Int,
        Float,
    }

    test_stringify(UnitLike::String, "¨String¨");
    test_stringify(UnitLike::Int, "¨Int¨");
    test_stringify(UnitLike::Float, "¨Float¨");
}

#[test]
fn test_tuple_like() {
    #[derive(Serialize)]
    enum TupleLike {
        Value(String, i64, f64),
        Unit(()),
        Empty(),
    }

    test_stringify(TupleLike::Unit(()), "{¨Unit¨§}");
    test_stringify(TupleLike::Empty(), "{¨Empty¨|÷}");
    test_stringify(
        TupleLike::Value("string".to_string(), 212301230, 0.312),
        "{¨Value¨|¨string¨¢EMnFO£0.52÷}",
    );
}

#[test]
fn test_struct_like() {
    #[derive(Serialize)]
    enum StructLike {
        Value {
            string: String,
            int: i64,
            float: f64,
        },
        Empty {},
    }
    test_stringify(StructLike::Empty {}, "{¨Empty¨{}}");
    test_stringify(
        StructLike::Value {
            string: "string".to_string(),
            int: 212301230,
            float: 0.312,
        },
        "{¨Value¨{¨string¨ß1¨int¨¢EMnFO¨float¨£0.52}}",
    );
}

#[test]
fn test_nested() {
    #[derive(Serialize)]
    enum StructLike {
        Value {
            string: String,
            int: i64,
            float: f64,
            nested: TupleLike,
        },
        None,
    }
    #[derive(Serialize)]
    enum TupleLike {
        Value(String, i64, f64),
    }

    test_stringify(StructLike::None, "¨None¨");
    test_stringify(
        StructLike::Value {
            string: "string".to_string(),
            int: 212301230,
            float: 0.312,
            nested: TupleLike::Value("string".to_string(), 212301230, 0.312),
        },
        "{¨Value¨{¨string¨ß1¨int¨¢EMnFO¨float¨£0.52¨nested¨{ß0|ß1º0Ý0÷}}}",
    );
}
