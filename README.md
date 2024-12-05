# serde_zipson

serde_zipson is [serde](https://serde.rs/)-compatible Rust implementation of [zipson](https://www.npmjs.com/package/zipson) compression format.

serde_zipson playground is available on [Github Pages](https://zoryamba.github.io/zipson-playground/)

## Usage


### Serialize


```rust
use indexmap::IndexMap;
use serde_zipson::ser::to_string;
use serde_zipson::value::{Number, Value};

fn main() {
    let string = to_string(
        &Value::Object(
            IndexMap::from([
                ("x".to_string(), Value::Number(Number::Int(1))),
                ("y".to_string(), Value::Number(Number::Int(2)))
            ])
        ),
        true, // full_precision_floats
        true, // detect_utc_timestamps
    ).unwrap();

    assert_eq!(string, "{´x´Ê´y´Ë}");
}
```


### Deserialize
```rust
use indexmap::IndexMap;
use serde_zipson::de::from_str;
use serde_zipson::value::{Number, Value};

fn main() {
    let value = from_str::<Value>("{´x´Ê´y´Ë}").unwrap();

    assert_eq!(
        value,
        Value::Object(
            IndexMap::from([
                ("x".to_string(), Value::Number(Number::Int(1))),
                ("y".to_string(), Value::Number(Number::Int(2)))
            ])
        )
    );
}
```


### Convert to JSON


```rust
use serde_zipson::de::from_str;
use serde_zipson::value::Value;

fn main() {
    let zipson_value = from_str::<Value>("{´x´Ê´y´Ë}").unwrap();

    let json_value = serde_json::to_value(zipson_value).unwrap();

    let json_string = json_value.to_string();

    assert_eq!(json_string, "{\"x\":1,\"y\":2}");
}
```


### Convert from JSON


```rust
use serde_json::from_str;
use serde_zipson::value::Value;
use serde_zipson::ser::to_string;

fn main() {
    let json_value = from_str::<serde_json::Value>("{\"x\":1,\"y\":2}").unwrap();

    let zipson_value = serde_json::from_value::<Value>(json_value).unwrap();

    let zipson_string = to_string(&zipson_value, true, true).unwrap();

    assert_eq!(zipson_string, "{´x´Ê´y´Ë}");
}
```


### Known issues

- `serialize_struct`/`deserialize_struct` are not implemented yet, so serde `derive` doesn't work for structs
- `serialize_enum`/`deserialize_enum` are not implemented yet, so serde `derive` doesn't work for enums
- `serde_zipson` panics on integer overflow
- `serde_zipson` repeat feature not working yet, so `[1,1,1,1,1,1,1,1,1]` ends up in `|ÊÊÊÊÊÊÊÊÊ÷` instead of `|Êþþþ^5÷`
- `serde_zipson` object template feature not working yet, so `[{"key":"value1"},{"key":"value2"}]` ends up in `|{¨key¨¨value1¨}{ß0¨value2¨}÷` instead of `|¦¨key¨‡¨value1¨¨value2¨—÷`

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>