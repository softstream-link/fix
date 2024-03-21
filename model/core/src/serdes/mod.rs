///! Key docs https://www.fixtrading.org/standards/tagvalue-online/
pub mod des;
pub mod ser;

const SOH_U8: u8 = 0x01;
const SOH_CHAR: char = SOH_U8 as char;
const PIPE_STR: &'static str = "|";
const EQS_U8: u8 = b'=';

// -----------------------------------------------------------------------------------------------

// use std::{error::Error, fmt::Display, io::Write};

// use crate::prelude::{MessageReader, MessageWriter, ParsingError, ParsingErrorKind,  TagStrWithEqSign, ValueStr};
// use bytes::{BufMut, BytesMut};
// use log::warn;

// #[derive(Debug)]
// pub struct FixParsingError {
//     msg: String,
//     source: Option<Box<dyn Error>>,
// }
// impl<C: AsRef<str>, S: 'static + Error> From<(C, S)> for FixParsingError {
//     fn from(value: (C, S)) -> Self {
//         let (msg, source) = value;
//         Self {
//             msg: msg.as_ref().to_owned(),
//             source: Some(source.into()),
//         }
//     }
// }
// impl Display for FixParsingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.msg.fmt(f)
//     }
// }
// impl Error for FixParsingError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         self.source.as_ref().map(|v| v.as_ref())
//     }
// }
// pub type ResultOld<T> = std::result::Result<T, FixParsingError>;
// pub type Result<T> = std::result::Result<T, ParsingError>;
// #[derive(Debug, Clone)]
// pub struct FixToken {
//     tag: Tag,
//     idx_value_start: usize,
//     idx_value_end: usize,
// }
// #[derive(Clone)]
// pub struct Message {
//     buf: BytesMut,
//     index: Vec<FixToken>,
//     parsing_error: Option<ParsingError>,
// }
// impl Message {
//     const DELIMITER: u8 = 0x01;
//     const DELIMITER_AS_CHAR: char = Message::DELIMITER as char;
//     const DELIMITER_AS_PIPE: &'static str = "|";
//     const EQUAL_SIGN: u8 = b'=';
//     pub fn with_capacity(capacity: usize) -> Message {
//         Message {
//             buf: BytesMut::with_capacity(capacity),
//             index: Vec::new(),
//             parsing_error: None,
//         }
//     }
//     pub fn clear(&mut self) {
//         self.buf.clear();
//         self.index.clear();
//         self.parsing_error = None;
//     }
//     pub fn index(&mut self) -> Result<()> {
//         self.index.clear();
//         self.parsing_error = None;
//         // as_ref() to hopefully ensure that ExactSizeIterator is returned and collect only does allocation once
//         let slice = self.buf.as_ref();
//         let find_token_start = |idx: usize| {
//             for i in (0..idx).rev() {
//                 if slice[i] == Self::DELIMITER {
//                     return i + 1;
//                 }
//             }
//             0
//         };
//         let find_token_end = |idx: usize| {
//             for i in idx..slice.len() {
//                 if slice[i] == Self::DELIMITER {
//                     return i - 1;
//                 }
//             }
//             slice.len() - 1
//         };
//         let mut parsing_error_detected = None::<ParsingError>;
//         let index = slice
//             .iter()
//             .enumerate()
//             .filter(|(_, v)| **v == Self::EQUAL_SIGN)
//             .map(|(idx_equal_sign, _)| {
//                 let idx_token_start = find_token_start(idx_equal_sign);
//                 let tag_number = match unsafe { std::str::from_utf8_unchecked(&slice[idx_token_start..idx_equal_sign]) }.parse::<Tag>() {
//                     Ok(v) => v,
//                     Err(error) => {
//                         parsing_error_detected = Some(
//                             ParsingErrorKind::NotValidTagNumber {
//                                 idx_tag_start: idx_token_start,
//                                 idx_tag_end: idx_equal_sign,
//                                 error,
//                             }
//                             .into(),
//                         );
//                         0
//                     }
//                 };

