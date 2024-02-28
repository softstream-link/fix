use super::read::Read;
use super::{deserializer::Deserializer, macros::impl_deserialize_unimplemented};
use crate::error::{Error, Result};
use fix_model_core::prelude::Schema;
use serde::de::{self};

const NAME: &str = "EnumIdentifierDeserializer";
struct EnumIdentifierDeserializer<'a, R, S> {
    de: &'a mut Deserializer<R, S>,
}
impl<'a, 'de, R, S> EnumIdentifierDeserializer<'a, R, S> {
    #[inline(always)]
    fn new(de: &'a mut Deserializer<R, S>) -> Self {
        debug_assert_eq!(
            std::any::type_name::<Self>()
                .split('<')
                .next()
                .unwrap()
                .split("::")
                .last()
                .unwrap_or("Unknown"),
            NAME,
            "Forgot to rename struct after refactoring"
        );

        Self { de }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S> de::Deserializer<'de> for EnumIdentifierDeserializer<'a, R, S> {
    type Error = Error;
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let tag = self.de.read.parse_value()?;
        let field = visitor.visit_bytes(tag);
        field
    }

    impl_deserialize_unimplemented!(
        NAME,
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
    impl_deserialize_unimplemented!(
        NAME,
        deserialize_unit_struct(self, _name: &'static str, _visitor: V),
        deserialize_newtype_struct(self, _name: &'static str, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME,
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME,
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME,
        deserialize_struct(self, _name: &'static str, _fields: &'static [&'static str], _visitor: V),
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
}

pub(crate) struct EnumAccess<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
}
impl<'a, 'de, R: 'a, S> EnumAccess<'a, R, S> {
    #[inline(always)]
    pub fn new(deserializer: &'a mut Deserializer<R, S>) -> Self {
        EnumAccess { deserializer }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::EnumAccess<'de> for EnumAccess<'a, R, S> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V: de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let variant = seed.deserialize(EnumIdentifierDeserializer::new(&mut *self.deserializer))?;
        Ok((variant, self))
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S> de::VariantAccess<'de> for EnumAccess<'a, R, S> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(self, _seed: T) -> Result<T::Value> {
        Err(Error::NotSupported("EnumAccess::newtype_variant_seed"))
    }

    fn tuple_variant<V: de::Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value> {
        Err(Error::NotSupported("EnumAccess::tuple_variant"))
    }

    fn struct_variant<V: de::Visitor<'de>>(self, _fields: &'static [&'static str], _visitor: V) -> Result<V::Value> {
        Err(Error::NotSupported("EnumAccess::struct_variant"))
    }
}
