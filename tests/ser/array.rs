use serde_zipson::value::{Value};
use crate::ser::test_stringify;

#[test]
fn test_empty_array() {
    test_stringify(Value::Array(vec![]), "|÷");
}
