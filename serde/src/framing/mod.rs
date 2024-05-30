pub mod decode;
pub mod enchode;

use crate::{
    de::read::SliceRead,
    prelude::{Error, Result},
};
use serde::Serialize;

fix_model_generator::fix_string!(BeginString, 8);
fix_model_generator::fix_usize_fixed_length!(BodyLength, 9);
fix_model_generator::fix_string!(MsgType, 35);
fix_model_generator::fix_string!(SenderCompID, 49);
fix_model_generator::fix_string!(TargetCompID, 56);
fix_model_generator::fix_u8_fixed_length!(CheckSum, 10);

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct TaggedBeginString<S> {
    #[serde(rename = "8")]
    #[serde(alias = "BeginString")]
    begin_string: BeginString<S>,
}
impl<S: serde::Serialize> Serialize for TaggedBeginString<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TaggedBeginString", 1)?;
            state.serialize_field("BeginString", &self.begin_string)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TaggedBeginString", 1)?;
            state.serialize_field("8", &self.begin_string)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct TaggedBodyLength {
    #[serde(rename = "9")]
    #[serde(alias = "BodyLength")]
    body_length: BodyLength,
}
impl Serialize for TaggedBodyLength {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TaggedBodyLength", 1)?;
            state.serialize_field("BodyLength", &self.body_length)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TaggedBodyLength", 1)?;
            state.serialize_field("9", &self.body_length)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct TaggedMsgType<S> {
    #[serde(rename = "35")]
    #[serde(alias = "MsgType")]
    msg_type: MsgType<S>,
}
impl<S: serde::Serialize> Serialize for TaggedMsgType<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TaggedMsgType", 1)?;
            state.serialize_field("MsgType", &self.msg_type)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TaggedMsgType", 1)?;
            state.serialize_field("35", &self.msg_type)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct TaggedSenderCompID<S> {
    #[serde(rename = "49")]
    #[serde(alias = "SenderCompID")]
    sender_comp_id: SenderCompID<S>,
}
impl<S: serde::Serialize> Serialize for TaggedSenderCompID<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TaggedSenderCompID", 1)?;
            state.serialize_field("SenderCompID", &self.sender_comp_id)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TaggedSenderCompID", 1)?;
            state.serialize_field("49", &self.sender_comp_id)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct TaggedTargetCompID<S> {
    #[serde(rename = "56")]
    #[serde(alias = "TargetCompID")]
    target_comp_id: TargetCompID<S>,
}
impl<S: serde::Serialize> Serialize for TaggedTargetCompID<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TaggedTargetCompID", 1)?;
            state.serialize_field("TargetCompID", &self.target_comp_id)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TaggedTargetCompID", 1)?;
            state.serialize_field("56", &self.target_comp_id)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Header1EnvelopeSequence<S> {
    #[serde(rename = "8")]
    #[serde(alias = "BeginString")]
    pub begin_string: BeginString<S>,

    #[serde(rename = "9")]
    #[serde(alias = "BodyLength")]
    pub body_length: BodyLength,
}
impl<S: serde::Serialize> Serialize for Header1EnvelopeSequence<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("Header1EnvelopeSequence", 2)?;
            state.serialize_field("BeginString", &self.begin_string)?;
            state.serialize_field("BodyLength", &self.body_length)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("Header1EnvelopeSequence", 2)?;
            state.serialize_field("8", &self.begin_string)?;
            state.serialize_field("9", &self.body_length)?;
            state.end()
        }
    }
}
impl<S: AsRef<str>> Header1EnvelopeSequence<S> {
    #[inline]
    pub fn new(begin_string: BeginString<S>) -> Self {
        Self {
            begin_string,
            body_length: Default::default(),
        }
    }
    #[allow(clippy::identity_op)]
    #[inline]
    pub fn size(&self) -> usize {
        0
        + b"8=".len() + self.begin_string.as_ref().as_bytes().len() + b"".len() // BeginString
        + b"9=".len() + 20 + b"".len() // fix_usize_fixed_length!(BodyLength, 9); generates 20 zero padded string
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Header2TypeCompIdSequence<S> {
    #[serde(rename = "35")]
    #[serde(alias = "MsgType")]
    pub msg_type: MsgType<S>,

    #[serde(rename = "49")]
    #[serde(alias = "SenderCompID")]
    pub sender_comp_id: SenderCompID<S>,

    #[serde(rename = "56")]
    #[serde(alias = "TargetCompID")]
    pub target_comp_id: TargetCompID<S>,
}
impl<S: serde::Serialize> Serialize for Header2TypeCompIdSequence<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("Header2TypeCompIdSequence", 3)?;
            state.serialize_field("MsgType", &self.msg_type)?;
            state.serialize_field("SenderCompID", &self.sender_comp_id)?;
            state.serialize_field("TargetCompID", &self.target_comp_id)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("Header2TypeCompIdSequence", 3)?;
            state.serialize_field("35", &self.msg_type)?;
            state.serialize_field("49", &self.sender_comp_id)?;
            state.serialize_field("56", &self.target_comp_id)?;
            state.end()
        }
    }
}
impl<S: AsRef<str>> Header2TypeCompIdSequence<S> {
    #[inline]
    pub fn new(msg_type: MsgType<S>, sender_comp_id: SenderCompID<S>, target_comp_id: TargetCompID<S>) -> Self {
        Self {
            msg_type,
            sender_comp_id,
            target_comp_id,
        }
    }