//                 FixToken {
//                     tag: tag_number,
//                     idx_value_start: idx_equal_sign + 1,
//                     idx_value_end: find_token_end(idx_equal_sign),
//                 }
//             })
//             .collect::<Vec<_>>();
//         match parsing_error_detected {
//             Some(err) => {
//                 self.parsing_error = Some(err.clone());
//                 Err(err)
//             }
//             None => {
//                 self.index = index;
//                 self.index.sort_by(|a, b| a.tag.cmp(&b.tag));
//                 Ok(())
//             }
//         }
//     }

//     #[inline(always)]
//     fn is_first_byte_delimiter(&self, skip_from_start: usize) -> bool {
//         self.buf[0 + skip_from_start] == Self::DELIMITER
//     }
//     #[inline(always)]
//     fn is_last_byte_delimiter(&self, skip_from_end: usize) -> bool {
//         self.buf[self.buf.len() - 1 - skip_from_end] == Self::DELIMITER
//     }

//     #[inline(always)]
//     fn get_str(&self, token: &FixToken) -> &str {
//         let slice = &self.buf[token.idx_value_start + 1..token.idx_value_end + 1];
//         unsafe { std::str::from_utf8_unchecked(slice) }
//     }

//     // #[inline(always)]
//     // fn get_i64(&self, token: &FixToken) -> Result<i64> {
//     //     self.get_str(token).parse()?
//     // }
//     #[inline(always)]
//     fn to_string_section(&self, idx_start: usize, idx_end: usize) -> String {
//         let slice = &self.buf[idx_start..idx_end];
//         let value = unsafe { std::str::from_utf8_unchecked(slice) };
//         value.replace(Message::DELIMITER_AS_CHAR, Message::DELIMITER_AS_PIPE)
//     }
//     #[inline(always)]
//     fn is_tag_equal(&self, tag: Tag, idx_tag_start: usize, idx_tag_end: usize) -> bool {
//         let slice = &self.buf[idx_tag_start..idx_tag_end];
//         let value = unsafe { std::str::from_utf8_unchecked(slice) };
//         match value.parse::<Tag>() {
//             Ok(v) => v == tag,
//             Err(err) => {
//                 let start = self.to_string_section(0, idx_tag_start);
//                 let end = self.to_string_section(idx_tag_end, self.buf.len());
//                 warn!(
//                     "value: {:?} can't be parsed into TagNumber. {:?} '{}>>>>>{}<<<<<{}'",
//                     value, err, start, value, end
//                 );
//                 false
//             }
//         }
//     }
//     #[inline(always)]
//     fn find_tag_fwd(&self, tag: Tag, skip: Option<usize>) -> Option<FixToken> {
//         let skip = skip.unwrap_or(0);
//         let idx_last = self.buf.len() - 1;
//         let idx_first = 0 + skip;
//         let mut idx_token_start = match self.is_first_byte_delimiter(skip) {
//             true => idx_first + 1,
//             false => idx_first,
//         };
//         let mut idx_last_seen_delimiter: usize = idx_first;
//         let idx_last_seen_delimiter_overflow_correction = match self.is_first_byte_delimiter(skip) {
//             true => 0,
//             false => 1,
//         };

//         let mut idx_equal_sign: usize = idx_token_start;

//         for (idx_current, v) in self.buf.iter().enumerate().skip(skip) {
//             match *v {
//                 Self::DELIMITER => {
//                     // found new delimiter
//                     if idx_last_seen_delimiter < idx_current {
//                         idx_token_start = idx_last_seen_delimiter + 1 - idx_last_seen_delimiter_overflow_correction;
//                         let idx_token_end = idx_current - 1;
//                         idx_last_seen_delimiter = idx_current;

