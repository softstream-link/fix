use crate::error::{Error, Result};
use fix_model_core::prelude::FixByteSlice2Display;
use std::cmp::min;
use std::fmt::{Debug, Display};

// Prevent users from implementing the Read trait.
mod private {
    pub trait Sealed {}
}

pub trait Read<'origin>: private::Sealed + Display {
    /// Peek at the next byte without consuming it.
    /// Returns `None` if at the end of the stream.
    #[doc(hidden)]
    fn is_end(&mut self) -> Result<bool>;

    // on
    #[doc(hidden)]
    fn discard(&mut self);

    /// * Assumes [Self] is positioned to read right after the last `=` sign.
    /// * Will return value upto but not including the `SOH` byte.
    /// * Will position [Self] immediatly after `SOH` byte by consuming it but not return `SOH` in result.
    /// * Returns [Error::UnexpectedEof] if `SOH` is not found, Except when [SliceRead] is impl
    fn parse_value(&mut self) -> Result<&'origin [u8]>;

    /// * Assumes [Self] is positioned to read right after the last `=` sign.
    /// * Will return the value of the specified length
    /// * Returns [Error::EnexpectedEof] specified length exceeds available bytes or if byte follwoing length size is not `SOH`
    fn parse_value_with_length(&mut self, length: usize) -> Result<&'origin [u8]>;

    /// * Assumes [Self] is positioned tor read right after the last `=` sign.
    /// * Will return the value upto but not including the SOH byte. However the value will be parsed as a number of type [T].
    /// * Will position [Self] immediatly after SOH byte but not return it.
    fn parse_value_as_number<T>(&mut self) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let value = self.parse_value()?;
        SliceRead::parse_number(value)
    }

    // /// returns [Some] if the value was found while internally preserving the index of found value
    // /// returns [None] if the value was not found and Eof reached
    // fn seek_eqs(&mut self) -> Result<Option<()>>;
    /// # Returns
    /// * [`Ok(Some(&\[u8\]))`] containing the next tag value `WITHOUT` consuming it, call [parse_tag] to consume it.
    /// * [`Ok(None)`] indicating Eof
    fn peek_tag(&mut self) -> Result<Option<&'origin [u8]>>;
    /// # Returns
    /// * [`Ok(&\[u8\])`] containing the next tag value in bytes form `WHILE` consuming it and also consuming one extra byte with `=` that follows the tag
    /// * [`Error::EnexpectedEof`] if the tag value is not found
    fn parse_tag(&mut self) -> Result<Option<&'origin [u8]>>;

    fn last_peeked_tag(&self) -> Option<&'origin [u8]>;
}

// Slice read and its methods are future proofing for the possibility of reading from Io
pub struct SliceRead<'origin> {
    slice: &'origin [u8],
    idx_current: usize,
    last_seek_tag: Option<&'origin [u8]>,
    idx_last_seek_eqs: usize, // TODO save tag slice instead of index to avoid boundary checks
    idx_last_seek_soh: usize,
}
impl<'origin> SliceRead<'origin> {
    #[inline]
    pub fn new(slice: &'origin [u8]) -> Self {
        Self {
            slice,
            idx_current: 0,
            last_seek_tag: None,
            idx_last_seek_eqs: 0,
            idx_last_seek_soh: 0,
        }
    }
    #[inline]
    pub fn idx(&self) -> usize {
        self.idx_current
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }

    pub fn parse_number<T>(value: &[u8]) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match std::str::from_utf8(value) {
            Ok(s) => {
                let int = s.parse::<T>().map_err(|e| Error::Message(format!("{}", e)))?;
                Ok(int)
            }
            Err(err) => {
                #[cfg(debug_assertions)]
                log::error!("parse_number value: {}, is not a valid number. err: {}", value.to_string(), err);
                Err(Error::InvalidInteger)
            }
        }
    }
    fn seek(&mut self, byte: u8) -> Option<usize> {
        let mut idx = self.idx_current;
        loop {
            if idx >= self.slice.len() {
                return None;
            } else if self.slice[idx] == byte {
                return Some(idx);
            } else {
                idx += 1;
            }
        }
    }

    fn seek_tag(&mut self) -> Option<&'origin [u8]> {
        if self.idx_current < self.idx_last_seek_eqs {
            self.last_seek_tag
        } else {
            match self.seek(crate::EQS) {
                Some(idx) => {
                    self.idx_last_seek_eqs = idx;
                    self.last_seek_tag = Some(&self.slice[self.idx_current..self.idx_last_seek_eqs]);
                    self.last_seek_tag
                }
                None => None,
            }
        }
    }
    #[inline]
    pub fn parse_tag_infallable(&mut self) -> Option<&'origin [u8]> {
        match self.seek_tag() {
            None => None,
            some => {
                // let tag = &self.slice[self.idx_current..self.idx_last_seek_eqs];
                self.idx_current = self.idx_last_seek_eqs + 1; // advance past eqs sign
                some
            }
        }
    }
    #[inline]
    pub fn parse_value_infallable(&mut self) -> Option<&'origin [u8]> {
        match self.seek(crate::SOH) {
            Some(idx) => {
                let res = &self.slice[self.idx_current..idx];
                self.idx_current = idx + 1;
                Some(res)
            }
            None => None,
        }
    }
    /// * Finds and remembers the location of the next `SOH` byte
    /// * When called repeatedly it and current position is less then last remembered `SOH` then it immediatelly returns without repeating `SOH` search
    /// * if `SOH` is not found then this implementation assumes the end of the slice is where the value ends
    fn seek_value(&mut self) -> Result<()> {
        // if already found then just return immediatelly
        if self.idx_current >= self.idx_last_seek_soh {
            match self.seek(crate::SOH) {
                Some(idx) => {
                    self.idx_last_seek_soh = idx;
                }
                None => {
                    // // if SOH not found then show last position as UnexpectedEof
                    // return Err(Error::UnexpectedEof(self.len().into()));
                    // However because this is a SliceRead just assume this is the end of the slice
                    self.idx_last_seek_soh = self.len();
                }
            }
        }
        Ok(())
    }
}
impl<'origin> private::Sealed for SliceRead<'origin> {}
impl<'origin> Display for SliceRead<'origin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // note that idx_current can be set past end of slice once the last value is read hence need min
        let read = &self.slice[..min(self.idx_current, self.len())];
        let unread = &self.slice[min(self.idx_current, self.len())..];
        let read = read.to_string();
        let unread = unread.to_string();

        write!(f, "len: {} slice: \"{}/#{}👉{}\"", self.len(), read, self.idx_current, unread)
    }
}
impl<'origin> Debug for SliceRead<'origin> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}
impl<'origin> Read<'origin> for SliceRead<'origin> {
    #[inline]
    fn is_end(&mut self) -> Result<bool> {
        Ok(self.idx_current >= self.slice.len()) // slice.len() is already past the last readable index hence if they are equal we are past the last byte
    }

    #[inline]
    fn discard(&mut self) {
        self.idx_current += 1;
    }

    #[inline]
    fn parse_value(&mut self) -> Result<&'origin [u8]> {
        self.seek_value()?; //position  self.idx_last_seek_soh

        let res = &self.slice[self.idx_current..self.idx_last_seek_soh];
        match res.len() {
            0 => Err(Error::EmptyValue(self.idx().into())),
            _ => {
                #[cfg(debug_assertions)] // info!("remaining {}", super::deserializer::to_str(Some(&self.slice[self.idx_current..])));
                log::trace!(
                    "{:<58} parsed_val: {:?}, SOH/EOF: {},  state: {}",
                    "SliceRead::parse_value",
                    res.to_string(),
                    self.idx_last_seek_soh,
                    self
                );

                self.idx_current = self.idx_last_seek_soh + 1; // discard SOH

                Ok(res)
            }
        }
    }

    #[inline]
    fn parse_value_with_length(&mut self, length: usize) -> Result<&'origin [u8]> {
        let start = self.idx_current;
        let end = start + length;
        if end >= self.len() {
            #[cfg(debug_assertions)]
            log::error!(
                "SliceRead::parse_value_with_length expected available length: {}+1/SOH from start: {}, however only {} bytes available.",
                length,
                start,
                self.len() - start,
            );
            // returns current position where length is expected to start
            Err(Error::UnexpectedEof(self.idx().into()))
        } else if self.slice[end] != crate::SOH {
            #[cfg(debug_assertions)]
            log::error!(
                "SliceRead::parse_value_with_length: Expected SOH at index: {} but got: '{}', read: {}",
                end,
                (&self.slice[end..end + 1]).to_string(),
                self,
            );
            Err(Error::InvalidFixFrame(end.into()))
        } else {
            self.idx_current = end;
            self.discard(); // SOH
            Ok(&self.slice[start..end])
        }
    }

    #[inline]
    fn peek_tag(&mut self) -> Result<Option<&'origin [u8]>> {
        Ok(self.seek_tag())
    }
    #[inline]
    fn last_peeked_tag(&self) -> Option<&'origin [u8]> {
        self.last_seek_tag
    }
    #[inline]
    fn parse_tag(&mut self) -> Result<Option<&'origin [u8]>> {
        Ok(self.parse_tag_infallable())
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use fix_model_test::unittest::setup;

    use crate::{error::IssueAtPosition, SOH};

    use super::*;
    #[test]
    fn parse_u8() {
        setup::log::configure();
        enum Expect {
            Pass(u8),
            Fail,
        }
        use Expect::*;
        let schenarios = [("123", Pass(123)), ("00000255", Pass(u8::MAX)), ("abc", Fail), ("256", Fail)];
        for (input, expect) in schenarios.iter() {
            let res = SliceRead::parse_number::<u8>(input.as_bytes());
            info!("input: {:?}, res: {:?}", input, res);
            match expect {
                Pass(expect) => assert_eq!(res.unwrap(), *expect),
                Fail => assert!(res.is_err()),
            }
        }
    }

    #[test]
    fn parse_i8() {
        setup::log::configure();
        enum Expect {
            Pass(i8),
            Fail,
        }
        use Expect::*;
        let schenarios = [("127", Pass(i8::MAX)), ("-00000128", Pass(i8::MIN)), ("abc", Fail), ("256", Fail)];
        for (input, expect) in schenarios.iter() {
            let res = SliceRead::parse_number::<i8>(input.as_bytes());
            info!("input: {:?}, res: {:?}", input, res);
            match expect {
                Pass(expect) => assert_eq!(res.unwrap(), *expect),
                Fail => assert!(res.is_err()),
            }
        }
    }

    #[test]
    fn parse_f32() {
        setup::log::configure();
        #[derive(Debug)]
        enum Expect {
            Pass(f32),
            Fail,
        }
        use Expect::*;
        let schenarios = [
            ("3.4028235e38", Pass(f32::MAX)),
            ("-3.4028235e38", Pass(f32::MIN)),
            ("abc", Fail),
            ("256..", Fail),
        ];
        for (input, expect) in schenarios.iter() {
            let res = SliceRead::parse_number::<f32>(input.as_bytes());
            info!("input: {:?}, expected: {:?}, res: {:?}", input, expect, res);
            match expect {
                Pass(expect) => assert_eq!(res.unwrap(), *expect),
                Fail => assert!(res.is_err()),
            }
        }
    }

    #[test]
    fn test_slice_read_valid() {
        setup::log::configure();
        let input = "8=FIX.4.2|9=123|".replace("|", &(SOH as char).to_string());
        info!("input: {:?}", input);

        let mut read = SliceRead::new(input.as_bytes());
        info!("read: {}", read);

        let peek_tag = read.peek_tag().unwrap().unwrap();
        info!("peek_tag:  {:?}, read: {}", peek_tag.to_string(), read);
        assert_eq!(peek_tag, b"8");
        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(peek_tag, parse_tag);
        let value = read.parse_value().unwrap();
        info!("value: {:?}, read: {}", value.to_string(), read);
        assert_eq!(value, b"FIX.4.2");

        let peek_tag = read.peek_tag().unwrap().unwrap();
        info!("peek_tag:  {:?}, read: {}", peek_tag.to_string(), read);
        assert_eq!(peek_tag, b"9", "pkeed_tag: {:?}", peek_tag.to_string());
        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(peek_tag, parse_tag);
        let value = read.parse_value().unwrap();
        info!("value:   {:?}, read: {}", value.to_string(), read);
        assert_eq!(value, b"123");

        let peek_tag = read.peek_tag().unwrap();
        info!("peek_tag: {:?}", peek_tag);
        assert!(matches!(peek_tag, None));

        let parse_tag = read.parse_tag().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, None);

        let err = read.parse_value().unwrap_err();
        info!("err: {:?}, read: {}", err, read);
        assert!(matches!(err, Error::EmptyValue(IssueAtPosition(16))));

        /////////////////////////////////
        let input = b"8=FIX.4.2"; // NO terminating SOH
        info!("input: {:?}", input.to_string());

        let mut read = SliceRead::new(input);
        info!("read: {}", read.to_string());

        let peek_tag = read.peek_tag().unwrap().unwrap();
        info!("peek_tag: {:?}, read: {}", peek_tag.to_string(), read);
        assert_eq!(peek_tag, b"8");
        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        let value = read.parse_value().unwrap();
        info!("value: {:?}. read: {}", value.to_string(), read);
        assert_eq!(value, b"FIX.4.2");
    }

    #[test]
    fn test_slice_read_valid_with_lenth() {
        setup::log::configure();
        let input = "0=10|1=0123456789|".replace("|", &(SOH as char).to_string());
        info!("input: {:?}", input);

        let mut read = SliceRead::new(input.as_bytes());
        info!("read: {}", read);

        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"0");

        let len: usize = read.parse_value_as_number().unwrap();
        info!("len: {:?}, read: {}", len, read);
        assert_eq!(len, 10);

        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"1");

        let value = read.parse_value_with_length(len).unwrap();
        info!("value: {:?}, read: {}", value.to_string(), read);
        assert_eq!(value, b"0123456789");
    }
    #[test]
    fn test_slice_read_invalid_with_length() {
        setup::log::configure();
        let input = "0=10|1=0123456789".replace("|", &(SOH as char).to_string()); // NO terminating SOH
        info!("input: {:?}", input);

        let mut read = SliceRead::new(input.as_bytes());
        info!("read: {}", read);

        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"0");

        let len: usize = read.parse_value_as_number().unwrap();
        info!("len: {:?}, read: {}", len, read);
        assert_eq!(len, 10);

        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"1");

        let res = read.parse_value_with_length(len);
        info!("value: {:?}, read: {}", res, read);
        assert!(matches!(res, Err(Error::UnexpectedEof(IssueAtPosition(7)))));

        /////////////
        let input = "0=10|1=0123456789????".replace("|", &(SOH as char).to_string()); // NO terminating SOH in value of tag 1 at length 11
        info!("input: {:?}", input);

        let mut read = SliceRead::new(input.as_bytes());
        info!("read: {}", read);
        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"0");

        let len: usize = read.parse_value_as_number().unwrap();
        info!("len: {:?}, read: {}", len, read);
        assert_eq!(len, 10);

        let parse_tag = read.parse_tag().unwrap().unwrap();
        info!("parse_tag: {:?}, read: {}", parse_tag.to_string(), read);
        assert_eq!(parse_tag, b"1");

        let res = read.parse_value_with_length(len);
        info!("value: {:?}, read: {}", res, read);
        assert!(matches!(res, Err(Error::InvalidFixFrame(IssueAtPosition(17)))));
    }
}
