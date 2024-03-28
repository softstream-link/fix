use super::read::Read;
use super::{deserializer::Deserializer, macros::deserialize_unimplemented};
use crate::error::{Error, Result};
use crate::macros::asserted_short_name;
use serde::de::{self};

struct FixEnumIdentifier<'any, R> {
    de: &'any mut Deserializer<R>,
}
impl<'any, R> FixEnumIdentifier<'any, R> {
    #[inline(always)]
    fn new(de: &'any mut Deserializer<R>) -> Self {
        Self { de }
    }
}
impl<'de, 'any, R: Read<'de> + 'any> de::Deserializer<'de> for FixEnumIdentifier<'any, R> {
    type Error = Error;
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let tag = self.de.read.parse_fix_value()?;
        let field = visitor.visit_bytes(tag);
        field
    }

    deserialize_unimplemented!(
        asserted_short_name!("FixTagIdentifier", Self),
        deserialize_any(self, _visitor: V),
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
        asserted_short_name!("FixEnumIdentifier", Self),
        deserialize_unit_struct(self, _name: &'static str, _visitor: V),
        deserialize_newtype_struct(self, _name: &'static str, _visitor: V)
    );
    deserialize_unimplemented!(
        asserted_short_name!("FixEnumIdentifier", Self),
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    deserialize_unimplemented!(
        asserted_short_name!("FixEnumIdentifier", Self),
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    deserialize_unimplemented!(
        asserted_short_name!("FixEnumIdentifier", Self),
        deserialize_struct(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V),
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
}

pub(crate) struct FixUnitVariantAccess<'any, R> {
    de: &'any mut Deserializer<R>,
}
impl<'any, R: 'any> FixUnitVariantAccess<'any, R> {
    #[inline(always)]
    pub fn new(de: &'any mut Deserializer<R>) -> Self {
        FixUnitVariantAccess { de }
    }
}

impl<'de, 'any, R: Read<'de> + 'any> de::EnumAccess<'de> for FixUnitVariantAccess<'any, R> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let variant = seed.deserialize(FixEnumIdentifier::new(&mut *self.de))?;
        Ok((variant, self))
    }
}
impl<'de, 'any, R: Read<'de> + 'any> de::VariantAccess<'de> for FixUnitVariantAccess<'any, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value> {
        // TODO add unimplimented like macro to include class name in the error
        Err(Error::Message("newtype_variant_seed is not supported".to_string()))
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value> {
        Err(Error::Message("tuple_variant is not supported".to_string()))
    }

    fn struct_variant<V: de::Visitor<'de>>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value> {
        Err(Error::Message("struct_variant is not supported".to_string()))
    }
}