//                         if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
//                             if self.is_tag_equal(tag, idx_token_start, idx_equal_sign) {
//                                 return Some(FixToken {
//                                     tag: 0,
//                                     idx_value_start: idx_equal_sign,
//                                     idx_value_end: idx_token_end,
//                                 });
//                             }
//                         }
//                     }
//                 }
//                 _ => {
//                     // can't have a separate branch for EQ because it can be last byte in the buffer
//                     if *v == Self::EQUAL_SIGN {
//                         idx_equal_sign = idx_current;
//                     }
//                     // we reached last element and it does not not have delimiter at the end
//                     if idx_last == idx_current {
//                         // we reached the last byte in the buffer and it is not a delimiter or equal sign hence delimiter START is now whatever happens to be delimiter END
//                         idx_token_start = idx_last_seen_delimiter + 1 - idx_last_seen_delimiter_overflow_correction;
//                         let idx_token_end = idx_current;

//                         if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
//                             if self.is_tag_equal(tag, idx_token_start, idx_equal_sign) {
//                                 return Some(FixToken {
//                                     tag: 0,
//                                     idx_value_start: idx_equal_sign,
//                                     idx_value_end: idx_token_end,
//                                 });
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         None
//     }

//     #[inline(always)]
//     fn find_rev(&self, tag: Tag, skip: Option<usize>) -> Option<&[u8]> {
//         let tag = TagStrWithEqSign::from(tag);
//         let skip = skip.unwrap_or(0);
//         let search = tag.as_ref();
//         let mut idx_last_seen_delimiter = self.buf.len()
//             - match self.is_last_byte_delimiter(skip) {
//                 true => 1 - skip,
//                 false => skip,
//             };
//         for i in (0..self.buf.len() - search.len() - skip).rev() {
//             if self.buf[i] == Self::DELIMITER {
//                 idx_last_seen_delimiter = i;
//             } else if self.buf[i + search.len() - 1] == Self::EQUAL_SIGN {
//                 let test = &self.buf[i..i + search.len()];
//                 if test == search {
//                     return Some(&self.buf[i + search.len()..idx_last_seen_delimiter]);
//                 }
//             }
//         }
//         None
//     }
//     #[inline(always)]
//     pub fn find_tag_rev(&self, tag: Tag, skip: Option<usize>) -> Option<FixToken> {
//         let skip = skip.unwrap_or(0);
//         let idx_last = self.buf.len() - 1 - skip;
//         let idx_first = 0;

//         let mut idx_last_seen_delimiter = match self.is_last_byte_delimiter(skip) {
//             true => idx_last,
//             false => idx_last + 1,
//         };
//         let mut idx_token_end = match self.is_last_byte_delimiter(skip) {
//             true => idx_last - 1,
//             false => idx_last,
//         };
//         let mut idx_equal_sign: usize = idx_token_end;

//         // enumerate in reverse idx will be [4, 3, 2, 1, 0] until found a matching tag
//         for (idx_current, v) in self.buf.iter().enumerate().rev().skip(skip) {
//             match *v {
//                 Self::DELIMITER => {
//                     // found new delimiter
//                     if idx_last_seen_delimiter > idx_current {
//                         let idx_token_start = idx_current + 1;
//                         idx_token_end = idx_last_seen_delimiter - 1;
//                         idx_last_seen_delimiter = idx_current;
//                         if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
//                             if self.is_tag_equal(tag, idx_token_start, idx_equal_sign) {
//                                 return Some(FixToken {
//                                     tag: 0,
//                                     idx_value_start: idx_equal_sign,
//                                     idx_value_end: idx_token_end,
//                                 });
//                             }
//                         }
//                     }
//                 }

