use crate::{
    de::{
        enums_deserializer::EnumAccess,
        macros::{impl_deserialize_float, impl_deserialize_signed, impl_deserialize_unimplemented, impl_deserialize_unsigned},
        read::{Read, SliceRead},
        rep_grp_deserializer::RepeatingGroupSeqAccess,
    },
    macros::asserted_short_name,
    prelude::{Error, Result},
};
use fix_model_core::{
    prelude::{FixByteSlice2Display, Schema},
    schema::BinaryDataLenPair,
};
use serde::de::{self};
use std::fmt::Display;

#[cfg(debug_assertions)]
use std::any::type_name;
#[cfg(debug_assertions)]
const NAME_GREEDY_MAPACESS: &str = "GreedyMapAccess";

// https://doc.rust-lang.org/rust-byexample/scope/lifetime/lifetime_bounds.html
/// Will continue to yeild keys untill it encounters an EndOfFile.
/// Typically reserved for deserializing the main body fo the FIX message since the fields are in the arbitrary order.
struct GreedyMapAccess<'a, R: 'a, X> {
    deserializer: &'a mut Deserializer<R, X>,
}
impl<'a, R: 'a, X> GreedyMapAccess<'a, R, X> {
    #[inline]
    pub fn new(deserializer: &'a mut Deserializer<R, X>) -> Self {
        GreedyMapAccess { deserializer }
    }
}
impl<'de, 'any, R: Read<'de> + 'any, X: Schema> de::MapAccess<'de> for GreedyMapAccess<'any, R, X> {
    type Error = Error;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        #[cfg(debug_assertions)]
        assert_eq!(
            NAME_GREEDY_MAPACESS,
            std::any::type_name::<Self>().split('<').next().unwrap().split("::").last().unwrap(),
            "Forgot to rename NAME_TAG_VALUE_MAPACESS after refactoring"
        );

        match self.deserializer.read.peek_tag()? {
            // not EndOfFile
            Some(_peeked_tag) => {
                #[cfg(debug_assertions)]
                log::trace!(
                    "{:<50} peeked_tag: {:?} ",
                    format!("{}::next_key_seed", NAME_GREEDY_MAPACESS),
                    _peeked_tag.to_string()
                );

                seed.deserialize(&mut *self.deserializer).map(Some)
            }
            // EndOfFiles
            _ => Ok(None),
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        let _parsed_tag = self.deserializer.read.parse_tag()?; // note that next_key_seed::FixTagIdentifier will only seek_tag
        #[cfg(debug_assertions)]
        log::trace!(
            "{:<50} parsed_tag: {:?} self.read: {}",
            format!("{}::next_value_seed", NAME_GREEDY_MAPACESS),
            _parsed_tag.to_string(),
            self.deserializer.read
        );

        let res = seed.deserialize(&mut *self.deserializer);
        res
    }
}

#[cfg(debug_assertions)]
const NAME_LAZY_MAPACCESS: &str = "LazyMapAccess";
/// Map access is initialized with a list of fields/fix tags that are expected to be deserialized.
/// `LazyMapAccess::next_key_seed` will stop yielding keys as soon at it encounters a tag that is not in the list of expected tags.
/// Typically reserved for deserializing parts fo the FIX Header because fields are fixed and known.
pub struct LazyMapAccess<'a, R, S> {
    deserializer: &'a mut Deserializer<R, S>,
    _name: &'static str,
    fields: &'static [&'static str],
}
impl<'a, R, S> LazyMapAccess<'a, R, S> {
    #[inline]
    pub fn new(deserializer: &'a mut Deserializer<R, S>, name: &'static str, fields: &'static [&'static str]) -> Self {
        #[cfg(debug_assertions)]
        assert_eq!(
            NAME_LAZY_MAPACCESS,
            type_name::<Self>().split('<').next().unwrap().split("::").last().unwrap(),
            "Forgot to rename NAME_MAPACESS after refactoring"
        );

        LazyMapAccess { deserializer, _name: name, fields }
    }
}
impl<'de, 'a, R: Read<'de> + 'a, S: Schema> de::MapAccess<'de> for LazyMapAccess<'a, R, S> {
    type Error = Error;
    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        #[cfg(debug_assertions)]
        {
            assert_eq!(
                NAME_LAZY_MAPACCESS,
                type_name::<Self>().split('<').next().unwrap().split("::").last().unwrap(),
                "Forgot to rename NAME_MAPACESS after refactoring"
            );
        }
        // peek_tag and determine if it should be deserialized_ident via seed so that it can be mapped to field
        match self.deserializer.read.peek_tag()? {
            // not EndOfFile
            Some(peeked_tag) => {
                let peeked_tag_idx = self.fields.iter().find(|tag| tag.as_bytes() == peeked_tag); // TODO does this need to do a binary search?

                #[cfg(debug_assertions)]
                log::trace!(
                    "{:<50} peeked_tag: '{}', peeked_tag_idx: {:?}, {}@['{}'] read: {}",
                    format!("{}::next_key_seed", NAME_LAZY_MAPACCESS),
                    peeked_tag.to_string(),
                    peeked_tag_idx,
                    self._name,
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
                "{:<50} parsed_tag: {:?} ",
                format!("{}::next_value_seed", NAME_LAZY_MAPACCESS),
                _parsed_tag.to_string()
            );
        }

        let res = seed.deserialize(&mut *self.deserializer);
        res
    }
}

