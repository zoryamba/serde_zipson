use std::fmt;
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use indexmap::IndexMap;
use serde::de::{self, Deserialize, DeserializeSeed, Visitor};
use crate::constants::{ARRAY_END_TOKEN, ARRAY_START_TOKEN, BOOLEAN_FALSE_TOKEN, BOOLEAN_TRUE_TOKEN, DATE_LOW_PRECISION, DATE_TOKEN, DELIMITING_TOKENS_THRESHOLD, ESCAPE_CHARACTER, FLOAT_COMPRESSION_PRECISION, FLOAT_FULL_PRECISION_DELIMITER, FLOAT_REDUCED_PRECISION_DELIMITER, FLOAT_TOKEN, INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER, INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER, INTEGER_SMALL_TOKEN_OFFSET, INTEGER_TOKEN, LP_DATE_TOKEN, NULL_TOKEN, OBJECT_END_TOKEN, OBJECT_START_TOKEN, REF_DATE_TOKEN, REF_FLOAT_TOKEN, REF_INTEGER_TOKEN, REF_LP_DATE_TOKEN, REF_STRING_TOKEN, STRING_TOKEN, UNREFERENCED_DATE_TOKEN, UNREFERENCED_FLOAT_TOKEN, UNREFERENCED_INTEGER_TOKEN, UNREFERENCED_LP_DATE_TOKEN, UNREFERENCED_STRING_TOKEN};
use crate::error::{Error, Result};
use crate::value::{Number, Value};

pub struct OrderedIndex {
    strings: Vec<String>,
    integers: Vec<i64>,
    floats: Vec<f64>,
    dates: Vec<String>,
    lp_dates: Vec<String>,
}

