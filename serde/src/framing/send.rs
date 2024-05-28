use std::ops::{Deref, DerefMut};

use bytes::BytesMut;
use fix_model_core::prelude::Schema;
use serde::Serialize;

use crate::{
    framing::check_sum,
    prelude::{BytesWrite, Result, Serializer},
};

pub struct SendFrame<S, X> {
    serializer_header: Serializer<BytesWrite, X>,
    serializer_body: Serializer<BytesWrite, X>,
    begin_string: S,
    msg_type: S,
    sender_comp_id: S,
    target_comp_id: S,
}
impl<S, X> Deref for SendFrame<S, X> {
    type Target = Serializer<BytesWrite, X>;
    fn deref(&self) -> &Self::Target {
        &self.serializer_body
    }
}
impl<S, X> DerefMut for SendFrame<S, X> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.serializer_body
    }
}
impl<S: Serialize + AsRef<str>, X: Schema> SendFrame<S, X> {
    pub fn with_capacity(capacity: usize, begin_string: S, msg_type: S, sender_comp_id: S, target_comp_id: S, _schema: X) -> SendFrame<S, X> {
        let header_capacity = 0// total len
        + b"8=".len() + begin_string.as_ref().as_bytes().len() + b"".len() // BeginString
        + b"9=".len() + 20 + b"".len() // fix_usize_fixed_length!(BodyLength, 9); generates 20 zero padded string 
        + b"35=".len() + msg_type.as_ref().as_bytes().len() + b"".len() // MsgType
        + b"49=".len() + sender_comp_id.as_ref().as_bytes().len() + b"".len() // SenderCompID
        + b"56=".len() + target_comp_id.as_ref().as_bytes().len() + b"".len() // TargetCompID
        ;
        debug_assert!(header_capacity < capacity, "header_capacity must be less than total_capacity");
        let mut header = BytesMut::with_capacity(capacity);
        let body = header.split_off(header_capacity);

        let write_header = BytesWrite::new(header);
        let write_body = BytesWrite::new(body);
        let serializer_header = Serializer::new(write_header);
        let serializer_body = Serializer::new(write_body);
        Self {
            serializer_header,
            serializer_body,
            begin_string,
            msg_type,
            sender_comp_id,
            target_comp_id,
        }
    }
    pub fn envelope(mut self, compute_check_sum: bool) -> Result<Serializer<BytesWrite, X>> {
        fix_model_generator::fix_string!(BeginString, 8);
        fix_model_generator::fix_usize_fixed_length!(BodyLength, 9);
        fix_model_generator::fix_u8_fixed_length!(CheckSum, 10);
        fix_model_generator::fix_string!(MsgType, 35);
        fix_model_generator::fix_string!(SenderCompID, 49);
        fix_model_generator::fix_string!(TargetCompID, 56);

        #[derive(serde::Serialize)]
        struct TagValueHead<S> {
            #[serde(rename = "8")]
            begin_string: BeginString<S>,
            #[serde(rename = "9")]
            body_length: BodyLength,
            #[serde(rename = "35")]
            msg_type: MsgType<S>,
            #[serde(rename = "49")]
            sender_comp_id: SenderCompID<S>,
            #[serde(rename = "56")]
            target_comp_id: TargetCompID<S>,
        }
        #[derive(serde::Serialize)]
        struct TagValueTail {
            #[serde(rename = "10")]
            check_sum: CheckSum,
        }
        let tag_value_head = TagValueHead {
            begin_string: BeginString(self.begin_string),
            body_length: BodyLength(self.serializer_body.len().into()),
            msg_type: MsgType(self.msg_type),
            sender_comp_id: SenderCompID(self.sender_comp_id),
            target_comp_id: TargetCompID(self.target_comp_id),
        };
        tag_value_head.serialize(&mut self.serializer_header)?;

        let mut joined = self.serializer_header;
        joined.join(self.serializer_body);
        let tag_value_check_sum = TagValueTail {
            check_sum: if compute_check_sum {
                check_sum(&joined).into()
            } else {
                CheckSum::default()
            },
        };
        tag_value_check_sum.serialize(&mut joined)?;
        Ok(joined)
    }
}
