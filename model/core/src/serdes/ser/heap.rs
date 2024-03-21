use crate::prelude::{Field, Tag, Value};
use super::{Serialize, Serializer};
use std::fmt::{Debug, Display};

use crate::macros::asserted_short_name;
use bytes::{BufMut, BytesMut};

pub struct Header<'h>(&'h BytesMut);
impl Display for Header<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = String::from_utf8_lossy(self.0).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        write!(f, "{}", header)
    }
}
pub struct Body<'b>(&'b mut BytesMut);
impl<'b> Serializer for Body<'b> {
    #[inline(always)]
    fn serialize_slice(&mut self, value: &[u8]) {
        self.0.put_slice(value);
    }
    #[inline(always)]
    fn serialize_u8(&mut self, value: u8) {
        self.0.put_u8(value);
    }
    #[inline(always)]
    fn serialize_eqs(&mut self) {
        self.0.put_u8(crate::serdes::EQS_U8);
    }
    #[inline(always)]
    fn serialize_soh(&mut self) {
        self.0.put_u8(crate::serdes::SOH_U8);
    }
    #[inline(always)]
    fn serialize_tag_value(&mut self, tag: Tag, value: impl Value) {
        tag.serialize(self);
        value.serialize(self);
    }

    #[inline(always)]
    fn serialize_field(&mut self, field: &impl Field) {
        field.serialize(self);
    }
    #[inline(always)]
    fn serialize_value(&mut self, value: impl Value) {
        value.serialize(self)
    }
}

impl Display for Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = String::from_utf8_lossy(self.0).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        write!(f, "{}", body)
    }
}
pub struct Trailer<'t>(&'t BytesMut);
impl Display for Trailer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trailer = String::from_utf8_lossy(self.0).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        write!(f, "{}", trailer)
    }
}