    #[allow(clippy::identity_op)]
    pub fn size(&self) -> usize {
        0
        + b"35=".len() + self.msg_type.as_ref().as_bytes().len() + b"".len() // MsgType
        + b"49=".len() + self.sender_comp_id.as_ref().as_bytes().len() + b"".len() // SenderCompID
        + b"56=".len() + self.target_comp_id.as_ref().as_bytes().len() + b"".len()
        // TargetCompID
    }
}

#[derive(serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Header2CompIdSequence<S> {
    #[serde(rename = "49")]
    #[serde(alias = "SenderCompID")]
    pub sender_comp_id: SenderCompID<S>,

    #[serde(rename = "56")]
    #[serde(alias = "TargetCompID")]
    pub target_comp_id: TargetCompID<S>,
}
impl<S> Header2CompIdSequence<S> {
    #[inline]
    pub fn new(sender_comp_id: SenderCompID<S>, target_comp_id: TargetCompID<S>) -> Self {
        Self {
            sender_comp_id,
            target_comp_id,
        }
    }
}
impl<S: serde::Serialize> Serialize for Header2CompIdSequence<S> {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("Header2CompIdSequence", 2)?;
            state.serialize_field("SenderCompID", &self.sender_comp_id)?;
            state.serialize_field("TargetCompID", &self.target_comp_id)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("Header2CompIdSequence", 2)?;
            state.serialize_field("49", &self.sender_comp_id)?;
            state.serialize_field("56", &self.target_comp_id)?;
            state.end()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(super) struct TaggedCheckSum {
    #[serde(rename = "10")]
    #[serde(alias = "CheckSum")]
    check_sum: CheckSum,
}
impl TaggedCheckSum {
    #[inline]
    fn serialize<__S: serde::Serializer>(&self, serializer: __S) -> std::result::Result<__S::Ok, __S::Error> {
        use serde::ser::SerializeStruct;
        if serializer.is_human_readable() {
            let mut state = serializer.serialize_struct("TrailerCheckSum", 1)?;
            state.serialize_field("CheckSum", &self.check_sum)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("TrailerCheckSum", 1)?;
            state.serialize_field("10", &self.check_sum)?;
            state.end()
        }
    }
}

pub fn find_frame_end(buf: &[u8]) -> Result<Option<usize>> {
    let mut read = SliceRead::new(buf);

    // beging string tag
    match read.parse_tag_infallable() {
        None => Ok(None), // frame incomplete
        Some(_begin_string_tag) => match read.parse_value_infallable() {
            None => Ok(None), // frame incomplete
            // body length tag
            Some(_begin_string) => match read.parse_tag_infallable() {
                None => Ok(None), // frame incomplete
                Some(_body_len_tag) => match read.parse_value_infallable() {
                    None => Ok(None),                                           // frame incomplete
                    Some([]) => Err(Error::InvalidFixFrame(read.idx().into())), // frame invalid // empty slice
                    Some(body_len) => {
                        let body_len = match SliceRead::parse_number::<usize>(body_len) {
                            Ok(body_len) => body_len,
                            Err(_) => return Err(Error::InvalidFixFrame(read.idx().into())),
                        };
                        let check_sum_len = b"10=000".len();
                        let idx_body_end = read.idx() + body_len;
                        match buf.len() < idx_body_end + check_sum_len {
                            true => Ok(None),                                // incomplete
                            false => Ok(Some(idx_body_end + check_sum_len)), // frame complete
                        }
                    }
                },
            },
        },
    }
}
/// Find the end of the frame by searching for the next BeginString tag=value pair.
// will return index of the next BeginString tag=value pair if found None otherwises
pub fn find_frame_end_with_begin_string_tag_value(buf: &[u8], begin_string_tag_value: &[u8]) -> Option<usize> {
    for idx in begin_string_tag_value.len()..buf.len() - begin_string_tag_value.len() {
        let chunk = &buf[idx..idx + begin_string_tag_value.len()];
        if chunk == begin_string_tag_value {
            return Some(idx);
        }
    }
    None
}

pub fn compute_check_sum(buf: &[u8]) -> u8 {
    (buf.iter().fold(0_usize, |acc, &b| acc.wrapping_add(b as usize)) % 256) as u8
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::IssueAtPosition;
    use fix_model_core::types::display::FixByteSlice2Display;
    use fix_model_test::unittest::setup;
    use log::info;

    #[test]
    fn test_find_frame_end() {
        setup::log::configure();
        let buf = "8=fix9=535=A10=000"; // ONE FRAME EXACT
        info!("buf: {:?}", buf.as_bytes().to_string());
        let idx = find_frame_end(buf.as_bytes()).unwrap().unwrap();
        info!("idx: {}, len: {}", idx, buf.len());
        info!("frame: {:?}", (&buf.as_bytes()[..idx]).to_string());
        assert_eq!(idx, buf.len());

        let buf = "8=fix9=535=A10=0008=fix"; // ONE + 1/2 FRAME
        info!("buf: {:?}", buf.as_bytes().to_string());
        let idx = find_frame_end(buf.as_bytes()).unwrap().unwrap();
        info!("idx: {}, len: {}", idx, buf.len());
        info!("frame: {:?}", (&buf.as_bytes()[..idx]).to_string());
        assert_eq!(idx, 22);

        let buf = "8=fix9=535=A10=000"; // INCOMPLETE FRAME
        info!("buf: {:?}", buf.as_bytes().to_string());
        let idx = find_frame_end(buf.as_bytes()).unwrap();
        info!("idx: {:?}, len: {}", idx, buf.len());
        assert!(matches!(idx, None));

        let buf = "8=fix9=x35=A10=000"; // INVALID FRAME
        info!("buf: {:?}", buf.as_bytes().to_string());
        let err = find_frame_end(buf.as_bytes()).unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, Error::InvalidFixFrame(IssueAtPosition(10))));
    }

    #[test]
    fn test_find_frame_end_with_begin_string_tag() {
        setup::log::configure();
        let buf = "8=fix9=535=A10=000"; // ONE FRAME EXACT
        info!("buf: {:?}", buf.as_bytes().to_string());
        let opt = find_frame_end_with_begin_string_tag_value(buf.as_bytes(), b"8=fix");
        info!("opt: {:?}, len: {}", opt, buf.len());
        assert!(opt.is_none());

        let buf = "8=fix9=535=A10=0008=fix"; // ONE + 1/2 FRAME
        info!("buf: {:?}", buf.as_bytes().to_string());
        let idx = find_frame_end_with_begin_string_tag_value(buf.as_bytes(), b"8=fix").unwrap();
        info!("idx: {}, len: {}", idx, buf.len());
        info!("frame: {:?}", (&buf.as_bytes()[..idx]).to_string());
        assert_eq!(idx, 22);
    }

    #[test]
    fn test_check_sum() {
        setup::log::configure();
        let buf = "8=fix9=535=A"; // 10=080";
        info!("buf: {:?}", buf.as_bytes().to_string());
        let csum = compute_check_sum(buf.as_bytes());
        info!("csum: {}", csum);
        assert_eq!(csum, 80);

        let buf = "8=FIX.4.49=14835=D34=108049=TESTBUY152=20180920-18:14:19.50856=TESTSELL111=63673064027889863415=USD21=238=700040=154=155=MSFT60=20180920-18:14:19.492"; // 10=092";
        info!("buf: {}", buf.as_bytes().to_string());
        let csum = compute_check_sum(buf.as_bytes());
        info!("csum: {}", csum);
        assert_eq!(csum, 92);
    }
}
