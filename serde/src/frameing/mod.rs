use crate::{
    de::read::{Read, SliceRead},
    prelude::{Error, Result},
};
use fix_model_core::types::display::FixByteSlice2Display;

#[derive(Debug)]
pub struct Parts<'a> {
    pub first: &'a [u8],
    pub second: &'a [u8],
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
                    None => Ok(None),                                                                        // frame incomplete
                    Some(body_len) if body_len.is_empty() => Err(Error::InvalidFixFrame(read.idx().into())), // frame invalid
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

pub fn split_off_check_sum<'a>(buf: &'a [u8]) -> Result<Parts> {
    let mut read = SliceRead::new(buf);
    let _begin_string = match read.parse_tag_infallable() {
        Some(b"8") => read.parse_value()?,
        opt => {
            #[cfg(debug_assertions)]
            log::error!(
                "Expected BeginString tag '8', instead got: {:?}, read: {}",
                opt.unwrap_or_else(|| b"").to_string(),
                read
            );
            return Err(Error::InvalidFixFrame(read.idx().into()));
        }
    };
    let body_length: usize = match read.parse_tag_infallable() {
        Some(b"9") => read.parse_value_as_number()?,
        opt => {
            #[cfg(debug_assertions)]
            log::error!(
                "Expected BodyLength tag '9', instead got: {:?}, read: {}",
                opt.unwrap_or_else(|| b"").to_string(),
                read
            );
            return Err(Error::InvalidFixFrame(read.idx().into()));
        }
    };
    // 01234567890123456789
    // 8=fix9=535=A10=000
    // idx = 9 + body_length = 5 == 14 ... buf.len() can't be less then 14
    let idx_body_end = read.idx() + body_length; // body_end includes SOH right before checksum
    if idx_body_end > buf.len() {
        #[cfg(debug_assertions)]
        log::error!(
            "BodyLength={} points idx_body_end: {} from current position, however its beyond slice length: {} read: {}",
            body_length,
            idx_body_end,
            buf.len(),
            read,
        );

        return Err(Error::InvalidFixFrame(read.idx().into()));
    }
    if buf[idx_body_end - 1] != crate::SOH {
        #[cfg(debug_assertions)]
        log::error!(
            "Expected SOH at idx_body_end-1: {}, instead got: '{}' read: {}",
            idx_body_end - 1,
            (&buf[idx_body_end - 1..idx_body_end]).to_string(),
            read
        );
        return Err(Error::InvalidFixFrame((idx_body_end - 1).into()));
    }
    let check_sum = &buf[idx_body_end..];
    if check_sum.len() != 7 || &check_sum[..3] != b"10=" {
        #[cfg(debug_assertions)]
        log::error!("Expected CheckSum tag '10=xxx', instead got: '{}' read: {}", check_sum.to_string(), read);
        return Err(Error::InvalidFixFrame((idx_body_end - 1).into()));
    }
    Ok(Parts {
        first: &buf[..idx_body_end],
        second: &buf[idx_body_end..],
    })
}

pub fn check_sum(buf: &[u8]) -> (u8, [u8; 3]) {
    let check_sum_u8 = (buf.iter().fold(0_usize, |acc, &b| acc.wrapping_add(b as usize)) % 256) as u8;

    use std::io::Write;
    let mut check_sum_bytes = [0_u8; 3];
    let mut buf = itoa::Buffer::new();
    let buf = buf.format(check_sum_u8);
    write!(&mut check_sum_bytes[..], "{:0>3}", buf).unwrap();
    (check_sum_u8, check_sum_bytes)
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::IssueAtPosition;
    use fix_model_core::types::display::FixByteSlice2Display;
    use fix_model_test::unittest::setup;
    use log::info;

    #[test]
    fn test_split_off_check_sum() {
        setup::log::configure();
        let buf = "8=fix9=535=A10=000"; // VALID WITH TRAILER
        let parts = split_off_check_sum(buf.as_bytes()).unwrap();
        info!("parts.fist: {}", parts.first.to_string());
        info!("parts.second: {}", parts.second.to_string());
        assert_eq!(parts.first, "8=fix9=535=A".as_bytes());
        assert_eq!(parts.second, "10=000".as_bytes());

        //                 01234567890123456789
        let buf = "8=fix9=535=A"; // INVALID NO CHECKSUM
        let err = split_off_check_sum(buf.as_bytes()).unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, crate::prelude::Error::InvalidFixFrame(IssueAtPosition(14))));

        //                 01234567890123456789
        let buf = "8=fix9=535=A?"; // INVALID SOH TERMINATOR
        let err = split_off_check_sum(buf.as_bytes()).unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, crate::prelude::Error::InvalidFixFrame(IssueAtPosition(14))));

        //                 01234567890123456789
        let buf = "8=fix9=535=A"; // INVALID FRAME TOO SHORT
        let err = split_off_check_sum(buf.as_bytes()).unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, crate::prelude::Error::InvalidFixFrame(IssueAtPosition(10))));
    }

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
    fn test_check_sum() {
        setup::log::configure();
        let buf = "8=fix9=535=A"; // 10=080";
        info!("buf: {:?}", buf.as_bytes().to_string());
        let (ch_u8, ch_bytes) = check_sum(buf.as_bytes());
        info!("ch_u8: {}", ch_u8);
        info!("ch_bytes: {}", ch_bytes.to_string());
        assert_eq!(ch_u8, 80);
        assert_eq!(&ch_bytes, b"080");

        let buf = "8=FIX.4.49=14835=D34=108049=TESTBUY152=20180920-18:14:19.50856=TESTSELL111=63673064027889863415=USD21=238=700040=154=155=MSFT60=20180920-18:14:19.492"; // 10=092";
        info!("buf: {}", buf.as_bytes().to_string());
        let (ch_u8, ch_bytes) = check_sum(buf.as_bytes());
        info!("ch_u8: {}", ch_u8);
        info!("ch_bytes: {}", ch_bytes.to_string());
        assert_eq!(ch_u8, 92);
        assert_eq!(&ch_bytes, b"092");
    }
}
