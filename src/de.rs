use std::fmt;
use serde::Deserialize;
use serde::de::{self, DeserializeSeed, Visitor};
use crate::constants::{ARRAY_END_TOKEN, ARRAY_START_TOKEN, ESCAPE_CHARACTER, INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER, INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER, INTEGER_SMALL_TOKEN_OFFSET, NULL_TOKEN, STRING_TOKEN, UNREFERENCED_STRING_TOKEN};
use crate::error::{Error, Result};
use crate::value::{Number, Value};

pub struct OrderedIndex {
    strings: Vec<String>,
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

    fn parse_string(&mut self) -> Result<String> {
        let token = self.next_char()?;
        if token != STRING_TOKEN && token != UNREFERENCED_STRING_TOKEN {
            return Err(Error::ExpectedString);
        }

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

    fn deserialize_number<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        match self.peek_char()? as u8 {
            ch if (ch > INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER) && (ch < INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER) => self.deserialize_small_integer(visitor),
            _ => Err(Error::ExpectedNumber)
        }
    }

    fn deserialize_small_integer<V>(&mut self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
    {
        let ch = self.next_char()? as i16;
        visitor.visit_i16(ch - INTEGER_SMALL_TOKEN_OFFSET)
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

            fn visit_string<E>(self, str: String) -> std::result::Result<Value, E>
            {
                Ok(Value::String(str))
            }

            fn visit_i64<E>(self, number: i64) -> std::result::Result<Value, E>
            {
                Ok(Value::Number(Number::Int(number)))
            }

            fn visit_seq<V>(self, mut seq: V) -> std::result::Result<Value, V::Error>
                where
                    V: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Array(vec))
            }

            fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
            {
                Ok(Value::Null)
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
            ch if ch == UNREFERENCED_STRING_TOKEN => self.deserialize_str(visitor),
            ch if ch == NULL_TOKEN => self.deserialize_unit(visitor),
            // 't' | 'f' => self.deserialize_bool(visitor),
            ch if (ch as u8 > INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_LOWER) && (INTEGER_SMALL_TOKEN_EXCLUSIVE_BOUND_UPPER > ch as u8) => self.deserialize_small_integer(visitor),
            // '0'..='9' => self.deserialize_u64(visitor),
            // '-' => self.deserialize_i64(visitor),
            ch if ch == ARRAY_START_TOKEN => self.deserialize_seq(visitor),
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
        self.deserialize_number(visitor)
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
        self.deserialize_i64(visitor)
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
        self.deserialize_number(visitor)
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
        let _ = self.next_char();
        visitor.visit_seq(SeqAccess::new(self))
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

struct SeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de: 'a> SeqAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        SeqAccess { de }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> std::result::Result<Option<T::Value>, Self::Error>
        where T: DeserializeSeed<'de>
    {
        match self.de.peek_char()? {
            ch if ch == ARRAY_END_TOKEN => {
                self.de.next_char()?;
                return Ok(None);
            },
            _ => {},
        };

        Ok(Some(seed.deserialize(&mut *self.de)?))
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