//                 _ => {
//                     // can't have a separate branch for EQ because it can be first byte in the buffer
//                     if *v == Self::EQUAL_SIGN {
//                         idx_equal_sign = idx_current;
//                     }
//                     // we reached first element in reverse and it does not have a delimiter in front
//                     if idx_current == idx_first {
//                         let idx_token_start = idx_current;
//                         idx_token_end = idx_last_seen_delimiter - 1;
//                         // we reached the first byte in the buffer and it is not a delimiter or equal sign hence delimiter END is now whatever happens to be delimiter START
//                         if idx_token_start < idx_equal_sign && idx_equal_sign <= idx_token_end {
//                             if self.is_tag_equal(tag, idx_first, idx_equal_sign) {
//                                 return Some(FixToken {
//                                     tag: 0,
//                                     idx_value_start: idx_equal_sign,
//                                     idx_value_end: idx_token_end,
//                                 });
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         None
//     }
//     pub fn write_slice(&mut self, slice: &[u8]) {
//         self.buf.put_slice(slice);
//     }
// }
// impl MessageWriter for Message {
//     fn write_str<S: AsRef<str>>(&mut self, tag: Tag, value: S) {
//         if self.buf.len() > 0 {
//             self.buf.put_u8(Self::DELIMITER);
//         }
//         crate::macros::start_field!(self.buf, tag);
//         self.buf.put_u8(Self::EQUAL_SIGN);
//         self.buf.put_slice(value.as_ref().as_bytes());
//     }
//     fn write_i64(&mut self, tag: Tag, value: i64) {
//         if self.buf.len() > 0 {
//             self.buf.put_u8(Self::DELIMITER);
//         }
//         crate::macros::start_field!(self.buf, tag);
//         self.buf.put_u8(Self::EQUAL_SIGN);
//         crate::macros::start_field!(self.buf, value);
//     }
// }
// impl MessageReader for Message {
//     #[inline(always)]
//     fn read_str_dir(&self, tag: Tag, reverse: bool) -> Option<&str> {
//         let token = match reverse {
//             true => self.find_tag_rev(tag, None),
//             false => self.find_tag_fwd(tag, None),
//         };
//         Some(self.get_str(&token?))
//     }
//     #[inline(always)]
//     fn read_str_1(&self, tag: Tag) -> Option<&str> {
//         if !self.index.is_empty() && self.parsing_error.is_none() {
//             match self.index.binary_search_by(|probe| probe.tag.cmp(&tag)) {
//                 Ok(idx) => {
//                     let token = &self.index[idx];
//                     let slice = &self.buf[token.idx_value_start..=token.idx_value_end];
//                     Some(unsafe { std::str::from_utf8_unchecked(slice) })
//                     // Some(std::str::from_utf8(slice))
//                 }
//                 Err(_) => None,
//             }
//         } else {
//             None
//         }
//     }
//     // #[inline(always)]
//     // fn read_i64_dir(&self, tag: TagNumber, reverse: bool) -> Result<Option<i64>> {
//     //     let token = match reverse {
//     //         true => self.find_tag_rev(tag, None),
//     //         false => self.find_tag_fwd(tag, None),
//     //     };
//     //     Some(self.get_i64(&token?)?)
//     // }
// }
// impl Display for Message {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.parsing_error {
//             Some(ParsingError(ParsingErrorKind::NotValidTagNumber {
//                 idx_tag_start,
//                 idx_tag_end,
//                 error,
//             })) => {
//                 write!(
//                     f,
//                     "{} {}>>>>>{}<<<<<{}",
//                     error,
//                     self.to_string_section(0, *idx_tag_start),
//                     self.to_string_section(*idx_tag_start, idx_tag_end + 1),
//                     self.to_string_section(idx_tag_end + 1, self.buf.len())
//                 )
//             }
//             None => self.to_string_section(0, self.buf.len()).fmt(f),
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::utils::ValueStr;

//     use super::*;
//     use crate::unittest::setup;
//     use log::info;

