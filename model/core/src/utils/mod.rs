// use std::{error::Error, fmt::Display, io::Write, num::ParseIntError, u32};

// use crate::prelude::Tag;

// #[derive(Debug, Clone)]
// pub enum ParsingErrorKind {
//     NotValidTagNumber {
//         idx_tag_start: usize,
//         idx_tag_end: usize,
//         error: ParseIntError,
//     },
// }
// #[derive(Debug, Clone)]
// pub struct ParsingError(pub(crate) ParsingErrorKind);

// impl From<ParsingErrorKind> for ParsingError {
//     fn from(kind: ParsingErrorKind) -> Self {
//         ParsingError(kind)
//     }
// }
// impl Display for ParsingError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match &self.0 {
//             ParsingErrorKind::NotValidTagNumber {
//                 idx_tag_start,
//                 idx_tag_end,
//                 error,
//             } => {
//                 write!(f, "NotValidTagNumber, start: {}, end: {}, error: {}", idx_tag_start, idx_tag_end, error)
//             }
//         }
//     }
// }
// impl Error for ParsingError {}

// // u32::MAX = 4294967295 = 10 digits
// // 11 digits to include '=' char
// const U32_MAX_LEN_WITH_DELIM: usize = 11;
// pub(crate) struct TagStrWithEqSign {
//     buf: [u8; U32_MAX_LEN_WITH_DELIM],
//     end: usize,
// }
// impl From<u32> for TagStrWithEqSign {
//     fn from(tag: u32) -> Self {
//         let mut buf = [b'\0'; U32_MAX_LEN_WITH_DELIM];
//         write!(&mut buf[..], "{}=", tag).expect("TagStrWithEqSign, failed to write to buffer as str");
//         let end = buf.iter().position(|v| *v == b'\0').unwrap_or(U32_MAX_LEN_WITH_DELIM);
//         TagStrWithEqSign { buf, end }
//     }
// }
// impl AsRef<[u8]> for TagStrWithEqSign {
//     #[inline(always)]
//     fn as_ref(&self) -> &[u8] {
//         &self.buf[..self.end]
//     }
// }
// impl Display for TagStrWithEqSign {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(std::str::from_utf8(self.as_ref()).expect("TagStrWithEqSign, failed to convert to str for Display"))
//     }
// }

// pub struct ValueStr<'a>(&'a [u8]);
// impl<'a> ValueStr<'a> {
//     #[inline(always)]
//     pub fn as_str(&self) -> &str {
//         std::str::from_utf8(self.0).expect("failed to convert &[u8] to &str")
//     }
// }
// impl<'a> AsRef<str> for ValueStr<'a> {
//     #[inline(always)]
//     fn as_ref(&self) -> &str {
//         std::str::from_utf8(self.0).expect("failed to convert &[u8] to &str")
//     }
// }
// impl<'a> From<&'a [u8]> for ValueStr<'a> {
//     #[inline(always)]
//     fn from(v: &'a [u8]) -> Self {
//         ValueStr(v)
//     }
// }
// impl<'a> Display for ValueStr<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(std::str::from_utf8(self.0).expect("failed to convert &[u8] to &str"))
//     }
// }

// pub trait MessageWriter {
//     fn write_str<S: AsRef<str>>(&mut self, tag: Tag, value: S);
//     fn write_i64(&mut self, tag: Tag, value: i64);
// }
// pub trait MessageReader {
//     fn read_str_dir(&self, tag: Tag, reverse: bool) -> Option<&str>;
//     #[inline(always)]
//     fn read_str(&self, tag: Tag) -> Option<&str> {
//         self.read_str_dir(tag, true)
//     }
//     #[inline(always)]
//     fn read_str_rev(&self, tag: Tag) -> Option<&str> {
//         self.read_str_dir(tag, true)
//     }
//     #[inline(always)]
//     fn read_str_fwd(&self, tag: Tag) -> Option<&str> {
//         self.read_str_dir(tag, false)
//     }

//     fn read_str_1(&self, tag: Tag) -> Option<&str>;
// }

// #[cfg(test)]
// mod tests {
//     use log::info;

//     use crate::unittest::setup;

//     use super::*;

//     #[test]
//     fn tag_str_from_u32() {
//         setup::log::configure();
//         let tag_str = TagStrWithEqSign::from(u32::MIN);
//         info!("tag_str: '{}'", tag_str);
//         assert_eq!(tag_str.as_ref(), b"0=".as_slice());

//         let tag_str = TagStrWithEqSign::from(u32::MAX);
//         info!("tag_str: '{}'", tag_str);
//         assert_eq!(tag_str.as_ref(), b"4294967295=".as_slice());
//     }
// }