const NAME_DESERIALIZER: &str = "Deserializer";
pub struct Deserializer<R, X> {
    pub(crate) read: R,
    phantom: std::marker::PhantomData<X>,
}
impl<'de, R: Read<'de>, X> Display for Deserializer<R, X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.read.fmt(f)
    }
}
impl<'de, R: Read<'de>, X> std::fmt::Debug for Deserializer<R, X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.read.fmt(f)
    }
}
impl<'de, R: Read<'de>, X> Deserializer<R, X> {
    pub fn new(read: R) -> Self {
        Deserializer {
            read,
            phantom: std::marker::PhantomData,
        }
    }
    #[inline]
    fn borrow_str(&mut self) -> Result<&'de str> {
        let slice = self.read.parse_value()?;
        std::str::from_utf8(slice).map_err(|_| Error::Message(format!("Invalid Utf8 {:?}", String::from_utf8_lossy(slice))))
    }
}
impl<'de, X> Deserializer<SliceRead<'de>, X> {}
impl<'de, R: Read<'de>, X> Deserializer<R, X> {
    /// The [Self::end] should be called after a value is fully deserialized to check if there are any trailing bytes.
    pub fn end(&mut self) -> Result<()> {
        if self.read.is_end()? {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }
}

impl<'de, 'a, R: Read<'de>, X: Schema> de::Deserializer<'de> for &'a mut Deserializer<R, X> {
    type Error = Error;

