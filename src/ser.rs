use std::result;
use chrono::DateTime;
use indexmap::IndexMap;
use serde::ser::{self, Serialize};
use crate::constants::{ARRAY_END_TOKEN, ARRAY_START_TOKEN, BASE_62, BOOLEAN_FALSE_TOKEN, BOOLEAN_TRUE_TOKEN, DATE_LOW_PRECISION, DATE_REGEX, DATE_TOKEN, ESCAPE_CHARACTER, ESCAPED_ESCAPE_CHARACTER, ESCAPED_STRING_TOKEN, ESCAPED_UNREFERENCED_STRING_TOKEN, FLOAT_COMPRESSION_PRECISION, FLOAT_FULL_PRECISION_DELIMITER, FLOAT_REDUCED_PRECISION_DELIMITER, FLOAT_TOKEN, INTEGER_SMALL_EXCLUSIVE_BOUND_LOWER, INTEGER_SMALL_EXCLUSIVE_BOUND_UPPER, INTEGER_SMALL_TOKEN_ELEMENT_OFFSET, INTEGER_SMALL_TOKENS, INTEGER_TOKEN, LP_DATE_TOKEN, NULL_TOKEN, STRING_TOKEN, UNREFERENCED_DATE_TOKEN, UNREFERENCED_FLOAT_TOKEN, UNREFERENCED_INTEGER_TOKEN, UNREFERENCED_LP_DATE_TOKEN, UNREFERENCED_STRING_TOKEN};
use crate::error::{Error, Result};
use crate::value::{Number, Value};

struct InvertedIndex {
    integers: IndexMap<i64, String>,
    floats: IndexMap<String, String>,
    strings: IndexMap<String, String>,
    dates: IndexMap<String, String>,
    lp_dates: IndexMap<String, String>,
}

pub struct Serializer {
    output: String,
    index: InvertedIndex,
    full_precision_floats: bool,
    detect_utc_timestamps: bool,
}

impl Serializer {
    fn new(full_precision_floats: bool, detect_utc_timestamps: bool) -> Self {
        Serializer {
            output: String::new(),
            index: InvertedIndex {
                integers: IndexMap::new(),
                floats: IndexMap::new(),
                strings: IndexMap::new(),
                dates: IndexMap::new(),
                lp_dates: IndexMap::new(),
            },
            full_precision_floats,
            detect_utc_timestamps,
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

    fn serialize_float(&self, v: f64) -> Result<String> {
        return if self.full_precision_floats {
            let v_string = v.to_string();
            let split: Vec<&str> = v_string.split('.').collect();
            let operator = if split[0] == "-0" && split.len() > 1 { "-" } else { "" };
            Ok([
                operator.to_string(),
                self.serialize_integer(split[0].parse::<i64>().unwrap())?,
                FLOAT_FULL_PRECISION_DELIMITER.to_string(),
                if split.len() > 1 { split[1].to_string() } else { '0'.to_string() }
            ].join(""))
        } else {
            let v_string = v.to_string();
            let split: Vec<&str> = v_string.split('.').collect();
            let integer = if split[0] == "-0" { 0 } else { split[0].parse::<i64>().unwrap() };
            let fraction = ((v % 1.) * FLOAT_COMPRESSION_PRECISION).round() as i64;

            Ok([
                self.serialize_integer(integer)?,
                FLOAT_REDUCED_PRECISION_DELIMITER.to_string(),
                self.serialize_integer(fraction)?,
            ].join(""))
        };
    }

    fn is_date(&self, v: &str) -> bool {
        DATE_REGEX.is_match(v)
    }

    fn serialize_date(&mut self, v: &str) -> Result<()> {
        let date_result = DateTime::parse_from_rfc3339(v);
        match date_result {
            Ok(datetime) => {
                let millis = datetime.timestamp_millis();

                let low_precision_date = millis as f64 / DATE_LOW_PRECISION;
                let is_low_precision = low_precision_date % 1_f64 == 0_f64;

                if is_low_precision {
                    let res = self.serialize_integer(low_precision_date as i64)?;
                    let index = self.serialize_integer(self.index.lp_dates.len() as i64)?;

                    if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
                        self.index.lp_dates.insert(v.to_string(), res.clone());
                        self.output.push(LP_DATE_TOKEN);
                        self.output += &res;
                    } else {
                        self.output.push(UNREFERENCED_LP_DATE_TOKEN);
                        self.output += &res;
                    }
                } else {
                    let res = self.serialize_integer(millis)?;
                    let index = self.serialize_integer(self.index.dates.len() as i64)?;

                    if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
                        self.index.dates.insert(v.to_string(), res.clone());
                        self.output.push(DATE_TOKEN);
                        self.output += &res;
                    } else {
                        self.output.push(UNREFERENCED_DATE_TOKEN);
                        self.output += &res;
                    }

                }
            }
            _ => self.serialize_string(v)?
        };

        Ok(())
    }

    fn serialize_string(&mut self, v: &str) -> Result<()> {
        let index = self.serialize_integer(self.index.strings.len() as i64)?;
        let v = v.replace(ESCAPE_CHARACTER, &ESCAPED_ESCAPE_CHARACTER);
        let escaped = v.replace(STRING_TOKEN, &ESCAPED_STRING_TOKEN);

        if index.chars().collect::<Vec<_>>().len() < escaped.chars().collect::<Vec<_>>().len() {
            self.index.strings.insert(v.to_string(), escaped.clone());
            self.output.push(STRING_TOKEN);
            self.output += &escaped;
            self.output.push(STRING_TOKEN);
        } else {
            self.output.push(UNREFERENCED_STRING_TOKEN);
            self.output += &v.replace(UNREFERENCED_STRING_TOKEN, &ESCAPED_UNREFERENCED_STRING_TOKEN);
            self.output.push(UNREFERENCED_STRING_TOKEN);
        }

        Ok(())
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
            let res = self.serialize_integer(v)?;
            let index = self.serialize_integer(self.index.integers.len() as i64)?;

            if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
                self.index.integers.insert(v, res.clone());
                self.output.push(INTEGER_TOKEN);
                self.output += &res;
            } else {
                self.output.push(UNREFERENCED_INTEGER_TOKEN);
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

    fn serialize_f64(self, v: f64) -> Result<()> {
        let res = self.serialize_float(v)?;
        let index = self.serialize_integer(self.index.floats.len() as i64)?;

        if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
            self.index.floats.insert(v.to_string(), res.clone());
            self.output.push(FLOAT_TOKEN);
            self.output += &res;
        } else {
            self.output.push(UNREFERENCED_FLOAT_TOKEN);
            self.output += &res;
        }

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        if self.detect_utc_timestamps && self.is_date(v) {
            return self.serialize_date(v);
        }

        self.serialize_string(v)
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
            Value::String(v) => serializer.serialize_str(v),
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
            Self::Float(v) => serializer.serialize_f64(*v),
        }
    }
}

pub fn to_string<T>(value: &T, full_precision_floats: bool, detect_utc_timestamps: bool) -> Result<String>
    where
        T: Serialize,
{
    let mut serializer = Serializer::new(full_precision_floats, detect_utc_timestamps);
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}