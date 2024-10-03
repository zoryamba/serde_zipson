use std::vec;
use indexmap::IndexMap;
use serde::{de, forward_to_deserialize_any};
use serde::de::{DeserializeSeed, Visitor};
use crate::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Undefined,
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>),
}

impl<'de> de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        match self {
            Value::Undefined => visitor.visit_unit(),
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Number(n) => match n {
                Number::Float(f) => visitor.visit_f64(f),
                Number::Int(i) => visitor.visit_i64(i),
            }
            Value::String(v) => visitor.visit_string(v),
            Value::Array(v) => visitor.visit_seq(&mut SeqAccess::new(v)),
            Value::Object(v) => visitor.visit_map(&mut MapAccess::new(v)),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str string unit seq tuple map
    }

    fn deserialize_char<V>(self, _visitor: V) -> crate::error::Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>
    {
        unimplemented!()
    }
}

struct SeqAccess {
    iter: vec::IntoIter<Value>,
}

impl SeqAccess {
    fn new(vec: Vec<Value>) -> Self {
        Self { iter: vec.into_iter() }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }
}

struct MapAccess {
    iter: <IndexMap<String, Value> as IntoIterator>::IntoIter,
    value: Option<Value>,
}

impl MapAccess {
    fn new(map: IndexMap<String, Value>) -> Self {
        Self {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                let key_de = Value::String(key);
                seed.deserialize(key_de).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }
}