pub struct Deserializer<'de> {
    input: &'de str,
    index: OrderedIndex,
}

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Self {
            input,
            index: OrderedIndex {
                strings: vec![],
                integers: vec![],
                floats: vec![],
                dates: vec![],
                lp_dates: vec![],
            },
        }
    }

    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn deserialize_integer<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        let token = self.next_char()?;
        match token {
            ch if (ch as u8) > INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER && (ch as u8) < INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER => {
                return visitor.visit_i16(ch as i16 - INTEGER_SMALL_TOKEN_OFFSET)
            },
            UNREFERENCED_INTEGER_TOKEN => {
                return visitor.visit_i64(self.parse_integer()?);
            }
            INTEGER_TOKEN => {
                let val = self.parse_integer()?;
                self.index.integers.push(val);
                return visitor.visit_i64(val);
            },
            _ => Err(Error::ExpectedInteger)
        }
    }

    fn deserialize_ref_integer<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        self.next_char()?;
        let ref_index = self.parse_integer()? as usize;

        return visitor.visit_i64(*self.index.integers.get(ref_index).unwrap());
    }

    fn parse_integer(&mut self) -> Result<i64>
    {
        let mut ch = self.next_char()?;

        if ch == '0' {
            return Ok(0);
        }

        let negative = ch == '-';

        let mut value = 0;

        fn parse_char(ch: char) -> i64 {
            let code = ch as i64;
            let mut current = code - 48;
            if code >= 97 {
                current -= 13
            } else if code >= 65 {
                current -= 7
            }
            current
        }

        if !negative {
            value = parse_char(ch);
        }

        loop {
            let res = self.peek_char();
            match res {
                Ok(ch) => {
                    if ch as u8 > DELIMITING_TOKENS_THRESHOLD {
                        break;
                    }
                    if ch == FLOAT_FULL_PRECISION_DELIMITER || ch == FLOAT_REDUCED_PRECISION_DELIMITER {
                        break;
                    }
                },
                Err(Error::Eof) => {
                    break;
                }
                Err(err) => return Err(err)
            }

            ch = self.next_char()?;
            value *= 62;
            value += parse_char(ch);
        }

        if negative {
            value = -value;
        }

        Ok(value)
    }

    fn deserialize_float(&mut self) -> Result<f64>
    {
        let token = self.next_char()?;

        let value = self.parse_float()?;

        if token == FLOAT_TOKEN {
            self.index.floats.push(value);
        }

        Ok(value)
    }

    fn deserialize_ref_float<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        self.next_char()?;
        let ref_index = self.parse_integer()? as usize;

        return visitor.visit_f64(*self.index.floats.get(ref_index).unwrap());
    }

    fn parse_float(&mut self) -> Result<f64>
    {
        let negative = self.peek_char()? == '-';

        let integer = self.parse_integer()?;

        let delimiter_token = self.next_char()?;

        let fraction: f64 = match delimiter_token {
            FLOAT_REDUCED_PRECISION_DELIMITER => self.parse_integer()? as f64 / FLOAT_COMPRESSION_PRECISION,
            FLOAT_FULL_PRECISION_DELIMITER => {
                let mut res = if negative { "-0." } else { "0." }.to_string();

                loop {
                    let ch = self.peek_char();
                    match ch {
                        Ok(ch) => match ch.to_digit(10) {
                            Some(_) => {
                                self.next_char()?;
                                res.push(ch);
                            },
                            None => break,
                        },
                        Err(Error::Eof) => {
                            break;
                        }
                        Err(err) => return Err(err)
                    }


                }

                match res.parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => return Err(Error::ExpectedFloat)
                }
            }
            _ => return Err(Error::ExpectedFloat)
        };

        let res = integer as f64 + fraction;

        Ok(res)
    }

    fn parse_string(&mut self) -> Result<String> {
        let token = self.next_char()?;

        match token {
            REF_STRING_TOKEN => {
                let ref_index = self.parse_integer()? as usize;

                return Ok(self.index.strings.get(ref_index).unwrap().clone());
            }
            _ => {
                let mut chars: Vec<char> = vec![];

                loop {
                    let mut ch = self.next_char()?;
                    let mut escaped = 0;

                    while ch == ESCAPE_CHARACTER {
                        escaped += 1;
                        ch = self.next_char()?;
                    }

                    if escaped > 0 {
                        for _ in 0..escaped / 2 {
                            chars.push(ESCAPE_CHARACTER);
                        }
                        if escaped % 2 == 1 && ch != token {
                            return Err(Error::ExpectedEscapedToken);
                        }
                    }

                    if escaped % 2 == 0 && ch == token {
                        break;
                    }

                    chars.push(ch);
                }

                let res = String::from_iter(chars);

                if token == STRING_TOKEN {
                    self.index.strings.push(res.clone());
                }

                Ok(res)
            }
        }
    }

    fn deserialize_date<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        let token = self.next_char()?;

        let integer = self.parse_integer()?;

        let seconds = integer / 1000;
        let millis = (integer % 1000) as u32;

        let nt = NaiveDateTime::from_timestamp_opt(seconds, millis * 1_000_000).unwrap();
        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(nt, Utc);
        let value = dt.to_rfc3339_opts(SecondsFormat::Millis, true);

        if token == DATE_TOKEN {
            self.index.dates.push(value.clone());
        }

        visitor.visit_string(value)
    }

    fn deserialize_lp_date<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        let token = self.next_char()?;

        let integer = self.parse_integer()? * DATE_LOW_PRECISION;

        let nt = NaiveDateTime::from_timestamp_opt(integer, 0).unwrap();
        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(nt, Utc);
        let value = dt.to_rfc3339_opts(SecondsFormat::Millis, true);

        if token == LP_DATE_TOKEN {
            self.index.lp_dates.push(value.clone());
        }

        visitor.visit_string(value)
    }

    fn deserialize_ref_date<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        self.next_char()?;
        let ref_index = self.parse_integer()? as usize;

        return visitor.visit_string(self.index.dates.get(ref_index).unwrap().clone());
    }

    fn deserialize_ref_lp_date<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        self.next_char()?;
        let ref_index = self.parse_integer()? as usize;

        return visitor.visit_string(self.index.lp_dates.get(ref_index).unwrap().clone());
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Value, D::Error>
        where D: de::Deserializer<'de>
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result
            {
                formatter.write_str("a string key")
            }

            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            {
                Ok(Value::Null)
            }

            fn visit_bool<E>(self, bool: bool) -> std::result::Result<Self::Value, E>
            {
                Ok(Value::Bool(bool))
            }

            fn visit_string<E>(self, str: String) -> std::result::Result<Value, E>
            {
                Ok(Value::String(str))
            }

            fn visit_i64<E>(self, number: i64) -> std::result::Result<Value, E>
            {
                Ok(Value::Number(Number::Int(number)))
            }

            fn visit_f64<E>(self, number: f64) -> std::result::Result<Value, E>
            {
                Ok(Value::Number(Number::Float(number)))
            }

            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Array(vec))
            }

            fn visit_map<A>(self, mut seq: A) -> std::result::Result<Value, A::Error>
                where
                    A: de::MapAccess<'de>,
            {
                let mut map = IndexMap::new();

                while let Some((key, value)) = seq.next_entry()? {
                    map.insert(key, value);
                }

                Ok(Value::Object(map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        match self.peek_char()? {
            NULL_TOKEN => self.deserialize_unit(visitor),
            BOOLEAN_TRUE_TOKEN => self.deserialize_bool(visitor),
            BOOLEAN_FALSE_TOKEN => self.deserialize_bool(visitor),
            ch if (ch as u8) > INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER && (ch as u8) < INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER => self.deserialize_integer(visitor),
            UNREFERENCED_INTEGER_TOKEN => self.deserialize_integer(visitor),
            INTEGER_TOKEN => self.deserialize_integer(visitor),
            REF_INTEGER_TOKEN => self.deserialize_ref_integer(visitor),
            UNREFERENCED_FLOAT_TOKEN => self.deserialize_f64(visitor),
            FLOAT_TOKEN => self.deserialize_f64(visitor),
            REF_FLOAT_TOKEN => self.deserialize_ref_float(visitor),
            UNREFERENCED_STRING_TOKEN => self.deserialize_str(visitor),
            STRING_TOKEN => self.deserialize_str(visitor),
            REF_STRING_TOKEN => self.deserialize_str(visitor),
            DATE_TOKEN => self.deserialize_date(visitor),
            UNREFERENCED_DATE_TOKEN => self.deserialize_date(visitor),
            REF_DATE_TOKEN => self.deserialize_ref_date(visitor),
            LP_DATE_TOKEN => self.deserialize_lp_date(visitor),
            UNREFERENCED_LP_DATE_TOKEN => self.deserialize_lp_date(visitor),
            REF_LP_DATE_TOKEN => self.deserialize_ref_lp_date(visitor),
            ARRAY_START_TOKEN => self.deserialize_seq(visitor),
            OBJECT_START_TOKEN => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let ch = self.next_char()?;
        match ch {
            BOOLEAN_TRUE_TOKEN => visitor.visit_bool(true),
            BOOLEAN_FALSE_TOKEN => visitor.visit_bool(false),
            _ => Err(Error::ExpectedBoolean)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_i64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_u64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_f64(self.deserialize_float()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        // Parse a string, check that it is one character, call `visit_char`.
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.next_char()?;
        visitor.visit_unit()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let _ = self.next_char()?;
        visitor.visit_seq(SeqAccess::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        let _ = self.next_char()?;
        visitor.visit_map(MapAccess::new(self))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de: 'a> SeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> std::result::Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.de.peek_char()? {
            ARRAY_END_TOKEN => {
                self.de.next_char()?;
                Ok(None)
            }
            _ => Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }
}

struct MapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de: 'a> MapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
        where K: DeserializeSeed<'de>
    {
        match self.de.peek_char()? {
            OBJECT_END_TOKEN => {
                self.de.next_char()?;
                Ok(None)
            }
            _ => Ok(Some(seed.deserialize(&mut *self.de)?))
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
        where V: DeserializeSeed<'de>
    {
        Ok(seed.deserialize(&mut *self.de)?)
    }
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}