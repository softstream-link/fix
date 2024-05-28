use crate::{
    framing::{compute_check_sum, BodyLength, CheckSum, Header1EnvelopeSequence, Header2TypeCompIdSequence, TrailerCheckSum},
    prelude::{BytesWrite, Result, Serializer},
};
use bytes::BytesMut;
use fix_model_core::prelude::Schema;
use serde::Serialize;

pub struct SendFrame<S, X> {
    serializer_header: Serializer<BytesWrite, X>,
    serializer_body: Serializer<BytesWrite, X>,
    header1: Header1EnvelopeSequence<S>,
    header2: Header2TypeCompIdSequence<S>,
}
impl<S: Serialize + AsRef<str>, X: Schema> SendFrame<S, X> {
    pub fn with_capacity(capacity: usize, header1: Header1EnvelopeSequence<S>, header2: Header2TypeCompIdSequence<S>, _schema: X) -> SendFrame<S, X> {
        #[allow(clippy::identity_op)]
        let header_capacity = 0// total len
        + header1.size() // Header1EnvelopeSequence
        + header2.size() // Header2TypeCompIdSequence
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
            header2,
        }
    }
    pub fn serialize<T: serde::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut self.serializer_body)
    }
    pub fn complete(mut self, calc_check_sum: bool) -> Result<Serializer<BytesWrite, X>> {
        self.header1.body_length = BodyLength(self.serializer_body.len().into());
        self.header1.serialize(&mut self.serializer_header)?;
        self.header2.serialize(&mut self.serializer_header)?;

        let mut joined = self.serializer_header;
        joined.join(self.serializer_body);
        let tag_value_check_sum = TrailerCheckSum {
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
