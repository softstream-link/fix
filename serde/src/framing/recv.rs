use crate::framing::compute_check_sum;
use crate::prelude::{Deserializer, Error, Result, SliceRead};
use fix_model_core::schema::Schema;
use fix_model_core::types::display::FixByteSlice2Display;
use serde::Deserialize;

use super::TrailerCheckSum;

pub struct RecvFrame<'de, X> {
    frame: &'de [u8],
    // phantom: PhantomData<X>,
    deserializer: Option<crate::Deserializer<SliceRead<'de>, X>>,
}
impl<'de, X: Schema> RecvFrame<'de, X> {
    const CHECK_SUM_LEN: usize = 7;

    pub fn new(frame: &'de [u8], _schema: X) -> Self {
        Self { frame, deserializer: None }
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
    pub fn check_sum(&mut self) -> Result<u8> {
        let (root_slice, check_sum_slice) = self.split()?;
        let mut des = Deserializer::<_, X>::new(SliceRead::new(check_sum_slice));

        let tag_value_check_sum = TrailerCheckSum::deserialize(&mut des)?;

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
        let des = self.deserializer.as_mut().expect("split should have assigned a deserializer");
        T::deserialize(des)
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
        let mut recv_frame = RecvFrame::new(slice_valid_check_sum, UnitTestSchema);
        let check_sum = recv_frame.check_sum().unwrap();
        assert_eq!(check_sum, 240);

        let slice_in_valid_check_sum = b"8=FIX.4.49=0000000000000000001235=A49=source56=dest98=0108=3010=241";
        let mut recv_frame = RecvFrame::new(slice_in_valid_check_sum, UnitTestSchema);
        let err = recv_frame.check_sum().unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, Error::InvalidChecksum(IssueAtPosition(68))));
    }
}
