use serde::Serialize;

pub mod array;
pub mod array_mixed;
pub mod enumeration;
pub mod object;
pub mod scalar;
pub mod structure;

fn test_stringify<T: Serialize>(value: T, expected: &str) {
    let res = serde_zipson::ser::to_string::<T>(&value, false, false);
    assert_eq!(res.unwrap(), expected);
}

fn test_stringify_full_precision<T: Serialize>(value: T, expected: &str) {
    let res = serde_zipson::ser::to_string::<T>(&value, true, false);
    assert_eq!(res.unwrap(), expected);
}

fn test_stringify_detect_dates<T: Serialize>(value: T, expected: &str) {
    let res = serde_zipson::ser::to_string::<T>(&value, false, true);
    assert_eq!(res.unwrap(), expected);
}
