use std::marker::PhantomData;

use fix_model_core::schema::Schema;
use fix_model_core::types::display::FixByteSlice2Display;
use serde::Deserialize;

use crate::prelude::{Error, Result};

use crate::de::read::SliceRead;
use crate::framing::check_sum;

pub struct RecvFrame<'de, X> {
    frame: &'de [u8],
    phantom: PhantomData<X>, // deserializer: crate::Deserializer<SliceRead<'de>, S>,
}
impl<'de, X: Schema> RecvFrame<'de, X> {
    const CHECK_SUM_LEN: usize = 7;

    pub fn new(frame: &'de [u8], _schema: X) -> Self {
        // let read = SliceRead::new(frame);
        // let deserializer = crate::Deserializer::new(read);
        // Self { frame, deserializer }
        Self { frame, phantom: PhantomData }
    }
    fn split(&self) -> Result<(&'de [u8], &'de [u8])> {
        if self.frame.len() < Self::CHECK_SUM_LEN {
            #[cfg(debug_assertions)]
            log::error!(
                "Invalid frame overall length too short even for check sum frame: {}",
                self.frame.to_string()
            );
            return Err(crate::prelude::Error::InvalidChecksum(0.into()));
        }
        let split = self.frame.len() - Self::CHECK_SUM_LEN;
        Ok(self.frame.split_at(split))
    }
    pub fn validate_check_sum(&self) -> Result<u8> {
        fix_model_generator::fix_u8_fixed_length!(CheckSum, 10);

        #[derive(serde::Deserialize)]
        struct TagValueCheckSum {
            #[serde(rename = "10")]
            check_sum: CheckSum,
        }

        let (head_slice, check_sum_slice) = self.split()?;
        let mut des = crate::Deserializer::<_, X>::new(SliceRead::new(check_sum_slice));

        let tag_value_check_sum = TagValueCheckSum::deserialize(&mut des)?;

        if tag_value_check_sum.check_sum.value() != 0 {
            let expected_check_sum = check_sum(head_slice);
            if tag_value_check_sum.check_sum.value() != expected_check_sum {
                #[cfg(debug_assertions)]
                log::warn!(
                    "expected_check_sum: {}, tag_value_check_sum.check_sum.value(): {}",
                    expected_check_sum,
                    tag_value_check_sum.check_sum.value()
                );
                return Err(Error::InvalidChecksum(head_slice.len().into())); 
            } else {
                return Ok(expected_check_sum);
            }
        }
        Ok(0)
    }
    // pub fn blah(&mut self) -> Result<()> {
    //     let (head_slice, check_sum_slice) = self.split()?;
    //     Ok(())
    // }
}

#[cfg(test)]
mod test {
    use fix_model_test::unittest::setup;
    use log::info;

    use crate::{prelude::IssueAtPosition, unittest::UnitTestSchema};

    use super::*;
    #[test]
    fn test_check_sum() {
        setup::log::configure();
        let slice_valid_check_sum = b"8=FIX.4.49=0000000000000000001235=A49=source56=dest98=0108=3010=240";
        let recv_frame = RecvFrame::new(slice_valid_check_sum, UnitTestSchema);
        let check_sum = recv_frame.validate_check_sum().unwrap();
        assert_eq!(check_sum, 240);

        let slice_in_valid_check_sum = b"8=FIX.4.49=0000000000000000001235=A49=source56=dest98=0108=3010=241";
        let recv_frame = RecvFrame::new(slice_in_valid_check_sum, UnitTestSchema);
        let err = recv_frame.validate_check_sum().unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, Error::InvalidChecksum(IssueAtPosition(68))));
    }
}
