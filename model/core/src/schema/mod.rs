use crate::types::display::FixByteSlice2Display;
use std::fmt::Display;
pub type Tag = &'static [u8];
// pub type RepatingGroupTagsSpecOrdered = &'static [Tag];
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
    fn lookup(tag: &[u8]) -> Option<BinaryDataLenPair> {
        let index = Self::index();
        match index.binary_search_by_key(&tag, |t| t.tag_len) {
            Ok(idx) => Some(index[idx]),
            Err(_) => None,
        }
    }
    fn to_string() -> String {
        let index = Self::index();
        index.iter().map(|t| t.to_string()).collect::<Vec<String>>().join("\n")
    }
    fn index() -> TagTypesSorted;
}
pub struct NoBinaryDataSchema;
impl Schema for NoBinaryDataSchema {
    fn index() -> TagTypesSorted {
        &[]
    }
}
