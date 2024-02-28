use crate::{
    prelude::Result,
    ser::{serializer::Serializer, write::BytesWrite},
};
use fix_model_core::{
    prelude::{Schema, TagTypesSorted},
    schema::BinaryDataLenPair,
};

pub struct UnitTestSchema;

impl Schema for UnitTestSchema {
    fn index() -> TagTypesSorted {
        static INDEX_PRE_SORTED_BY_TAG_LEN: TagTypesSorted = &[BinaryDataLenPair {
            tag_len: b"95",
            tag_data: b"96",
        }];
        INDEX_PRE_SORTED_BY_TAG_LEN
    }
}

#[cfg(test)]
mod tests {
    use fix_model_core::{prelude::Tag, schema::BinaryDataLenPair};

    use super::*;
    use fix_model_core::types::display::FixByteSlice2Display;
    use fix_model_test::unittest::setup;
    use log::info;

    #[test]
    fn test_schema() {
        setup::log::configure();

        info!("DefaultSchema::to_string():\n{}", UnitTestSchema::to_string());

        let tag: Tag = b"95";
        info!("tag: {}", tag.to_string());
        let found = UnitTestSchema::lookup(tag).unwrap();
        info!("found: {}", found);
        assert_eq!(
            found,
            BinaryDataLenPair {
                tag_len: b"95",
                tag_data: b"96"
            }
        );

        let tag: Tag = b"999";
        info!("tag: {}", tag.to_string());
        let found = UnitTestSchema::lookup(tag);
        info!("found: {:?}", found);
        assert_eq!(found, None);
    }
}

pub fn to_bytes_unittest<T: serde::ser::Serialize>(value: &T) -> Result<Serializer<BytesWrite, UnitTestSchema>> {
    crate::to_bytes_with_schema(value, None, UnitTestSchema)
}
pub fn from_slice_unittest<'de, T: serde::de::Deserialize<'de>>(slice: &'de [u8]) -> Result<T> {
    crate::from_slice_with_schema(slice, UnitTestSchema)
}