//     #[test]
//     fn test_index() -> Result<()> {
//         setup::log::configure_compact(log::LevelFilter::Info);
//         let sparkle_heart = vec![0, 159, 146, 150];
//         let s = unsafe { std::str::from_utf8_unchecked(&sparkle_heart) };
//         info!("s: '{:?}'", s);
//         return Ok(());
//         let mut msg = setup::model::get_large_fix_message();
//         info!("msg: '{}'", msg);
//         let res = msg.index();
//         info!("msg: '{}'", msg);
//         for tag in 1..=100 {
//             let value = msg.read_str_1(tag);
//             info!("tag: {}, value: {:?}", tag, value);
//             assert!(value.is_some());
//             assert_eq!(value.unwrap().len(), tag as usize);
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_find_tag_missing() -> ResultOld<()> {
//         setup::log::configure_compact(log::LevelFilter::Info);
//         let msg_valid = vec![
//             b"0".as_slice(),
//             b"0\x01".as_slice(),
//             b"\x010".as_slice(),
//             b"\x010\x01".as_slice(),
//             b"=".as_slice(),
//             b"\x01=".as_slice(),
//             b"=\x01".as_slice(),
//             b"\x01=\x01".as_slice(),
//             b"=0".as_slice(),
//             b"=0\x01".as_slice(),
//             b"\x01=0".as_slice(),
//             b"\x01=0\x01".as_slice(),
//             b"gibberish=".as_slice(),
//             b"\x01gibberish=".as_slice(),
//             b"\x01gibberish=".as_slice(),
//             b"\x01gibberish=\x01".as_slice(),
//         ];
//         for m in msg_valid {
//             let mut msg = Message::with_capacity(1024);
//             msg.write_slice(m);
//             info!("*********************************");
//             info!("msg: '{}'", msg);

//             let search_tag = 0;
//             // rev
//             let token = msg.find_tag_rev(search_tag, None);
//             info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_none());
//             // fwd
//             let token = msg.find_tag_fwd(search_tag, None);
//             info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_none());
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_find_tag_valid_not_empty() -> ResultOld<()> {
//         setup::log::configure_compact(log::LevelFilter::Info);
//         let msg_valid = vec![
//             b"0=2345".as_slice(),
//             b"\x010=2345\x01".as_slice(),
//             b"\x010=2345".as_slice(),
//             b"0=2345\x01".as_slice(),
//         ];
//         let as_str_expected = "2345";
//         // let as_str_expected = b"2345".as_slice();
//         for m in msg_valid {
//             let mut msg = Message::with_capacity(1024);
//             msg.write_slice(m);
//             info!("*********************************");
//             info!("msg: '{}'", msg);

//             let search_tag = 0;

//             // // rev
//             // let token = msg.find_tag_rev(search_tag, None);
//             // info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
//             // assert!(token.is_some());
//             // let as_str_actual = msg.get_str(&token.unwrap());
//             // info!("as_str_actual: {:?}", as_str_actual);
//             // assert_eq!(as_str_expected, as_str_actual);
//             // // fwd
//             // let token = msg.find_tag_fwd(search_tag, None);
//             // info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
//             // assert!(token.is_some());
//             // let as_str_actual = msg.get_str(&token.unwrap());
//             // info!("as_str_actual: {:?}", as_str_actual);
//             // assert_eq!(as_str_expected, as_str_actual);

//             // let search_tag = 1;
//             // let token = msg.find_tag_rev(search_tag, None);
//             // info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
//             // assert!(token.is_none());

//             // let token = msg.find_tag_fwd(search_tag, None);
//             // info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
//             // assert!(token.is_none());

//             // rev

//             let slice = msg.find_rev(search_tag, None);
//             assert!(slice.is_some());
//             let value = ValueStr::from(slice.unwrap());
//             info!("value: {:?}", value.as_str());
//             assert_eq!(as_str_expected, value.as_str());
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_find_tag_valid_empty() -> ResultOld<()> {
//         setup::log::configure_compact(log::LevelFilter::Info);
//         let msg_valid = vec![b"0=".as_slice(), b"\x010=\x01".as_slice(), b"\x010=".as_slice(), b"0=\x01".as_slice()];
//         let as_str_expected = "";
//         for m in msg_valid {
//             let mut msg = Message::with_capacity(1024);
//             msg.write_slice(m);
//             info!("*********************************");
//             info!("msg: '{}'", msg);

//             let search_tag = 0;
//             // rev
//             let token = msg.find_tag_rev(search_tag, None);
//             info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_some());
//             let as_str_actual = msg.get_str(&token.unwrap());
//             info!("as_str_actual: {:?}", as_str_actual);
//             assert_eq!(as_str_expected, as_str_actual);

//             // fwd
//             let token = msg.find_tag_fwd(search_tag, None);
//             info!("find_tag_fwd <- search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_some());
//             let as_str_actual = msg.get_str(&token.unwrap());
//             info!("as_str_actual: {:?}", as_str_actual);
//             assert_eq!(as_str_expected, as_str_actual);

//             let search_tag = 1;
//             let token = msg.find_tag_rev(search_tag, None);
//             info!("find_tag_rev <- search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_none());

//             let token = msg.find_tag_fwd(search_tag, None);
//             info!("find_tag_fwd -> search_tag: {} {:?}", search_tag, token);
//             assert!(token.is_none());
//         }

//         Ok(())
//     }

//     #[test]
//     fn test_write_read_str() {
//         setup::log::configure_compact(log::LevelFilter::Info);

//         let mut msg = Message::with_capacity(1024);

//         let tag = 1;
//         msg.write_str(tag, "1");
//         assert_eq!(format!("{}", msg), "1=1");
//         msg.write_str(tag, "2");
//         assert_eq!(format!("{}", msg), "1=1|1=2");
//         msg.write_str(tag, "3");
//         assert_eq!(format!("{}", msg), "1=1|1=2|1=3");

//         info!("msg: '{}'", msg);

//         let value = msg.read_str_dir(tag, true).unwrap();
//         info!("tag: {}, value: {}", tag, value);
//         assert_eq!(value, "3");

//         let value = msg.read_str_dir(tag, false).unwrap();
//         info!("tag: {}, value: {}", tag, value);
//         assert_eq!(value, "1");

//         let tag = Tag::MAX;
//         msg.write_str(tag, "1");
//         info!("msg: '{}'", msg);
//         assert_eq!(format!("{}", msg), "1=1|1=2|1=3|4294967295=1");
//     }

//     #[test]
//     fn test_scratchpad(){
//         setup::log::configure();
//         // let a = 12_i8;
//         // let b = -12_i8;
//         // let c = 0_i8;
//         // info!("{:08b} <- a", a);
//         // info!("{:08b} <- b", b);
//         // info!("{:08b} <-!a", !a);
//         // info!("{:08b} <-!a+1", !a+1);
//         // info!("{:08b} <- c", c);

//         let target: u16 = 60;
//         let alignment = 32;
//         let mask = alignment - 1;

//         info!("{:016b} {} <- alignment", alignment, alignment);
//         info!("{:016b} {} <- mask -> alignment - 1", mask, mask);
//         info!("{:016b} {} <- target", target, target);
//         info!("{:016b} {} <- target + mask", target + mask , target + mask);
//         info!("{:016b} {} <- !mask ", !mask, !mask);
//         info!("{:016b} {} <- aligned -> target + mask & !mask ", (target + mask) & !mask, (target + mask) & !mask);
//         let aligned = (target + mask) & !mask;
//         info!("aligned: {}", aligned);

//     }

//     #[test]
//     fn test_scratchpad1(){
//         setup::log::configure();
//         let mut header = BytesMut::with_capacity(1024);
//         // info!("header: {:?}, len/capacity {}/{}", header, header.len(), header.capacity());
//         // header.extend_from_slice(b"8=FIX\x019=0\x01");
//         // info!("header: {:?}, len/capacity {}/{}", header, header.len(), header.capacity());
//         // let mut body = header.split_off(6);

//         let mut body = header.split_off(10);
//         header.extend_from_slice(b"8=FIX\x01");
//         body.extend_from_slice(b"9=01\x01");

//         info!("header: {:?}, len/capacity {}/{}", header, header.len(), header.capacity());
//         info!("body: {:?} len/capacity {}/{}", body, body.len(), body.capacity());

//         // body.unsplit(header);
//         // info!("body: {:?}, len/capacity {}/{}", body, body.len(), body.capacity());
//         header.unsplit(body);
//         info!("header: {:?}, len/capacity {}/{}", header, header.len(), header.capacity());

//     }
// }
