use std::any::type_name;

use super::macros::deserialize_unimplemented;
use super::map::FixMapAccess;
use super::read::{Read, SliceRead};
use crate::error::{Error, Result};
use serde::de::{self};

pub struct Deserializer<R> {
    pub(crate) read: R,
}

impl<'de, R: Read<'de>> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Deserializer { read }
    }
}
impl<'a> Deserializer<SliceRead<'a>> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self::new(SliceRead::new(slice))
    }
}
impl<'de, R: Read<'de>> Deserializer<R> {
    /// The [Self::end] should be called after a value is fully deserialized to check if there are any trailing bytes.
    pub fn end(&mut self) -> Result<()> {
        if self.read.peek()?.is_none() {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }
}

fn from_trait<'de, R: Read<'de>, T: de::Deserialize<'de>>(read: R) -> Result<T> {
    let mut deserializer = Deserializer::new(read);
    let t = T::deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(t)
}
pub fn from_slice<'a, T: de::Deserialize<'a>>(slice: &'a [u8]) -> Result<T> {
    from_trait(SliceRead::new(slice))
}

impl<'de, 'any, R: Read<'de>> de::Deserializer<'de> for &'any mut Deserializer<R> {
    type Error = Error;

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // TODO is ? operator still causes performance issues because of implied into Serde suggests that it does
        let int = self.read.parse_unsigned()?;
        visitor.visit_u64(int)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let slice = self.read.parse_fix_value()?;
        // TODO should this unchecked be optional when fix is running ascii explicitly via a feature setting
        let string = unsafe { std::string::String::from_utf8_unchecked(slice.to_vec()) };
        visitor.visit_string(string)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_map(FixMapAccess::new(self))
    }
    fn deserialize_struct<V: de::Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    deserialize_unimplemented!(
        Deserializer<R>,
        deserialize_any(self, _visitor: V),
        deserialize_bool(self, _visitor: V),
        deserialize_i8(self, _visitor: V),
        deserialize_i16(self, _visitor: V),
        deserialize_i32(self, _visitor: V),
        deserialize_i64(self, _visitor: V),
        deserialize_u8(self, _visitor: V),
        deserialize_u16(self, _visitor: V),
        deserialize_u32(self, _visitor: V),
        deserialize_f32(self, _visitor: V),
        deserialize_f64(self, _visitor: V),
        deserialize_char(self, _visitor: V),
        deserialize_str(self, _visitor: V),
        deserialize_bytes(self, _visitor: V),
        deserialize_byte_buf(self, _visitor: V),
        deserialize_option(self, _visitor: V),
        deserialize_unit(self, _visitor: V),
        deserialize_ignored_any(self, _visitor: V),
        deserialize_identifier(self, _visitor: V),
        deserialize_seq(self, _visitor: V),
    );
    deserialize_unimplemented!(
        Deserializer<R>,
        deserialize_unit_struct(self, _name: &'static str, _visitor: V)
    );
    deserialize_unimplemented!(
        Deserializer<R>,
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
    deserialize_unimplemented!(
        Deserializer<R>,
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    deserialize_unimplemented!(
        Deserializer<R>,
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
}
