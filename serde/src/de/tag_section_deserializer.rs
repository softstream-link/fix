use super::{deserializer::Deserializer, macros::impl_deserialize_unimplemented, read::Read};
use crate::prelude::{Error, Result};
use fix_model_core::prelude::{FixByteSlice2Display, Schema};
use serde::de::{self};
use std::{any::type_name, usize};

const NAME_TAG_SECTION_MAPACCESS: &str = "TagSectionMapAccess";
pub struct TagSectionMapAccess<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
    name: &'static str,
    fields: &'static [&'static str],
}
impl<'a, R, S> TagSectionMapAccess<'a, R, S> {
    #[inline]
    pub fn new(deserializer: &'a mut Deserializer<R, S>, name: &'static str, fields: &'static [&'static str]) -> Self {
        #[cfg(debug_assertions)]
        assert_eq!(
            NAME_TAG_SECTION_MAPACCESS,
            type_name::<Self>().split("<").next().unwrap().split("::").last().unwrap(),
            "Forgot to rename NAME_MAPACESS after refactoring"
        );

        TagSectionMapAccess { deserializer, name, fields }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::MapAccess<'de> for TagSectionMapAccess<'a, R, S> {
    type Error = Error;
    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        #[cfg(debug_assertions)]
        {
            assert_eq!(
                NAME_TAG_SECTION_MAPACCESS,
                type_name::<Self>().split("<").next().unwrap().split("::").last().unwrap(),
                "Forgot to rename NAME_MAPACESS after refactoring"
            );
        }
        // peek_tag and determine if it should be deserialized_ident via seed so that it can be mapped to field
        match self.deserializer.read.peek_tag()? {
            // not EndOfFile
            Some(peeked_tag) => {
                let peeked_tag_idx = self.fields.iter().find(|tag| if tag.as_bytes() == peeked_tag { true } else { false });

                #[cfg(debug_assertions)]
                log::trace!(
                    "{:<50} peeked_tag: '{}', peeked_tag_idx: {:?}, {}@['{}'] read: {}",
                    format!("{}::next_key_seed", NAME_TAG_SECTION_MAPACCESS),
                    peeked_tag.to_string(),
                    peeked_tag_idx,
                    self.name,
                    self.fields.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("', '"),
                    self.deserializer.read
                );

                match peeked_tag_idx {
                    Some(_) => seed.deserialize(&mut *self.deserializer).map(Some),
                    None => Ok(None),
                }
            }
            // EndOfFiles
            _ => Ok(None), // REPEATING GROUP END
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        // always deserialize/parse_tag but remember to also parse_tag which until now was only peeked
        let _parsed_tag = self.deserializer.read.parse_tag()?;
        #[cfg(debug_assertions)]
        {
            log::trace!(
                "{:<50} parased_tag: {:?} ",
                format!("{}::next_value_seed", NAME_TAG_SECTION_MAPACCESS),
                _parsed_tag.to_string()
            );
        }

        let res = seed.deserialize(&mut *self.deserializer);
        res
    }
}

const NAME_TAG_SECTION_DESERIALIZER: &str = "TagSectionDeserializer";
pub(super) struct TagSectionDeserializer<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
    // map_access: RepeatingGroupMapAccess<'a, R, S>,
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::Deserializer<'de> for &mut TagSectionDeserializer<'a, R, S> {
    type Error = Error;
    fn deserialize_struct<V: de::Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        #[cfg(debug_assertions)]
        log::trace!(
            "{:<50} {}@['{}']",
            format!("{}::deserialize_struct", NAME_TAG_SECTION_DESERIALIZER),
            name,
            { fields.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("', '") }
        );

        visitor.visit_map(&mut TagSectionMapAccess::new(self.deserializer, name, fields))
    }
    impl_deserialize_unimplemented!(
        NAME_TAG_SECTION_DESERIALIZER,
        deserialize_map(self, _visitor: V),
        deserialize_identifier(self, _visitor: V),
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
        deserialize_ignored_any(self, _visitor: V),
    );

    impl_deserialize_unimplemented!(
        NAME_TAG_SECTION_DESERIALIZER,
        deserialize_unit_struct(self, _name: &'static str, _visitor: V),
        deserialize_newtype_struct(self, _name: &'static str, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_TAG_SECTION_DESERIALIZER,
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_TAG_SECTION_DESERIALIZER,
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_TAG_SECTION_DESERIALIZER,
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
}
