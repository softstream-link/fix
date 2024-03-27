use crate::prelude::{Error, Result};

use super::macros::deserialize_unimplemented;
use super::{deserializer::Deserializer, read::Read};
use serde::de::{self};

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_bounds.html
struct FixMapKey<'any, R: 'any> {
    de: &'any mut Deserializer<R>,
}
impl<'any, R: 'any> FixMapKey<'any, R> {
    #[inline(always)]
    fn new(de: &'any mut Deserializer<R>) -> Self {
        Self { de }
    }
}

impl<'de, 'any, R: Read<'de> + 'any> de::Deserializer<'de> for FixMapKey<'any, R> {
    type Error = Error;
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let tag = self.de.read.parse_fix_tag()?;
        // visitor here is typically a #[derive(Deserialize)] __Visitor that
        // maps either "" or b"" to a enum __Field { __field0, __field1, ... }
        let field = visitor.visit_bytes(tag);
        field
    }

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let tag = self.de.read.parse_unsigned()?;
        visitor.visit_u64(tag)
    }

    deserialize_unimplemented!(
        FixMapKey,
        deserialize_bool(self, _visitor: V),
        deserialize_i8(self, _visitor: V),
        deserialize_i16(self, _visitor: V),
        deserialize_i32(self, _visitor: V),
        deserialize_i64(self, _visitor: V),
        deserialize_u8(self, _visitor: V),
        deserialize_u16(self, _visitor: V),
        deserialize_u32(self, _visitor: V),
        deserialize_u64(self, _visitor: V),
        deserialize_f32(self, _visitor: V),
        deserialize_f64(self, _visitor: V),
        deserialize_char(self, _visitor: V),
        deserialize_str(self, _visitor: V),
        deserialize_string(self, _visitor: V),
        deserialize_bytes(self, _visitor: V),
        deserialize_byte_buf(self, _visitor: V),
        deserialize_option(self, _visitor: V),
        deserialize_unit(self, _visitor: V),
        deserialize_seq(self, _visitor: V),
        deserialize_map(self, _visitor: V),
        deserialize_ignored_any(self, _visitor: V),
    );
    deserialize_unimplemented!(
        FixMapKey,
        deserialize_unit_struct(self, _name: &'static str, _visitor: V),
        deserialize_newtype_struct(self, _name: &'static str, _visitor: V)
    );
    deserialize_unimplemented!(
        FixMapKey,
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    deserialize_unimplemented!(
        FixMapKey,
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    deserialize_unimplemented!(
        FixMapKey,
        deserialize_struct(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V),
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
}
pub(super) struct FixMapAccess<'any, R: 'any> {
    de: &'any mut Deserializer<R>,
}
impl<'any, R: 'any> FixMapAccess<'any, R> {
    #[inline(always)]
    pub fn new(de: &'any mut Deserializer<R>) -> Self {
        FixMapAccess { de }
    }
}

impl<'de, 'any, R: Read<'de> + 'any> de::MapAccess<'de> for FixMapAccess<'any, R> {
    type Error = Error;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match self.de.read.peek()? {
            // not EndOfFile
            Some(_) => {
                let res = seed.deserialize(FixMapKey::new(self.de));
                res.map(Some)
            }
            // EndOfFiles
            _ => Ok(None),
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        let res = seed.deserialize(&mut *self.de);
        res
    }
}
