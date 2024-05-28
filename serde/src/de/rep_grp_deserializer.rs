use super::{deserializer::Deserializer, macros::impl_deserialize_unimplemented, read::Read};
use crate::prelude::{Error, Result};
use fix_model_core::prelude::{FixByteSlice2Display, Schema};
use serde::de::{self};
use std::{any::type_name, usize};

const NAME_REP_GROUP_MAPACCESS: &str = "RepeatingGroupMapAccess";
struct RepeatingGroupMapAccess<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
    name: &'static str,
    fields: &'static [&'static str],
    idx_of_last_processed_tag: Option<usize>,
}
impl<'a, R, S> RepeatingGroupMapAccess<'a, R, S> {
    #[inline]
    pub fn new(deserializer: &'a mut Deserializer<R, S>, name: &'static str, fields: &'static [&'static str]) -> Self {
        #[cfg(debug_assertions)]
        assert_eq!(
            NAME_REP_GROUP_MAPACCESS,
            type_name::<Self>().split("<").next().unwrap().split("::").last().unwrap(),
            "Forgot to rename NAME_MAPACESS after refactoring"
        );

        RepeatingGroupMapAccess {
            deserializer,
            name,
            fields,
            idx_of_last_processed_tag: None,
        }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::MapAccess<'de> for RepeatingGroupMapAccess<'a, R, S> {
    type Error = Error;
    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        #[cfg(debug_assertions)]
        {
            assert_eq!(
                NAME_REP_GROUP_MAPACCESS,
                type_name::<Self>().split('<').next().unwrap().split("::").last().unwrap(),
                "Forgot to rename NAME_MAPACESS after refactoring"
            );
        }
        // peek_tag and determine if it should be deserialized_ident via seed so that it can be mapped to field
        match self.deserializer.read.peek_tag()? {
            // not EndOfFile
            Some(peeked_tag) => {
                let peeked_tag_idx = self
                    .fields
                    .iter()
                    .skip(self.idx_of_last_processed_tag.unwrap_or(0))
                    .enumerate()
                    .find_map(|(idx, tag)| {
                        if tag.as_bytes() == peeked_tag {
                            Some(idx + self.idx_of_last_processed_tag.unwrap_or(0))
                        } else {
                            None
                        }
                    });

                #[cfg(debug_assertions)]
                log::trace!(
                    "{:<50} peeked_tag: '{}', peeked_tag_idx: {:?}, idx_of_last_processed_tag: {:?}, {}@['{}'] read: {}",
                    format!("{}::next_key_seed", NAME_REP_GROUP_MAPACCESS),
                    peeked_tag.to_string(),
                    peeked_tag_idx,
                    self.idx_of_last_processed_tag,
                    self.name,
                    self.fields.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("', '"),
                    self.deserializer.read
                );

                match (peeked_tag_idx, self.idx_of_last_processed_tag) {
                    // REPEATING GROUP START
                    (Some(found_idx), None) => {
                        self.idx_of_last_processed_tag = Some(found_idx);
                        seed.deserialize(&mut *self.deserializer).map(Some)
                    }
                    // REPEATING GROUP CONTINUE  when this is first occurrence of the tag in the repeating group continue processing if found_idx > idx_of_last_processed_tag
                    (Some(found_idx), Some(idx_of_last_processed_tag)) if found_idx > idx_of_last_processed_tag => {
                        self.idx_of_last_processed_tag = Some(found_idx);
                        seed.deserialize(&mut *self.deserializer).map(Some)
                    }
                    // REPEATING GROUP END when tag repeats or was previously processed it is a new repeating group  if found_idx <= idx_of_last_processed_tag
                    (Some(_), Some(_)) => {
                        self.idx_of_last_processed_tag = None;
                        Ok(None) // REPEATING GROUP END
                    }
                    // REPEATING GROUP END
                    (None, _) => {
                        self.idx_of_last_processed_tag = None; // if tag not found in self.tags meaning not in own set
                        Ok(None)
                    }
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
                format!("{}::next_value_seed", NAME_REP_GROUP_MAPACCESS),
                _parsed_tag.to_string()
            );
        }

        let res = seed.deserialize(&mut *self.deserializer);
        res
    }
}

const NAME_REP_GROUP_DESERIALIZER: &str = "RepeatingGroupDeserializer";
pub(super) struct RepeatingGroupDeserializer<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
    // map_access: RepeatingGroupMapAccess<'a, R, S>,
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::Deserializer<'de> for &mut RepeatingGroupDeserializer<'a, R, S> {
    type Error = Error;
    // fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
    //     #[cfg(debug_assertions)]
    //     log::trace!("{:<50}", format!("{}::deserialize_map", NAME_REP_GROUP_DESERIALIZER));
    //     visitor.visit_map(&mut self.map_access)
    // }
    fn deserialize_struct<V: de::Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        #[cfg(debug_assertions)]
        log::trace!(
            "{:<50} {}@['{}']",
            format!("{}::deserialize_struct", NAME_REP_GROUP_DESERIALIZER),
            name,
            { fields.iter().map(|f| f.to_string()).collect::<Vec<String>>().join("', '") }
        );

        visitor.visit_map(&mut RepeatingGroupMapAccess::new(self.deserializer, name, fields))
    }
    impl_deserialize_unimplemented!(
        NAME_REP_GROUP_DESERIALIZER,
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
        NAME_REP_GROUP_DESERIALIZER,
        deserialize_unit_struct(self, _name: &'static str, _visitor: V),
        deserialize_newtype_struct(self, _name: &'static str, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_REP_GROUP_DESERIALIZER,
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_REP_GROUP_DESERIALIZER,
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        NAME_REP_GROUP_DESERIALIZER,
        deserialize_enum(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V)
    );
}

const NAME_REP_GROUP_SEQ_ACCESS: &str = "RepeatingGroupSeqAccess";
pub(super) struct RepeatingGroupSeqAccess<'a, R, S> {
    deserializer: RepeatingGroupDeserializer<'a, R, S>,
    rep_grp_len_expected: usize,
    rep_grp_len_processed: usize,
}
impl<'a, R, S: Schema> RepeatingGroupSeqAccess<'a, R, S> {
    #[inline(always)]
    pub fn new(deserializer: &'a mut Deserializer<R, S>, len: usize) -> Self {
        #[cfg(debug_assertions)]
        assert_eq!(
            NAME_REP_GROUP_SEQ_ACCESS,
            type_name::<Self>().split('<').next().unwrap().split("::").last().unwrap(),
            "Forgot to rename NAME_SEQ_ACCESS after refactoring"
        );

        RepeatingGroupSeqAccess {
            deserializer: RepeatingGroupDeserializer { deserializer },
            rep_grp_len_expected: len,
            rep_grp_len_processed: 0,
        }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::SeqAccess<'de> for RepeatingGroupSeqAccess<'a, R, S> {
    type Error = Error;
    fn next_element_seed<T: de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        #[cfg(debug_assertions)]
        log::trace!(
            "{:<50} rep_grp_len_expected: {}, rep_grp_len_processed: {}",
            format!("{}::next_element_seed", NAME_REP_GROUP_SEQ_ACCESS),
            self.rep_grp_len_expected,
            self.rep_grp_len_processed
        );

        if self.rep_grp_len_processed >= self.rep_grp_len_expected {
            return Ok(None);
        }

        let value = seed.deserialize(&mut self.deserializer)?;
        self.rep_grp_len_processed += 1;
        Ok(Some(value))
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.rep_grp_len_expected)
    }
}
