use std::fmt;
use serde::Deserialize;
use serde::de::{self, Visitor};
use crate::constants::{STRING_TOKEN};
use crate::error::{Error, Result};
use crate::value::Value;

pub struct Deserializer<'de> {
    input: &'de str,
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

impl<'de> Deserializer<'de> {
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }

    fn peek_char(&mut self) -> Result<char> {
        self.input.chars().next().ok_or(Error::Eof)
    }

    fn next_char(&mut self) -> Result<char> {
        let ch = self.peek_char()?;
        self.input = &self.input[ch.len_utf8()..];
        Ok(ch)
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if self.next_char()? != STRING_TOKEN {
            return Err(Error::ExpectedString);
        }
        match self.input.find(STRING_TOKEN) {
            Some(len) => {
                let s = &self.input[..len];
                self.input = &self.input[len + STRING_TOKEN.len_utf8()..];
                Ok(s)
            }
            None => Err(Error::Eof),
        }
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

            fn visit_borrowed_str<E>(self, str: &str) -> std::result::Result<Self::Value, E>
            {
                Ok(Value::String(str.into()))
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
            ch if ch == STRING_TOKEN => self.deserialize_str(visitor),
            // 'n' => self.deserialize_unit(visitor),
            // 't' | 'f' => self.deserialize_bool(visitor),
            // '0'..='9' => self.deserialize_u64(visitor),
            // '-' => self.deserialize_i64(visitor),
            // ch if ch == ARRAY_START_TOKEN => self.deserialize_seq(visitor),
            // '{' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
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
        visitor.visit_borrowed_str(self.parse_string()?)
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

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
    {
        unimplemented!()
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