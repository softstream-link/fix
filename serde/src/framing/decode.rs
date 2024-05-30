use crate::framing::compute_check_sum;
use crate::prelude::{Deserializer, Error, Result, SliceRead};
use fix_model_core::prelude::Schema;
use fix_model_core::types::dat::dat;
use fix_model_core::types::display::FixByteSlice2Display;

use serde::Deserialize;

use super::{Header1EnvelopeSequence, Header2CompIdSequence, TaggedCheckSum, TaggedMsgType};

pub struct FrameDecoder<'de, X: Schema> {
    frame: &'de [u8],
    // is_some if split was called
    deserializer: Option<crate::Deserializer<SliceRead<'de>, X>>,
    // is_some if header1 was called
    header1: Option<Header1EnvelopeSequence<&'de str>>,
    // is some if header2 was called
    msg_type: Option<TaggedMsgType<&'de str>>,
    header2: Option<Header2CompIdSequence<&'de str>>,
    // is true if header3 was called
    header3: Option<X::Header<'de, &'de str, char, &'de dat>>,
}
impl<'de, X: Schema> FrameDecoder<'de, X> {
    const CHECK_SUM_LEN: usize = 7;

    pub fn new(frame: &'de [u8]) -> Self {
        Self {
            frame,
            deserializer: None,
            header1: None,
            msg_type: None,
            header2: None,
            header3: None,
        }
    }
    fn split(&mut self) -> Result<(&'de [u8], &'de [u8])> {
        if self.frame.len() < Self::CHECK_SUM_LEN {
            #[cfg(debug_assertions)]
            log::error!(
                "Invalid frame overall length too short even for check sum frame: {}",
                self.frame.to_string()
            );
            return Err(crate::prelude::Error::InvalidChecksum(0.into()));
        }
        let split = self.frame.len() - Self::CHECK_SUM_LEN;
        let (root_slice, check_sum_slice) = self.frame.split_at(split);
        self.deserializer = Some(Deserializer::<_, X>::new(SliceRead::new(root_slice)));
        Ok((root_slice, check_sum_slice))
    }
    pub fn validate_check_sum(&mut self) -> Result<u8> {
        let (root_slice, check_sum_slice) = self.split()?;
        let mut des = Deserializer::<_, X>::new(SliceRead::new(check_sum_slice));

        let tag_value_check_sum = TaggedCheckSum::deserialize(&mut des)?;

        if tag_value_check_sum.check_sum.value() != 0 {
            let expected_check_sum = compute_check_sum(root_slice);
            if tag_value_check_sum.check_sum.value() != expected_check_sum {
                #[cfg(debug_assertions)]
                log::warn!(
                    "expected_check_sum: {}, tag_value_check_sum.check_sum.value(): {}",
                    expected_check_sum,
                    tag_value_check_sum.check_sum.value()
                );
                return Err(Error::InvalidChecksum(root_slice.len().into()));
            } else {
                return Ok(expected_check_sum);
            }
        }
        Ok(0)
    }
    pub fn deserialize<T: serde::Deserialize<'de>>(&mut self) -> Result<T> {
        if self.deserializer.is_none() {
            // this will assign a serializer
            self.split()?;
        }
        let des = self.deserializer.as_mut().expect("split call should have assigned a deserializer");
        T::deserialize(des)
    }
    pub fn deserialize_header1(&mut self) -> Result<&Header1EnvelopeSequence<&str>> {
        match self.header1 {
            Some(ref header1) => Ok(header1),
            None => {
                let header1 = self.deserialize::<Header1EnvelopeSequence<&str>>()?;
                self.header1 = Some(header1);
                Ok(self.header1.as_ref().unwrap())
            }
        }
    }
    pub fn deserialize_header2(&mut self) -> Result<&Header2CompIdSequence<&str>> {
        let _header1 = self.deserialize_header1()?;

        match self.header2 {
            Some(ref header2) => Ok(header2),
            None => {
                self.msg_type = Some(self.deserialize::<TaggedMsgType<&str>>()?);
                let header2 = self.deserialize::<Header2CompIdSequence<&str>>()?;
                self.header2 = Some(header2);
                Ok(self.header2.as_ref().unwrap())
            }
        }
    }

    pub fn deserialize_header3(&mut self) -> Result<&X::Header<'de, &'de str, char, &'de dat>>
    where
        <X as Schema>::Header<'de, &'de str, char, &'de dat>: Deserialize<'de>,
    {
        let _header2 = self.deserialize_header2()?;
        match self.header3 {
            Some(ref header3) => Ok(header3),
            None => {
                // let header3 = X::Header::deserialize(self.deserializer.as_mut().expect("split should have assigned a deserializer"))?;
                // let header3 = X::Header::<'_, _, _, _>::deserialize(self.deserializer.as_mut().expect("split should have assigned a deserializer"))?;
                let header3 = self.deserialize::<X::Header<'_, _, _, _>>()?;
                self.header3 = Some(header3);
                Ok(self.header3.as_ref().unwrap())
            }
        }
    }
    pub fn deserialize_msg<S, C, D>(&mut self) -> Result<(Option<X::AdmType<S, C, D>>, Option<X::AppType<S, C, D>>)>
    where
        S: serde::Deserialize<'de>,
        C: serde::Deserialize<'de>,
        D: serde::Deserialize<'de>,
        <X as Schema>::Header<'de, &'de str, char, &'de dat>: Deserialize<'de>,

    {
        let _header3 = self.deserialize_header3()?;

        let msg_type = self
            .msg_type
            .as_ref()
            .expect("deserialize_header2 should have assigned a msg_type")
            .msg_type
            .value();

        X::deserializer_msg::<_, S, C, D>(msg_type, self.deserializer.as_mut().expect("split should have assigned a deserializer"))
    }

    pub fn complete(&mut self) -> Result<()> {
        if self.deserializer.is_none() {
            // this will assign a serializer
            self.split()?;
        }
        let des = self.deserializer.as_mut().expect("split should have assigned a deserializer");
        des.end()
    }
}

#[cfg(test)]
mod test {
    use fix_model_test::unittest::setup;
    use log::info;

    use crate::{prelude::IssueAtPosition, unittest::UnitTestSchema};

    use super::*;
    #[test]
    fn test_check_sum() {
        setup::log::configure_level(log::LevelFilter::Info);
        let slice_valid_check_sum = b"8=FIX.4.49=0000000000000000001235=A49=source56=dest98=0108=3010=240";
        let mut recv_frame = FrameDecoder::<UnitTestSchema>::new(slice_valid_check_sum);
        let check_sum = recv_frame.validate_check_sum().unwrap();
        assert_eq!(check_sum, 240);

        let slice_in_valid_check_sum = b"8=FIX.4.49=0000000000000000001235=A49=source56=dest98=0108=3010=241";
        let mut recv_frame = FrameDecoder::<UnitTestSchema>::new(slice_in_valid_check_sum);
        let err = recv_frame.validate_check_sum().unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, Error::InvalidChecksum(IssueAtPosition(68))));
    }
}
