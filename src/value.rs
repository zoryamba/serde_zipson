use indexmap::{IndexMap};

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i64),
    Float(f64)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map<K: PartialEq + Eq + std::hash::Hash, V: PartialEq> {
    _map: MapImpl<K, V>,
}

type MapImpl<K, V> = IndexMap<K, V>;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Undefined,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}