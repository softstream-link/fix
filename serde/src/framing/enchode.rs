use super::{Header2CompIdSequence, TaggedMsgType};
use crate::{
    framing::{compute_check_sum, BodyLength, CheckSum, Header1EnvelopeSequence, TaggedCheckSum},
    prelude::{BytesWrite, Result, Serializer},
};
use bytes::BytesMut;
use fix_model_core::prelude::{dat, Schema};
use serde::Serialize;
use std::cmp::max;

pub struct FrameEnchoder<X> {
    head: Option<Serializer<BytesWrite, X>>,
    body: Option<Serializer<BytesWrite, X>>,
    envelope: Header1EnvelopeSequence<String>,
}
impl<X: Schema> FrameEnchoder<X> {
    pub fn with_capacity(capacity: usize, envelope: Header1EnvelopeSequence<String>) -> Self {
        let capacity = max(capacity, envelope.size());
        Self {
            head: Some(Serializer::new(BytesWrite::new(BytesMut::with_capacity(capacity)))),
            body: None, // empty
            envelope,
        }
    }
    #[inline]
    fn re_init_without_alloc(&mut self) {
        let mut header = self.head.take().expect("header is empty"); // take header buf

        // body could be left as none after with_capacity & if failed to write check sum because body is arleady joined to header at that point
        if let Some(body) = self.body.take() {
            header.join(body);
        }

        let mut head = header.take();
        head.clear();

        let body = head.split_off(self.envelope.size()); // split off body
        let head = Serializer::new(BytesWrite::new(head)); // init header
        let body = Serializer::new(BytesWrite::new(body)); // init body
        self.head = Some(head);
        self.body = Some(body);
    }
    pub fn serialize<'a, M>(
        &mut self,
        comp_ids: &Header2CompIdSequence<String>,
        header: &X::Header<'a, &'a str, char, &'a dat>,
        msg: &M,
        calc_check_sum: bool,
    ) -> Result<&Serializer<BytesWrite, X>>
    where
        M: serde::Serialize + fix_model_core::prelude::MsgTypeCode,
        <X as Schema>::Header<'a, &'a str, char, &'a dat>: Serialize,
    {
        self.re_init_without_alloc();
        let mut body = self.body.take().expect("body is empty");

        let msg_type = TaggedMsgType {
            msg_type: msg.msg_type().into(),
        };

        use serde::Serialize;
        if let Err(e) = msg_type.serialize(&mut body) {
            self.body = Some(body); // return serializer
            return Err(e);
        }
        if let Err(e) = comp_ids.serialize(&mut body) {
            self.body = Some(body); // return serializer
            return Err(e);
        }
        if let Err(e) = header.serialize(&mut body) {
            self.body = Some(body); // return serializer
            return Err(e);
        }
        if let Err(e) = msg.serialize(&mut body) {
            self.body = Some(body); // return serializer
            return Err(e);
        }

        let mut header = self.head.take().expect("header is empty");
        self.envelope.body_length = BodyLength(body.len());
        if let Err(e) = self.envelope.serialize(&mut header) {
            self.head = Some(header); // return serializer
            return Err(e);
        }
        // now header refers to both header & body
        header.join(body);

        let tag_value_check_sum = TaggedCheckSum {
            check_sum: if calc_check_sum {
                compute_check_sum(&header).into()
            } else {
                CheckSum::default()
            },
        };

        if let Err(e) = tag_value_check_sum.serialize(&mut header) {
            self.head = Some(header); // return serializer but nody is already None
            return Err(e);
        }

        self.head = Some(header);
        Ok(self.head.as_ref().expect("header is empty"))
    }

    // pub fn envelope(mut self, calc_check_sum: bool) -> Result<Serializer<BytesWrite, X>> {
    //     self.envelope.body_length = BodyLength(self.body.len());
    //     self.envelope.serialize(&mut self.head)?;

    //     let mut joined = self.head;
    //     joined.join(self.body);
    //     let tag_value_check_sum = TaggedCheckSum {
    //         check_sum: if calc_check_sum {
    //             compute_check_sum(&joined).into()
    //         } else {
    //             CheckSum::default()
    //         },
    //     };
    //     tag_value_check_sum.serialize(&mut joined)?;
    //     Ok(joined)
    // }
}
