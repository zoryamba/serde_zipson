use indexmap::{IndexMap};

type N = String;

#[derive(Debug, PartialEq)]
pub struct Number {
    _n: N,
}

#[derive(Debug, PartialEq)]
pub struct Map<K: PartialEq + Eq + std::hash::Hash, V: PartialEq> {
    _map: MapImpl<K, V>,
}

type MapImpl<K, V> = IndexMap<K, V>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}