    impl_deserialize_unsigned!(deserialize_u64, deserialize_u32, deserialize_u16, deserialize_u8);
    impl_deserialize_signed!(deserialize_i64, deserialize_i32, deserialize_i16, deserialize_i8);
    impl_deserialize_float!(deserialize_f64, deserialize_f32);
    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let string = self.borrow_str()?;
        visitor.visit_string(string.to_owned())
    }
    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let string = self.borrow_str()?;
        visitor.visit_borrowed_str(string)
    }
    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let slice = self.read.parse_value()?;
        visitor.visit_byte_buf(slice.to_vec())
    }
    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let slice = self.read.parse_value()?;
        visitor.visit_borrowed_bytes(slice)
    }
    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // TODO NOTE that this relies on valid utf8 str to visit char
        self.deserialize_str(visitor)
    }
    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let slice = self.read.parse_value()?;
        match slice {
            b"Y" => visitor.visit_bool(true),
            b"N" => visitor.visit_bool(false),
            _ => Err(Error::Message(format!("Invalid bool value {:?}", String::from_utf8_lossy(slice)))),
        }
    }
    fn deserialize_newtype_struct<V: de::Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value> {
        #[cfg(debug_assertions)]
        log::trace!("{:<50} {}", format!("{}::deserialize_newtype_struct", NAME_DESERIALIZER), _name);

        visitor.visit_newtype_struct(self)
    }
    fn deserialize_struct<V: de::Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        #[cfg(debug_assertions)]
        log::trace!("{:<50} {}", format!("{}::deserialize_struct", NAME_DESERIALIZER), name);
        if name.starts_with("Header") || name.starts_with("Tagged") {
            visitor.visit_map(LazyMapAccess::new(self, name, fields))
        } else {
            visitor.visit_map(GreedyMapAccess::new(self))
        }
    }
    fn deserialize_enum<V: de::Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value> {
        visitor.visit_enum(EnumAccess::new(self))
    }
    fn deserialize_identifier<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // TODO  this is unwrap of Option not error,
        // it is sound to unwrap because check for Some is done during Map next_keey_seed but code will be more readable
        // if we return error here if though it is not possible
        let peeked_tag = self.read.peek_tag()?.unwrap();
        // #[cfg(debug_assertions)]
        // log::trace!(
        //     "{:<50} peeked_tag: {:?}",
        //     format!("{}::deserialize_identifier", NAME_DESERIALIZER),
        //     peeked_tag.to_string()
        // );
        visitor.visit_bytes(peeked_tag)
    }
    /// * Will forward to `Self::deserialize_seq` if the peeked tag is a repeating group.
    /// * Will forward to `Self::deserialize_bytes` if the peeked tag is a Len/Data binary section.
    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        #[cfg(debug_assertions)]
        {
            let last_peeked_tag = self.read.last_peeked_tag().map_or("None".to_owned(), |v| v.to_string());
            log::trace!(
                "{:<50} last_peeked_tag: {},  self.read: {}",
                format!("{}::deserialize_seq", NAME_DESERIALIZER),
                last_peeked_tag,
                self.read
            );
        }

        match self.read.last_peeked_tag() {
            Some(tag) => match X::binary_data_len_pair_index_lookup(tag) {
                // if foudn we are looking at binary pair otherwise it is a repeating group
                // Some(TagType::BinaryData { tag_data, .. }) => {
                Some(BinaryDataLenPair { tag_data, .. }) => {
                    let data_len = self.read.parse_value_as_number::<usize>()?;
                    match (self.read.parse_tag()?, tag_data) {
                        (Some(actual_tag), expected_tag) if actual_tag == expected_tag => {
                            let data = self.read.parse_value_with_length(data_len)?;
                            visitor.visit_borrowed_bytes(data) // TODO is this correct/ or should call visit_bytes?
                        }
                        (actual_tag, expected_tag) => Err(Error::Message(format!(
                            "{}::deserialize_seq: Expected to parse tag: '{}', instead found tag: '{}' ",
                            NAME_DESERIALIZER,
                            expected_tag.to_string(),
                            actual_tag.map_or("None".to_owned(), |v| v.to_string())
                        ))),
                    }
                }
                // this is a repeating group, first parse its length and then use SeqAccess to deserialize
                _ => {
                    let len = self.read.parse_value_as_number::<usize>()?;
                    visitor.visit_seq(RepeatingGroupSeqAccess::new(self, len))
                }
            },
            None => {
                // Special case for Data/Length pair where Data is not nested in another struct instead of serializing as
                // "95=4|data=1234|" it will be serialized as "4|1234|" // NOTE this is not a valid fix format but it helps with testing
                let data_len = self.read.parse_value_as_number::<usize>()?;
                let data = self.read.parse_value_with_length(data_len)?;
                visitor.visit_borrowed_bytes(data)
            }
        }
    }
    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let value = self.read.parse_value()?;
        #[cfg(debug_assertions)]
        log::warn!(
            "{:<50} parsed_val: {:?}, visitor: {}",
            format!("{}::deserialize_ignored_any", NAME_DESERIALIZER),
            value.to_string(),
            std::any::type_name::<V>()
        );
        visitor.visit_bytes(value)
    }
    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // fix always has a value for optional fields, or entire key-value pair is missing
        visitor.visit_some(self)
    }

    impl_deserialize_unimplemented!(
        asserted_short_name!("Deserializer", Self),
        deserialize_any(self, _visitor: V),
        deserialize_map(self, _visitor: V),
        deserialize_unit(self, _visitor: V),
    );
    impl_deserialize_unimplemented!(
        asserted_short_name!("Deserializer", Self),
        deserialize_unit_struct(self, _name: &'static str, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        asserted_short_name!("Deserializer", Self),
        deserialize_tuple(self, _len: usize, _visitor: V)
    );
    impl_deserialize_unimplemented!(
        asserted_short_name!("Deserializer", Self),
        deserialize_tuple_struct(self, _name: &'static str, _len: usize, _visitor: V)
    );
    fn is_human_readable(&self) -> bool {
        false
    }
}
