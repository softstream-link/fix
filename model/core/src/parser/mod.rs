use std::{error::Error, fmt::Display};

use bytes::{BufMut, BytesMut};
pub type TagNumber = u32;

#[derive(Debug)]
pub struct FixParsingError {
    msg: String,
    source: Option<Box<dyn Error>>,
}
// impl<C: AsRef<str>> From<C> for ParsingFixError {
//     fn from(value: C) -> Self {
//         Self {
//             msg: value.as_ref().to_owned(),
//             source: None,
//         }
//     }
// }
impl<C: AsRef<str>, S: 'static + Error> From<(C, S)> for FixParsingError {
    fn from(value: (C, S)) -> Self {
        let (msg, source) = value;
        Self {
            msg: msg.as_ref().to_owned(),
            source: Some(source.into()),
        }
    }
}
impl Display for FixParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}
impl Error for FixParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|v| v.as_ref())
    }
}
pub type Result<T> = std::result::Result<T, FixParsingError>;

#[derive(Debug, PartialEq)]
pub struct FixToken {
    idx_token_start: usize,
    idx_equal_sign: usize,
    idx_token_end: usize,
}
impl FixToken {
    pub fn new(idx_token_start: usize, idx_equal_sign: usize, idx_token_end: usize) -> Self {
        Self {
            idx_token_start,
            idx_equal_sign,
            idx_token_end,
        }
    }
}

pub trait MessageWriter {
    fn add_string<S: AsRef<str>>(&mut self, tag: TagNumber, value: S);
}
pub trait MessageReader {
    // fn read_slice(&self, tag: Tag) -> Option<&[u8]>;
}

#[derive(Clone)]
pub struct Message {
    buf: BytesMut,
}
impl Message {
    const DELIMITER: u8 = 0x01;
    const DELIMITER_AS_CHAR: char = Message::DELIMITER as char;
    const DELIMITER_AS_PIPE: &'static str = "|";
    const EQUAL_SIGN: u8 = b'=';
    pub fn with_capacity(capacity: usize) -> Message {
        Message {
            buf: BytesMut::with_capacity(capacity),
        }
    }

