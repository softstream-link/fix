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
    envelope: Header1EnvelopeSequence<S>,
}
impl<S: Serialize + AsRef<str>, X: Schema> FrameEnchoder<S, X> {
    pub fn with_capacity(capacity: usize, envelope: Header1EnvelopeSequence<S>) -> FrameEnchoder<S, X> {
        let mut header = BytesMut::with_capacity(capacity + envelope.size());
        let body = header.split_off(envelope.size());

        let serializer_header = Serializer::new(BytesWrite::new(header));
        let serializer_body = Serializer::new(BytesWrite::new(body));
        Self {
            serializer_header,
            serializer_body,
            envelope,
        }
    }
    pub fn reset(&mut self) {
        self.serializer_header.reset();
        self.serializer_body.reset();
    }
    pub fn serialize<'a, M>(&mut self, comp_ids: &Header2CompIdSequence<S>, header: &'a X::Header<'a, &'a str, char, &'a dat>, msg: &M) -> Result<()>
    where
        M: serde::Serialize + fix_model_core::prelude::MsgTypeCode,
    {
        TaggedMsgType::serialize(
            &TaggedMsgType {
                msg_type: msg.msg_type().into(),
            },
            &mut self.serializer_body,
        )?;
        comp_ids.serialize(&mut self.serializer_body)?;
        use serde::Serialize;
        header.serialize(&mut self.serializer_body)?;
        msg.serialize(&mut self.serializer_body)
    }
    pub fn envelope(mut self, calc_check_sum: bool) -> Result<Serializer<BytesWrite, X>> {
        self.envelope.body_length = BodyLength(self.serializer_body.len());
        self.envelope.serialize(&mut self.serializer_header)?;

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
