use bytes::Bytes;

use crate::prelude::FixStr;

use super::{EQS_U8, SOH_U8};

pub struct Deserializer {
    buf: Bytes,
}
impl Deserializer {
    pub fn iter(&self) -> DeserializerIterator {
        DeserializerIterator {
            buf: &self.buf,
            cur_idx: 0,
            beg_idx: 0,
            eqs_idx: 0,
        }
    }
}

pub struct DeserializerIterator<'b> {
    buf: &'b Bytes,
    cur_idx: usize,
    beg_idx: usize,
    eqs_idx: usize,
}
impl<'b> Iterator for DeserializerIterator<'b> {
    type Item = (&'b FixStr, &'b FixStr);
    fn next(&mut self) -> Option<Self::Item> {
        while self.buf.len() != self.cur_idx {
            // TODO handle length & data fields by skipping of reading each byte
            match self.buf[self.cur_idx] {
                SOH_U8 => {
                    let tag = &self.buf[self.beg_idx..self.eqs_idx];
                    let value = &self.buf[self.eqs_idx + 1..self.cur_idx];
                    self.cur_idx += 1;
                    self.eqs_idx = self.cur_idx;
                    self.beg_idx = self.cur_idx;
                    // let as_tag = FixStr::from_ascii(tag).as_tag();
                    return Some((FixStr::from_ascii(tag), FixStr::from_ascii(value)));
                }
                EQS_U8 => {
                    self.eqs_idx = self.cur_idx;
                    self.cur_idx += 1;
                }
                _ => {
                    self.cur_idx += 1;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use super::*;
    use crate::{prelude::*, unittest::setup};
    #[test]
    fn test_iter() {
        setup::log::configure();
        let bytes = Bytes::copy_from_slice(b"8=FIX.4.4\x019=12\x0135=0\x0134=1\x0149=BRKR\x0156=INVMGR\x0152=20201123-19:38:00.000\x0110=220\x01");
        let des = super::Deserializer { buf: bytes };
        for (tag, value) in des.iter() {
            info!("tag: {:?}, value: {:?}", tag, value);

        }
        let mut iter = des.iter();
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"8"), FixStr::from_ascii(b"FIX.4.4")));
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"9"), FixStr::from_ascii(b"12")));
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"35"), FixStr::from_ascii(b"0")));
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"34"), FixStr::from_ascii(b"1")));
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"49"), FixStr::from_ascii(b"BRKR")));
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"56"), FixStr::from_ascii(b"INVMGR")));
        assert_eq!(
            iter.next().unwrap(),
            (FixStr::from_ascii(b"52"), FixStr::from_ascii(b"20201123-19:38:00.000"))
        );
        assert_eq!(iter.next().unwrap(), (FixStr::from_ascii(b"10"), FixStr::from_ascii(b"220")));
        assert_eq!(iter.next(), None);
        
    }
}
