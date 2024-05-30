use crate::{
    prelude::Result,
    ser::{serializer::Serializer, write::BytesWrite},
};
use fix_model_core::{
    prelude::{Schema, TagTypesSorted},
    schema::BinaryDataLenPair,
    types::fixmsgtype::{Header, MsgTypeCode},
};

use serde::{de::Error, Deserialize, Serialize};

pub struct UnitTestSchema;

impl Schema for UnitTestSchema {
    type Header<'de, S: Deserialize<'de> + Serialize, C: Deserialize<'de> + Serialize, D: Deserialize<'de> + Serialize> = Header3<S>;
    type AdmType<S, C, D> = AdminMsg<S>;
    type AppType<S, C, D> = ();

    fn binary_data_len_pair_index() -> TagTypesSorted {
        static INDEX_PRE_SORTED_BY_TAG_LEN: TagTypesSorted = &[BinaryDataLenPair {
            tag_len: b"95",
            tag_data: b"96",
        }];
        INDEX_PRE_SORTED_BY_TAG_LEN
    }
    fn deserializer_msg<'de, __D, S, C, D>(
        msg_type: &str,
        deserializer: __D,
    ) -> std::result::Result<(Option<Self::AdmType<S, C, D>>, Option<Self::AppType<S, C, D>>), __D::Error>
    where
        __D: serde::Deserializer<'de>,
        S: serde::Deserialize<'de>,
        C: serde::Deserialize<'de>,
        D: serde::Deserialize<'de>,
    {
        match msg_type {
            "A" => Ok((Some(AdminMsg::<S>::Logon(Logon::deserialize(deserializer)?)), None)),
            _ => Err(Error::custom(format!("unknown msg_type: {}", msg_type))),
        }
    }
    // fn deserializer_header<'de, __D, S, C, D>(deserializer: __D) -> std::result::Result<Self::Header<'de, S, C, D>, __D::Error>
    // where
    //     __D: serde::Deserializer<'de>,
    //     S: serde::Deserialize<'de>,
    //     C: serde::Deserialize<'de>,
    //     D: serde::Deserialize<'de>,
    // {
    //     Self::Header::<'de, S, C, D>::deserialize(deserializer)
    // }
}

fix_model_generator::prelude::fix_usize!(EncryptMethod, 98);
fix_model_generator::prelude::fix_usize!(HeartBtInt, 108);
fix_model_generator::prelude::fix_string!(Username, 553);
// <field name='EncryptMethod' required='Y' />
// <field name='HeartBtInt' required='Y' />
// <field name='RawDataLength' required='N' />
// <field name='RawData' required='N' />
// <field name='ResetSeqNumFlag' required='N' />
// <field name='NextExpectedMsgSeqNum' required='N' />
// <field name='MaxMessageSize' required='N' />
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Logon<S> {
    // fields
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    // ....
    #[serde(rename = "553")]
    pub username: Option<Username<S>>,
    // more fields
    #[serde(rename = "108")]
    pub heart_bt_int: HeartBtInt,
}
impl<S> MsgTypeCode for Logon<S> {
    const MSG_TYPE_CODE: &'static str = "A";
    fn is_app(&self) -> bool {
        true
    }
    fn is_adm(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum AdminMsg<S> {
    Logon(Logon<S>),
}

fix_model_generator::prelude::fix_string!(OnBehalfOfCompID, 115);

// <field name='OnBehalfOfCompID' required='N' />
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Header3<S> {
    #[serde(rename = "115")]
    pub on_behalf_of_comp_id: OnBehalfOfCompID<S>,
}
impl<S> Header for Header3<S> {}

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
        let found = UnitTestSchema::binary_data_len_pair_index_lookup(tag).unwrap();
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
        let found = UnitTestSchema::binary_data_len_pair_index_lookup(tag);
        info!("found: {:?}", found);
        assert_eq!(found, None);
    }
}

pub fn to_bytes_unittest<T: serde::ser::Serialize>(value: &T) -> Result<Serializer<BytesWrite, UnitTestSchema>> {
    crate::to_bytes_with_schema::<T, UnitTestSchema>(value, None)
}
pub fn from_slice_unittest<'de, T: serde::de::Deserialize<'de>>(slice: &'de [u8]) -> Result<T> {
    crate::from_slice_with_schema::<T, UnitTestSchema>(slice)
}
