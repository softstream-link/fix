use crate::{
    framing::{compute_check_sum, BodyLength, CheckSum, Header1EnvelopeSequence, TaggedCheckSum},
    prelude::{BytesWrite, Result, Serializer},
};
use bytes::BytesMut;
use fix_model_core::{prelude::Schema, types::dat::dat};
use serde::Serialize;

use super::{Header2CompIdSequence, TaggedMsgType};

pub struct FrameEnchoder<S, X> {
    serializer_header: Serializer<BytesWrite, X>,
    serializer_body: Serializer<BytesWrite, X>,
    header1: Header1EnvelopeSequence<S>,
}
impl<S: Serialize + AsRef<str>, X: Schema> FrameEnchoder<S, X> {
    pub fn with_capacity(capacity: usize, header1: Header1EnvelopeSequence<S>) -> FrameEnchoder<S, X> {
        #[allow(clippy::identity_op)]
        let header_capacity = 0// total len
        + header1.size() // Header1EnvelopeSequence
        ;
        debug_assert!(header_capacity < capacity, "header_capacity must be less than total_capacity");
        let mut header = BytesMut::with_capacity(capacity);
        let body = header.split_off(header_capacity);

        let serializer_header = Serializer::new(BytesWrite::new(header));
        let serializer_body = Serializer::new(BytesWrite::new(body));
        Self {
            serializer_header,
            serializer_body,
            header1,
            // header2,
        }
    }
    pub fn reset(&mut self) {
        self.serializer_header.reset();
        self.serializer_body.reset();
    }
    // pub fn serialize<H, M>(&mut self, header2: &Header2CompIdSequence<S>, header3: &H, msg: &M) -> Result<()>
    pub fn serialize<'a, M>(&mut self, header2: &Header2CompIdSequence<S>, header3: &'a X::Header<'a, &'a str, char, &'a dat>, msg: &M) -> Result<()>
    where
        M: serde::Serialize + fix_model_core::prelude::MsgTypeCode,
    {
        TaggedMsgType::serialize(
            &TaggedMsgType {
                msg_type: msg.msg_type().into(),
            },
            &mut self.serializer_body,
        )?;
        header2.serialize(&mut self.serializer_body)?;
        use serde::Serialize;
        header3.serialize(&mut self.serializer_body)?;
        msg.serialize(&mut self.serializer_body)
    }
    pub fn complete(mut self, calc_check_sum: bool) -> Result<Serializer<BytesWrite, X>> {
        self.header1.body_length = BodyLength(self.serializer_body.len());
        self.header1.serialize(&mut self.serializer_header)?;

        let mut joined = self.serializer_header;
        joined.join(self.serializer_body);
        let tag_value_check_sum = TaggedCheckSum {
            check_sum: if calc_check_sum {
                compute_check_sum(&joined).into()
            } else {
                CheckSum::default()
            },
        };
        tag_value_check_sum.serialize(&mut joined)?;
        Ok(joined)
    }
}