pub struct HeapSerializer {
    header: BytesMut,
    body: BytesMut,
    trailer: BytesMut,
}
impl HeapSerializer {
    // TODO make these constants configurable or pick correct defaults
    const HEADER_CAPACITY: usize = 50;
    const TRAILER_CAPACITY: usize = 50;
    pub fn with_capacity(capacity: usize) -> Self {
        let mut header = BytesMut::with_capacity(capacity + Self::HEADER_CAPACITY + Self::TRAILER_CAPACITY);
        let mut body = header.split_off(Self::HEADER_CAPACITY);
        let trailer = body.split_off(body.capacity() - Self::TRAILER_CAPACITY);
        Self { header, body, trailer }
    }
    #[inline(always)]
    pub fn header(&self) -> Header {
        Header(&self.header)
    }
    #[inline(always)]
    pub fn body(&mut self) -> Body {
        Body(&mut self.body)
    }
    #[inline(always)]
    pub fn trailer(&self) -> Trailer {
        Trailer(&self.trailer)
    }
}
impl Serializer for HeapSerializer {
    #[inline(always)]
    fn serialize_slice(&mut self, value: &[u8]) {
        self.body().serialize_slice(value);
    }
    #[inline(always)]
    fn serialize_u8(&mut self, value: u8) {
        self.body().serialize_u8(value);
    }
    #[inline(always)]
    fn serialize_eqs(&mut self) {
        self.body().serialize_eqs();
    }
    #[inline(always)]
    fn serialize_soh(&mut self) {
        self.body().serialize_soh();
    }
    #[inline(always)]
    fn serialize_tag_value(&mut self, tag: Tag, value: impl Value) {
        self.body().serialize_tag_value(tag, value)
    }
    #[inline(always)]
    fn serialize_field(&mut self, field: &impl Field) {
        self.body().serialize_field(field)
    }
    #[inline(always)]
    fn serialize_value(&mut self, value: impl Value) {
        self.body().serialize_value(value)
    }
}
impl Debug for HeapSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(asserted_short_name!("HeapSerializer", Self))
            .field("header_len", &self.header.len())
            .field("header_capacity", &self.header.capacity())
            .field("header", &self.header)
            .field("body_len", &self.body.len())
            .field("body_capacity", &self.body.capacity())
            .field("body", &self.body)
            .field("trailer_len", &self.trailer.len())
            .field("trailer_capacity", &self.trailer.capacity())
            .field("trailer", &self.trailer)
            .finish()
    }
}
impl Display for HeapSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = String::from_utf8_lossy(&self.header).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        let body = String::from_utf8_lossy(&self.body).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        let trailer = String::from_utf8_lossy(&self.trailer).replace(crate::serdes::SOH_CHAR, crate::serdes::PIPE_STR);
        write!(f, "header: '{}', body: '{}', trailer: '{}'", header, body, trailer)
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use crate::prelude::*;
    use crate::unittest::setup;
    #[test]
    fn test_write_string() {
        setup::log::configure();
        let mut msg = HeapSerializer::with_capacity(1024);

        let v = "str";
        msg.serialize_tag_value(1.into(), v);
        let v = "string".to_string();
        msg.serialize_tag_value(2.into(), v);

        info!("msg: {:?}", msg);
        info!("msg: {}", msg);
        assert_eq!(msg.body().to_string(), String::from("1=str|2=string|"));
    }

    #[test]
    fn test_write_char() {
        setup::log::configure();
        let mut msg = super::HeapSerializer::with_capacity(1024);

        let v = 'a';
        msg.serialize_tag_value(1.into(), v);

        info!("msg: {:?}", msg);
        info!("msg: {}", msg);
        assert_eq!(msg.body().to_string(), String::from("1=a|"));
    }

    #[test]
    fn test_write_inumber() {
        setup::log::configure();
        let mut msg = super::HeapSerializer::with_capacity(1024);

        let v = i8::MIN;
        msg.serialize_tag_value(1.into(), v);
        let v = i8::MAX;
        msg.serialize_tag_value(2.into(), v);

        let v = i16::MIN;
        msg.serialize_tag_value(3.into(), v);
        let v = i16::MAX;
        msg.serialize_tag_value(4.into(), v);

        let v = i32::MIN;
        msg.serialize_tag_value(5.into(), v);
        let v = i32::MAX;
        msg.serialize_tag_value(6.into(), v);

        let v = i64::MIN;
        msg.serialize_tag_value(7.into(), v);
        let v = i64::MAX;
        msg.serialize_tag_value(8.into(), v);

        let v = isize::MIN;
        msg.serialize_tag_value(9.into(), v);
        let v = isize::MAX;
        msg.serialize_tag_value(10.into(), v);

        let v = i128::MIN;
        msg.serialize_tag_value(11.into(), v);
        let v = i128::MAX;
        msg.serialize_tag_value(12.into(), v);

        info!("msg: {:?}", msg);
        info!("msg: {}", msg);

        assert_eq!(msg.body().to_string(), String::from("1=-128|2=127|3=-32768|4=32767|5=-2147483648|6=2147483647|7=-9223372036854775808|8=9223372036854775807|9=-9223372036854775808|10=9223372036854775807|11=-170141183460469231731687303715884105728|12=170141183460469231731687303715884105727|"));
    }

    #[test]
    fn test_write_unumber() {
        setup::log::configure();
        let mut msg = super::HeapSerializer::with_capacity(1024);

        let v = u8::MIN;
        msg.serialize_tag_value(1.into(), v);
        let v = u8::MAX;
        msg.serialize_tag_value(2.into(), v);

        let v = u16::MIN;
        msg.serialize_tag_value(3.into(), v);
        let v = u16::MAX;
        msg.serialize_tag_value(4.into(), v);

        let v = u32::MIN;
        msg.serialize_tag_value(5.into(), v);
        let v = u32::MAX;
        msg.serialize_tag_value(6.into(), v);

        let v = u64::MIN;
        msg.serialize_tag_value(7.into(), v);
        let v = u64::MAX;
        msg.serialize_tag_value(8.into(), v);

        let v = usize::MIN;
        msg.serialize_tag_value(9.into(), v);
        let v = usize::MAX;
        msg.serialize_tag_value(10.into(), v);

        let v = u128::MIN;
        msg.serialize_tag_value(11.into(), v);
        let v = u128::MAX;
        msg.serialize_tag_value(12.into(), v);

        info!("msg: {:?}", msg);
        info!("msg: {}", msg);

        assert_eq!(
            msg.body().to_string(),
            String::from("1=0|2=255|3=0|4=65535|5=0|6=4294967295|7=0|8=18446744073709551615|9=0|10=18446744073709551615|11=0|12=340282366920938463463374607431768211455|")
        );
    }

    #[test]
    fn test_write_fnumber() {
        setup::log::configure();
        let mut msg = super::HeapSerializer::with_capacity(1024);

        let v = f32::MIN;
        msg.serialize_tag_value(1.into(), v);
        let v = f32::MAX;
        msg.serialize_tag_value(2.into(), v);
        let v = 1.0 / 3.0_f32;
        msg.serialize_tag_value(3.into(), v);

        let v = f64::MIN;
        msg.serialize_tag_value(4.into(), v);
        let v = f64::MAX;
        msg.serialize_tag_value(5.into(), v);
        let v = 1.0 / 3.0_f64;
        msg.serialize_tag_value(6.into(), v);

        info!("msg: {:?}", msg);
        info!("msg: {}", msg);

        assert_eq!(
            msg.body().to_string(),
            String::from("1=-340282350000000000000000000000000000000|2=340282350000000000000000000000000000000|3=0.33333334|4=-179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000|5=179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000|6=0.3333333333333333|")
        );
    }
}
