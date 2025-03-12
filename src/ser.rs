use crate::constants::{
    ARRAY_END_TOKEN, ARRAY_REPEAT_COUNT_THRESHOLD, ARRAY_REPEAT_MANY_TOKEN, ARRAY_REPEAT_TOKEN,
    ARRAY_START_TOKEN, BASE_62, BOOLEAN_FALSE_TOKEN, BOOLEAN_TRUE_TOKEN, DATE_LOW_PRECISION,
    DATE_REGEX, DATE_TOKEN, ESCAPED_ESCAPE_CHARACTER, ESCAPED_STRING_TOKEN,
    ESCAPED_UNREFERENCED_STRING_TOKEN, ESCAPE_CHARACTER, FLOAT_COMPRESSION_PRECISION,
    FLOAT_FULL_PRECISION_DELIMITER, FLOAT_REDUCED_PRECISION_DELIMITER, FLOAT_TOKEN,
    INTEGER_SMALL_EXCLUSIVE_BOUND_LOWER, INTEGER_SMALL_EXCLUSIVE_BOUND_UPPER, INTEGER_SMALL_TOKENS,
    INTEGER_SMALL_TOKEN_ELEMENT_OFFSET, INTEGER_TOKEN, LP_DATE_TOKEN, NULL_TOKEN, OBJECT_END_TOKEN,
    OBJECT_START_TOKEN, REF_DATE_TOKEN, REF_FLOAT_TOKEN, REF_INTEGER_TOKEN, REF_LP_DATE_TOKEN,
    REF_STRING_TOKEN, STRING_TOKEN, UNREFERENCED_DATE_TOKEN, UNREFERENCED_FLOAT_TOKEN,
    UNREFERENCED_INTEGER_TOKEN, UNREFERENCED_LP_DATE_TOKEN, UNREFERENCED_STRING_TOKEN,
};
use crate::error::{Error, Result};
use crate::value::{Number, Value};
use chrono::DateTime;
use indexmap::IndexMap;
use serde::ser::{self, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

struct InvertedIndex {
    integers: IndexMap<i64, String>,
    floats: IndexMap<String, String>,
    strings: IndexMap<String, String>,
    dates: IndexMap<String, String>,
    lp_dates: IndexMap<String, String>,
}

impl InvertedIndex {
    fn new() -> Self {
        InvertedIndex {
            integers: IndexMap::new(),
            floats: IndexMap::new(),
            strings: IndexMap::new(),
            dates: IndexMap::new(),
            lp_dates: IndexMap::new(),
        }
    }
}

pub struct Serializer {
    output: String,
    // TODO: pass serializer to SerializeSeq and get rid of Rc<RefCell<_>>
    index: Rc<RefCell<InvertedIndex>>,
    full_precision_floats: bool,
    detect_utc_timestamps: bool,
}

impl Serializer {
    fn new(
        full_precision_floats: bool,
        detect_utc_timestamps: bool,
        index: Option<Rc<RefCell<InvertedIndex>>>,
    ) -> Self {
        Serializer {
            output: String::new(),
            index: if let Some(index) = index {
                index
            } else {
                Rc::new(RefCell::new(InvertedIndex::new()))
            },
            full_precision_floats,
            detect_utc_timestamps,
        }
    }

    fn serialize_integer(v: i64) -> Result<String> {
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

        Ok(result)
    }

    fn serialize_float(&self, v: f64) -> Result<String> {
        if self.full_precision_floats {
            let v_string = v.to_string();
            let split: Vec<&str> = v_string.split('.').collect();
            let operator = if split[0] == "-0" && split.len() > 1 {
                "-"
            } else {
                ""
            };
            Ok([
                operator.to_string(),
                Self::serialize_integer(split[0].parse::<i64>().unwrap())?,
                FLOAT_FULL_PRECISION_DELIMITER.to_string(),
                if split.len() > 1 {
                    split[1].to_string()
                } else {
                    '0'.to_string()
                },
            ]
            .join(""))
        } else {
            let v_string = v.to_string();
            let split: Vec<&str> = v_string.split('.').collect();
            let integer = if split[0] == "-0" {
                0
            } else {
                split[0].parse::<i64>().unwrap()
            };
            let fraction = ((v % 1.) * FLOAT_COMPRESSION_PRECISION).round() as i64;

            Ok([
                Self::serialize_integer(integer)?,
                FLOAT_REDUCED_PRECISION_DELIMITER.to_string(),
                Self::serialize_integer(fraction)?,
            ]
            .join(""))
        }
    }

    fn serialize_date(&mut self, v: &str) -> Result<()> {
        let date_result = DateTime::parse_from_rfc3339(v);

        match date_result {
            Ok(date) => {
                let millis = date.timestamp_millis();

                let low_precision_date = millis as f64 / DATE_LOW_PRECISION;
                let is_low_precision = low_precision_date % 1_f64 == 0_f64;

                if is_low_precision {
                    if self.try_index_lp_date(v) {
                        return Ok(());
                    }

                    let res = Self::serialize_integer(low_precision_date as i64)?;
                    let index = Self::serialize_integer(self.get_lp_dates_len() as i64)?;

                    if index.chars().collect::<Vec<_>>().len()
                        < res.chars().collect::<Vec<_>>().len()
                    {
                        self.add_lp_date(v.to_string(), index);
                        self.output.push(LP_DATE_TOKEN);
                        self.output += &res;
                    } else {
                        self.output.push(UNREFERENCED_LP_DATE_TOKEN);
                        self.output += &res;
                    }
                } else {
                    if self.try_index_date(v) {
                        return Ok(());
                    }

                    let res = Self::serialize_integer(millis)?;
                    let index = Self::serialize_integer(self.get_dates_len() as i64)?;

                    if index.chars().collect::<Vec<_>>().len()
                        < res.chars().collect::<Vec<_>>().len()
                    {
                        self.add_date(v.to_string(), index);
                        self.output.push(DATE_TOKEN);
                        self.output += &res;
                    } else {
                        self.output.push(UNREFERENCED_DATE_TOKEN);
                        self.output += &res;
                    }
                }

                Ok(())
            }
            _ => self.serialize_string(v),
        }
    }

    fn serialize_string(&mut self, v: &str) -> Result<()> {
        if self.try_index_string(v) {
            return Ok(());
        }

        let escaped = v.replace(ESCAPE_CHARACTER, &ESCAPED_ESCAPE_CHARACTER);
        let escaped_token = escaped.replace(STRING_TOKEN, &ESCAPED_STRING_TOKEN);
        let index = Self::serialize_integer(self.get_strings_len() as i64)?;

        if index.chars().collect::<Vec<_>>().len() < escaped_token.chars().collect::<Vec<_>>().len()
        {
            self.add_string(v.to_string(), index);
            self.output.push(STRING_TOKEN);
            self.output += &escaped_token;
            self.output.push(STRING_TOKEN);
        } else {
            self.output.push(UNREFERENCED_STRING_TOKEN);
            self.output += &escaped.replace(
                UNREFERENCED_STRING_TOKEN,
                &ESCAPED_UNREFERENCED_STRING_TOKEN,
            );
            self.output.push(UNREFERENCED_STRING_TOKEN);
        }

        Ok(())
    }

    fn add_integer(&self, key: i64, value: String) {
        self.index.borrow_mut().integers.insert(key, value);
    }
    fn try_index_integer(&mut self, key: &i64) -> bool {
        let index = self.index.borrow();
        let found_ref = index.integers.get(key);

        if let Some(found) = found_ref {
            self.output.push(REF_INTEGER_TOKEN);
            self.output += &found;
            true
        } else {
            false
        }
    }
    fn get_integers_len(&self) -> usize {
        self.index.borrow().integers.len()
    }
    fn add_float(&self, key: String, value: String) {
        self.index.borrow_mut().floats.insert(key, value);
    }
    fn try_index_float(&mut self, key: &str) -> bool {
        let index = self.index.borrow();
        let found_ref = index.floats.get(key);

        if let Some(found) = found_ref {
            self.output.push(REF_FLOAT_TOKEN);
            self.output += &found;
            true
        } else {
            false
        }
    }
    fn get_floats_len(&self) -> usize {
        self.index.borrow().floats.len()
    }
    fn add_string(&self, key: String, value: String) {
        self.index.borrow_mut().strings.insert(key, value);
    }
    fn try_index_string(&mut self, key: &str) -> bool {
        let index = self.index.borrow();

        let found_ref = index.strings.get(key);
        if let Some(found) = found_ref {
            self.output.push(REF_STRING_TOKEN);
            self.output += &found;
            true
        } else {
            false
        }
    }
    fn get_strings_len(&self) -> usize {
        self.index.borrow().strings.len()
    }
    fn add_date(&self, key: String, value: String) {
        self.index.borrow_mut().dates.insert(key, value);
    }
    fn try_index_date(&mut self, key: &str) -> bool {
        let index = self.index.borrow();

        let found_ref = index.dates.get(key);
        if let Some(found) = found_ref {
            self.output.push(REF_DATE_TOKEN);
            self.output += &found;
            true
        } else {
            false
        }
    }
    fn get_dates_len(&self) -> usize {
        self.index.borrow().dates.len()
    }
    fn add_lp_date(&self, key: String, value: String) {
        self.index.borrow_mut().lp_dates.insert(key, value);
    }
    fn try_index_lp_date(&mut self, key: &str) -> bool {
        let index = self.index.borrow();

        let found_ref = index.lp_dates.get(key);
        if let Some(found) = found_ref {
            self.output.push(REF_LP_DATE_TOKEN);
            self.output += &found;
            true
        } else {
            false
        }
    }
    fn get_lp_dates_len(&self) -> usize {
        self.index.borrow().lp_dates.len()
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SerializeSeq<'a>;
    type SerializeTuple = SerializeSeq<'a>;
    type SerializeTupleStruct = SerializeSeq<'a>;
    type SerializeTupleVariant = Self;
    type SerializeMap = SerializeSeq<'a>;
    type SerializeStruct = SerializeSeq<'a>;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push(if v {
            BOOLEAN_TRUE_TOKEN
        } else {
            BOOLEAN_FALSE_TOKEN
        });
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
            self.output
                .push(INTEGER_SMALL_TOKENS[(v + INTEGER_SMALL_TOKEN_ELEMENT_OFFSET) as usize]);
            return Ok(());
        }
        if self.try_index_integer(&v) {
            return Ok(());
        }

        let res = Serializer::serialize_integer(v)?;
        let index = Serializer::serialize_integer(self.get_integers_len() as i64)?;

        if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
            self.add_integer(v, index);
            self.output.push(INTEGER_TOKEN);
            self.output += &res;
        } else {
            self.output.push(UNREFERENCED_INTEGER_TOKEN);
            self.output += &res;
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

        if self.try_index_float(&res) {
            return Ok(());
        }

        let index = Serializer::serialize_integer(self.get_floats_len() as i64)?;

        if index.chars().collect::<Vec<_>>().len() < res.chars().collect::<Vec<_>>().len() {
            self.add_float(res.clone(), index);
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
        if self.detect_utc_timestamps && DATE_REGEX.is_match(v) {
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

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
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
        // TODO: serialize enums
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.push(ARRAY_START_TOKEN);
        Ok(SerializeSeq {
            output: &mut self.output,
            index: self.index.clone(),
            full_precision_floats: self.full_precision_floats,
            detect_utc_timestamps: self.detect_utc_timestamps,
            last_value: None,
            repeat_count: 0,
        })
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
        // TODO: serialize enums
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output.push(OBJECT_START_TOKEN);
        Ok(SerializeSeq {
            output: &mut self.output,
            index: self.index.clone(),
            full_precision_floats: self.full_precision_floats,
            detect_utc_timestamps: self.detect_utc_timestamps,
            last_value: None,
            repeat_count: 0,
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // TODO: serialize enums
        unimplemented!()
    }
}

pub struct SerializeSeq<'a> {
    output: &'a mut String,
    index: Rc<RefCell<InvertedIndex>>,
    full_precision_floats: bool,
    detect_utc_timestamps: bool,

    last_value: Option<String>,
    repeat_count: i64,
}

impl SerializeSeq<'_> {
    fn handle_last_value(&mut self, is_repeat: bool) -> Result<()> {
        if let Some(ref last_value) = self.last_value {
            if self.repeat_count == 0 {
                self.output.push_str(last_value);
            } else if self.repeat_count < ARRAY_REPEAT_COUNT_THRESHOLD {
                self.output.push(ARRAY_REPEAT_TOKEN);
            } else if self.repeat_count >= ARRAY_REPEAT_COUNT_THRESHOLD {
                if !is_repeat {
                    self.output.push(ARRAY_REPEAT_MANY_TOKEN);
                    self.output.push_str(&Serializer::serialize_integer(
                        self.repeat_count - ARRAY_REPEAT_COUNT_THRESHOLD + 1,
                    )?);
                }
            }
        }

        Ok(())
    }
}

impl<'a> ser::SerializeSeq for SerializeSeq<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // TODO: implement to_value serializer and compare values instead of strings
        let value_string = to_string_nested(
            &value,
            self.full_precision_floats,
            self.detect_utc_timestamps,
            self.index.clone(),
        )?;

        match self.last_value {
            None => {
                // first array element
                self.last_value = Some(value_string);
            }
            Some(ref last_value) => {
                // after 1-st array element
                let is_repeat = *last_value == value_string;

                self.handle_last_value(is_repeat)?;

                if is_repeat {
                    // element repeated
                    self.repeat_count += 1;
                } else {
                    // element not repeated
                    self.last_value = Some(value_string);
                    self.repeat_count = 0;
                }
            }
        }

        Ok(())
    }

    fn end(mut self) -> Result<()> {
        self.handle_last_value(false)?;

        self.last_value = None;
        self.repeat_count = 0;

        self.output.push(ARRAY_END_TOKEN);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for SerializeSeq<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for SerializeSeq<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

// TODO: serialize enums
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

impl<'a> ser::SerializeMap for SerializeSeq<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key_string = to_string_nested(
            &key,
            self.full_precision_floats,
            self.detect_utc_timestamps,
            self.index.clone(),
        )?;
        self.output.push_str(&key_string);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let value_string = to_string_nested(
            &value,
            self.full_precision_floats,
            self.detect_utc_timestamps,
            self.index.clone(),
        )?;
        self.output.push_str(&value_string);

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(OBJECT_END_TOKEN);
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for SerializeSeq<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let field_string = to_string_nested(
            &key,
            self.full_precision_floats,
            self.detect_utc_timestamps,
            self.index.clone(),
        )?;
        self.output.push_str(&field_string);
        let value_string = to_string_nested(
            &value,
            self.full_precision_floats,
            self.detect_utc_timestamps,
            self.index.clone(),
        )?;
        self.output.push_str(&value_string);

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(OBJECT_END_TOKEN);
        Ok(())
    }
}

// TODO: serialize enums
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
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Undefined => serializer.serialize_unit(),
            Value::Null => serializer.serialize_unit(),
            Value::Bool(v) => serializer.serialize_bool(*v),
            Value::Number(n) => n.serialize(serializer),
            Value::String(v) => serializer.serialize_str(v),
            Value::Array(v) => v.serialize(serializer),
            Value::Object(v) => v.serialize(serializer),
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Int(v) => serializer.serialize_i64(*v),
            Self::Float(v) => serializer.serialize_f64(*v),
        }
    }
}

pub fn to_string<T>(
    value: &T,
    full_precision_floats: bool,
    detect_utc_timestamps: bool,
) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(full_precision_floats, detect_utc_timestamps, None);
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

fn to_string_nested<T>(
    value: &T,
    full_precision_floats: bool,
    detect_utc_timestamps: bool,
    index: Rc<RefCell<InvertedIndex>>,
) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new(full_precision_floats, detect_utc_timestamps, Some(index));
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}
