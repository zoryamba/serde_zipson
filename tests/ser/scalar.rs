use serde_zipson::value::{Value};
use crate::ser::test_stringify;

#[test]
fn test_null() {
    test_stringify(Value::Null, "§");
}
