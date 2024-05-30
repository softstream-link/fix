use crate::types::display::FixByteSlice2Display;
use std::fmt::Display;
pub type Tag = &'static [u8];
pub type TagTypesSorted = &'static [BinaryDataLenPair];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryDataLenPair {
    pub tag_len: Tag,
    pub tag_data: Tag,
}

impl Display for BinaryDataLenPair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"BinaryDataLenPair {{ tag_len: "{}", data: "{}" }}"#,
            self.tag_len.to_string(),
            self.tag_data.to_string()
        )
    }
}

pub trait Schema {
    type Header<'a, S, C, D>;
    type AdmType<S, C, D>;
    type AppType<S, C, D>;
    fn binary_data_len_pair_index_lookup(tag: &[u8]) -> Option<BinaryDataLenPair> {
        let index = Self::binary_data_len_pair_index();
        match index.binary_search_by_key(&tag, |t| t.tag_len) {
            Ok(idx) => Some(index[idx]),
            Err(_) => None,
        }
    }
    fn to_string() -> String {
        let index = Self::binary_data_len_pair_index();
        index.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\n")
    }
    fn binary_data_len_pair_index() -> TagTypesSorted;

    fn deserializer_msg<'de, __D, S, C, D>(
        msg_type: &str,
        deserializer: __D,
    ) -> std::result::Result<(Option<Self::AdmType<S, C, D>>, Option<Self::AppType<S, C, D>>), __D::Error>
    where
        __D: serde::Deserializer<'de>,
        S: serde::Deserialize<'de>,
        C: serde::Deserialize<'de>,
        D: serde::Deserialize<'de>;
}
pub struct NoBinaryDataSchema;
impl Schema for NoBinaryDataSchema {
    type Header<'a, S, C, D> = ();
    type AdmType<S, C, D> = ();
    type AppType<S, C, D> = ();
    fn binary_data_len_pair_index() -> TagTypesSorted {
        &[]
    }
    fn deserializer_msg<'de, __D, S, C, D>(
        _msg_type: &str,
        _deserializer: __D,
    ) -> std::result::Result<(Option<Self::AdmType<S, C, D>>, Option<Self::AppType<S, C, D>>), __D::Error>
    where
        __D: serde::Deserializer<'de>,
        S: serde::Deserialize<'de>,
        C: serde::Deserialize<'de>,
        D: serde::Deserialize<'de>,
    {
        Ok((None, None))
    }
}