    #[inline(always)]
    fn is_first_byte_delimiter(&self, skip_from_start: usize) -> bool {
        self.buf[0 + skip_from_start] == Self::DELIMITER
    }
    #[inline(always)]
    fn is_last_byte_delimiter(&self, skip_from_end: usize) -> bool {
        self.buf[self.buf.len() - 1 - skip_from_end] == Self::DELIMITER
    }
    #[inline(always)]
    fn read_token_as_slice(&self, token: &FixToken) -> &[u8] {
        &self.buf[token.idx_token_start..token.idx_token_end + 1]
    }
    #[inline(always)]
    fn read_value_as_slice(&self, token: &FixToken) -> &[u8] {
        &self.buf[token.idx_equal_sign + 1..token.idx_token_end + 1]
    }
    #[inline(always)]
    fn read_tag_as_slice(&self, token: &FixToken) -> &[u8] {
        &self.buf[token.idx_token_start..token.idx_equal_sign]
    }
    #[inline(always)]
    fn get_string(&self, token: &FixToken) -> &str {
        let slice = &self.buf[token.idx_equal_sign + 1..token.idx_token_end + 1];
        unsafe { std::str::from_utf8_unchecked(slice) }
    }
    #[inline(always)]
    fn to_string_section(&self, idx_start: usize, idx_end: usize) -> String {
        let slice = &self.buf[idx_start..idx_end];
        let value = unsafe { std::str::from_utf8_unchecked(slice) };
        value.replace(Message::DELIMITER_AS_CHAR, Message::DELIMITER_AS_PIPE)
    }
    #[inline(always)]
    fn is_tag_equal(&self, tag: TagNumber, idx_tag_start: usize, idx_tag_end: usize) -> Result<bool> {
        let slice = &self.buf[idx_tag_start..idx_tag_end];
        let value = unsafe { std::str::from_utf8_unchecked(slice) };
        match value.parse::<TagNumber>() {
            Ok(v) => Ok(v == tag),
            Err(err) => {
                let start = self.to_string_section(0, idx_tag_start);
                let end = self.to_string_section(idx_tag_end, self.buf.len());
                Err((
                    format!("value: {:?} can't be parsed into TagNumber. '{}>>>>>{}<<<<<{}'", value, start, value, end),
                    err,
                )
                    .into())
            }
        }
    }
    #[inline(always)]
    fn find_tag_fwd(&self, tag: TagNumber, skip: Option<usize>) -> Result<Option<FixToken>> {
        let skip = skip.unwrap_or(0);
        let idx_last = self.buf.len() - 1;
        let idx_first = 0 + skip;
        let mut idx_token_start = match self.is_first_byte_delimiter(skip) {
            true => idx_first + 1,
            false => idx_first,
        };
        let mut idx_last_seen_delimiter: usize = idx_first;
        let idx_last_seen_delimiter_overflow_correction = match self.is_first_byte_delimiter(skip) {
            true => 0,
            false => 1,
        };

        #[allow(unused_assignments)]
        let mut idx_token_end = match self.is_last_byte_delimiter(0) {
            true => idx_last - 1,
            false => idx_last,
        };
        let mut idx_equal_sign: usize = idx_token_start;

        for (idx_current, v) in self.buf.iter().enumerate().skip(skip) {
            match *v {
                Self::DELIMITER => {
                    // found new delimiter
                    if idx_last_seen_delimiter < idx_current {
                        idx_token_start = idx_last_seen_delimiter + 1 - idx_last_seen_delimiter_overflow_correction;
                        idx_token_end = idx_current - 1;
                        idx_last_seen_delimiter = idx_current;

                        if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
                            if self.is_tag_equal(tag, idx_token_start, idx_equal_sign)? {
                                return Ok(Some(FixToken {
                                    idx_token_start,
                                    idx_equal_sign,
                                    idx_token_end,
                                }));
                            }
                        }
                    }
                }
                _ => {
                    // can't have a separate branch for EQ because it can be last byte in the buffer
                    if *v == Self::EQUAL_SIGN {
                        idx_equal_sign = idx_current;
                    }
                    // we reached last element and it does not not have delimiter at the end
                    if idx_last == idx_current {
                        // we reached the last byte in the buffer and it is not a delimiter or equal sign hence delimiter START is now whatever happens to be delimiter END
                        idx_token_start = idx_last_seen_delimiter + 1 - idx_last_seen_delimiter_overflow_correction;
                        idx_token_end = idx_current;

                        if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
                            if self.is_tag_equal(tag, idx_token_start, idx_equal_sign)? {
                                return Ok(Some(FixToken {
                                    idx_token_start,
                                    idx_equal_sign,
                                    idx_token_end,
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    #[inline(always)]
    fn find_tag_rev(&self, tag: TagNumber, skip: Option<usize>) -> Result<Option<FixToken>> {
        let skip = skip.unwrap_or(0);
        let idx_last = self.buf.len() - 1 - skip;
        let idx_first = 0;
        let mut idx_token_start = match self.is_first_byte_delimiter(0) {
            true => idx_first + 1,
            false => idx_first,
        };
        let mut idx_last_seen_delimiter = match self.is_last_byte_delimiter(skip) {
            true => idx_last,
            false => idx_last + 1,
        };
        let mut idx_token_end = match self.is_last_byte_delimiter(skip) {
            true => idx_last - 1,
            false => idx_last,
        };
        let mut idx_equal_sign: usize = idx_token_end;

        // enumerate in reverse idx will be [4, 3, 2, 1, 0] until found a matching tag
        for (idx_current, v) in self.buf.iter().enumerate().rev().skip(skip) {
            match *v {
                Self::DELIMITER => {
                    // found new delimiter
                    if idx_last_seen_delimiter > idx_current {
                        idx_token_start = idx_current + 1;
                        idx_token_end = idx_last_seen_delimiter - 1;
                        idx_last_seen_delimiter = idx_current;
                        if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
                            if self.is_tag_equal(tag, idx_token_start, idx_equal_sign)? {
                                return Ok(Some(FixToken {
                                    idx_token_start,
                                    idx_equal_sign,
                                    idx_token_end,
                                }));
                            }
                        }
                    }
                }

                _ => {
                    // can't have a separate branch for EQ because it can be first byte in the buffer
                    if *v == Self::EQUAL_SIGN {
                        idx_equal_sign = idx_current;
                    }
                    // we reached first element in reverse and it does not have a delimiter in front
                    if idx_current == idx_first {
                        idx_token_start = idx_current;
                        idx_token_end = idx_last_seen_delimiter - 1;
                        // we reached the first byte in the buffer and it is not a delimiter or equal sign hence delimiter END is now whatever happens to be delimiter START
                        if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
                            if self.is_tag_equal(tag, idx_first, idx_equal_sign)? {
                                return Ok(Some(FixToken {
                                    idx_token_start: idx_first,
                                    idx_equal_sign,
                                    idx_token_end,
                                }));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
    fn write_slice(&mut self, slice: &[u8]) {
        self.buf.put_slice(slice);
    }
}
impl MessageWriter for Message {
    fn add_string<S: AsRef<str>>(&mut self, tag: TagNumber, value: S) {
        self.buf.put_slice(tag.to_string().as_bytes()); // TODO avoid alloc, write to [u8; 10]?
        self.buf.put_u8(Self::EQUAL_SIGN);
        self.buf.put_slice(value.as_ref().as_bytes());
        self.buf.put_u8(Self::DELIMITER);
    }
}
impl MessageReader for Message {}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string_section(0, self.buf.len()).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fix_model_test::unittest::setup;
    use log::info;

    #[test]
    fn test_is_tag_equal_error() -> Result<()> {
        setup::log::configure();

        // empty tag should always return false
        let mut msg = Message::with_capacity(1024);
        msg.write_slice(b"\x01=\x01");
        info!("msg: '{}'", msg);
        let res = msg.is_tag_equal(2, 1, 1);
        assert!(res.is_err());
        info!("{:?}", res.unwrap_err());

        let mut msg = Message::with_capacity(1024);
        msg.write_slice(b"\x01-1=\x01");
        info!("msg: '{}'", msg);
        let res = msg.is_tag_equal(2, 1, 2);
        assert!(res.is_err());
        info!("{:?}", res.unwrap_err());

        Ok(())
    }

    #[test]
    fn test_find_tag_valid_not_empty() -> Result<()> {
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_valid = vec![
            b"0=2345".as_slice(),
            b"\x010=2345\x01".as_slice(),
            b"\x010=2345".as_slice(),
            b"0=2345\x01".as_slice(),
        ];
        let as_str_expected = "2345";
        for m in msg_valid {
            let mut msg = Message::with_capacity(1024);
            msg.write_slice(m);
            info!("*********************************");
            info!("msg: '{}'", msg);

            let search_tag = 0;
            // rev
            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
            assert!(token.is_some());
            let as_str_actual = msg.get_string(&token.unwrap());
            info!("as_str_actual: {:?}", as_str_actual);
            assert_eq!(as_str_expected, as_str_actual);
            // fwd
            let token = msg.find_tag_fwd(search_tag, None)?;
            info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
            assert!(token.is_some());
            let as_str_actual = msg.get_string(&token.unwrap());
            info!("as_str_actual: {:?}", as_str_actual);
            assert_eq!(as_str_expected, as_str_actual);

            let search_tag = 1;
            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
            assert!(token.is_none());

            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
            assert!(token.is_none());
        }
        Ok(())
    }

    #[test]
    fn test_find_tag_valid_empty() -> Result<()> {
        setup::log::configure_compact(log::LevelFilter::Info);
        let msg_valid = vec![b"0=".as_slice(), b"\x010=\x01".as_slice(), b"\x010=".as_slice(), b"0=\x01".as_slice()];
        let as_str_expected = "";
        for m in msg_valid {
            let mut msg = Message::with_capacity(1024);
            msg.write_slice(m);
            info!("*********************************");
            info!("msg: '{}'", msg);

            let search_tag = 0;
            // rev
            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
            assert!(token.is_some());
            let as_str_actual = msg.get_string(&token.unwrap());
            info!("as_str_actual: {:?}", as_str_actual);
            assert_eq!(as_str_expected, as_str_actual);
            // fwd
            let token = msg.find_tag_fwd(search_tag, None)?;
            info!("find_tag_fwd <- search_tag: {} {:?}", search_tag, token);
            assert!(token.is_some());
            let as_str_actual = msg.get_string(&token.unwrap());
            info!("as_str_actual: {:?}", as_str_actual);
            assert_eq!(as_str_expected, as_str_actual);

            let search_tag = 1;
            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
            assert!(token.is_none());

            let token = msg.find_tag_rev(search_tag, None)?;
            info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
            assert!(token.is_none());
        }

        Ok(())
    }
}
