use std::result;
use indexmap::IndexMap;
use serde::ser::{self, Serialize};
use crate::constants::{ARRAY_END_TOKEN, ARRAY_START_TOKEN, BASE_62, BOOLEAN_FALSE_TOKEN, BOOLEAN_TRUE_TOKEN, INTEGER_SMALL_EXCLUSIVE_BOUND_LOWER, INTEGER_SMALL_EXCLUSIVE_BOUND_UPPER, INTEGER_SMALL_TOKEN_ELEMENT_OFFSET, INTEGER_SMALL_TOKENS, INTEGER_TOKEN, NULL_TOKEN, UNREFERENCED_INTEGER_TOKEN};
use crate::error::{Error, Result};
use crate::value::{Number, Value};

struct InvertedIndex {
    integers: IndexMap<i64, String>
}

pub struct Serializer {
    output: String,
    index: InvertedIndex,
}

impl Serializer {
    fn new() -> Self {
        Serializer {
            output: String::new(),
            index: InvertedIndex {
                integers: IndexMap::new()
            }
        }
    }

    fn serialize_integer(&self, v: i64) -> Result<String> {
        if v == 0 {
            return Ok('0'.into());
        }

        let mut result = String::new();

        let mut modulus = if v < 0 { -v } else { v };
        let mut current;

        while modulus > 0 {
            current = modulus % 62;
            modulus -= current;
            modulus /= 62;
            result.insert(0, BASE_62[current as usize]);
        }
        if v < 0 {
            result.insert(0, '-');
        }

        return Ok(result);
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push(if v { BOOLEAN_TRUE_TOKEN } else { BOOLEAN_FALSE_TOKEN });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        if v > INTEGER_SMALL_EXCLUSIVE_BOUND_LOWER && v < INTEGER_SMALL_EXCLUSIVE_BOUND_UPPER {
            self.output.push(INTEGER_SMALL_TOKENS[(v + INTEGER_SMALL_TOKEN_ELEMENT_OFFSET) as usize]);
        } else {
            let mut res = self.serialize_integer(v)?;
            let index = self.serialize_integer(self.index.integers.len() as i64)?;

            if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
                self.index.integers.insert(v, res.clone());
                res.insert(0, INTEGER_TOKEN);
                self.output += &res;
            } else {
                res.insert(0, UNREFERENCED_INTEGER_TOKEN);
                self.output += &res;
            }
        }

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v as i64)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.push(NULL_TOKEN);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.push(ARRAY_START_TOKEN);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(ARRAY_END_TOKEN);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}


impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
        where
            T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Number(n) => n.serialize(serializer),
            Value::Array(v) => v.serialize(serializer),
            _ => unimplemented!()
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        match self {
            Self::Int(v) => serializer.serialize_i64(*v),
            Self::Float(_v) => unimplemented!()
        }
    }
}

pub fn to_string<T>(value: &T) -> Result<String>
    where
        T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}