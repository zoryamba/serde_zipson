use serde_zipson::value::{Value};
use crate::ser::test_stringify;

#[test]
fn test_null() {
    test_stringify(Value::Null, "§");
}

#[test]
fn test_bool() {
    test_stringify(Value::Bool(true), "»");
    test_stringify(Value::Bool(false), "«");
}
