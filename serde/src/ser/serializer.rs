use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use bytes::BytesMut;
use serde::{ser, Serialize};
// use serde_json::value::Serializer;
use super::write::{BytesWrite, Write};

use crate::error::{Error, Result};

pub struct Serializer<W> {
    write: W,
}
impl<W: Write> Serializer<W> {
    pub fn new(write: W) -> Self {
        Self { write }
    }
    #[inline(always)]
    pub fn serialize_soh(&mut self) -> Result<()> {
        self.write.write_soh()
    }
    #[inline(always)]
    pub fn serialize_eqs(&mut self) -> Result<()> {
        self.write.write_eqs()
    }
    pub fn into_inner(self) -> W {
        self.write
    }
}
impl From<Serializer<BytesWrite>> for BytesMut {
    fn from(serializer: Serializer<BytesWrite>) -> Self {
        serializer.write.into()
    }
}
impl Serializer<BytesWrite> {
    pub fn from_bytes(bytes: BytesMut) -> Self {
        Self::new(BytesWrite::new(bytes))
    }
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}
impl Deref for Serializer<BytesWrite> {
    type Target = BytesWrite;
    fn deref(&self) -> &Self::Target {
        &self.write
    }
}
fn from_trait<W: Write, T: ser::Serialize>(write: W, value: T) -> Result<Serializer<W>> {
    let mut serializer = Serializer::new(write);
    value.serialize(&mut serializer)?;
    Ok(serializer)
}

pub fn to_bytes<T: ser::Serialize>(value: &T) -> Result<Serializer<BytesWrite>> {
    let write = BytesWrite::new(BytesMut::new());
    let ser = from_trait(write, value);
    ser

}
// pub struct Serializer1 {
//     header: BytesMut,
//     pub body: BytesMut,
//     trailer: BytesMut,
// }
// impl Serializer {
// TODO make these constants configurable or pick correct defaults
// const HEADER_CAPACITY: usize = 50;
// const TRAILER_CAPACITY: usize = 50;
// pub fn with_capacity(capacity: usize) -> Self {
//     let mut header = BytesMut::with_capacity(capacity + Self::HEADER_CAPACITY + Self::TRAILER_CAPACITY);
//     let mut body = header.split_off(Self::HEADER_CAPACITY);
//     let trailer = body.split_off(body.capacity() - Self::TRAILER_CAPACITY);
//     Self { header, body, trailer }
// }
// pub fn serialize_soh(&mut self) -> Result<()> {
//     self.body.put_u8(crate::SOH);
//     Ok(())
// }
// }
impl<W: Write> Display for Serializer<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write.fmt(f)
    }
}
impl<W: Write> Debug for Serializer<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write.fmt(f)
    }
}

impl<W: Write> ser::Serializer for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write.write_slice(v.to_string().as_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write.write_slice(v.to_string().as_bytes());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write.write_slice(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        todo!()
    }

    fn serialize_none(self) -> Result<()> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<()> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<()> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<W: Write> ser::SerializeSeq for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<W: Write> ser::SerializeTuple for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<W: Write> ser::SerializeTupleStruct for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<W: Write> ser::SerializeTupleVariant for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<W: Write> ser::SerializeMap for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<W: Write> ser::SerializeStruct for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.write.write_slice(key.as_bytes())?;
        self.write.write_u8(crate::EQS)?;
        value.serialize(&mut **self)?;
        self.write.write_u8(crate::SOH)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<W: Write> ser::SerializeStructVariant for &mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// pub fn to_bytes(value: &impl Serialize) -> Result<Serializer1> {
//     let mut ser = Serializer1::with_capacity(1024);
//     value.serialize(&mut ser)?;
//     Ok(ser)
// }

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {
    use crate::unittest::setup;

    use super::*;
    use crate::prelude::{FixStr, FixString, FixStringLike, Tag};
    use log::info;
    use serde::Deserialize;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_tag_serialize() {
        setup::log::configure();
        let tag = Tag::new(1);
        let fix = to_bytes(&tag).unwrap();
        info!("fix: {}", fix);
        let jsn = to_string(&tag).unwrap();
        info!("jsn: {}", jsn);
        assert_eq!(jsn, fix.to_string());
    }

    #[test]
    fn test_fix_string_serialize() {
        setup::log::configure();
        let str = FixString::try_from(b"ABC").unwrap();
        let fix = to_bytes(&str).unwrap();
        info!("fix: {}", fix);
        let jsn = to_string(&str).unwrap();
        info!("jsn: {}", jsn);
        assert_eq!(jsn, format!("\"{}\"", fix.to_string()));
    }

    #[test]
    fn test_msg_serialize() {
        setup::log::configure();

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Account<T: FixStringLike>(T);
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct AdvId<T: FixStringLike>(T);
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct BeginSeqNo(usize);

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Msg<T: FixStringLike> {
            #[serde(rename = "1")]
            #[serde(alias = "Account")]
            account: Account<T>,
            #[serde(rename = "2")]
            #[serde(alias = "AdvId")]
            adv_id: AdvId<T>,
            #[serde(rename = "7")]
            #[serde(alias = "BeginSeqNo")]
            begin_seq_no: BeginSeqNo,
        }

        let account = Account(b"ABC".try_into().unwrap());
        let adv_id = AdvId(b"DEF".try_into().unwrap());
        let begin_seq_no = BeginSeqNo(100);
        let msg = Msg::<&FixStr> {
            account,
            adv_id,
            begin_seq_no,
        };
        // let msg = Msg::<FixString> { account, adv_id };
        // let msg = msg.to_owned();
        let fix = to_bytes(&msg).unwrap();
        info!("fix: {}", fix);
        let jsn = to_string(&msg).unwrap();
        info!("jsn: {}", jsn);
        let msg: Msg<FixString> = from_str(&jsn).unwrap();
        info!("msg: {:?}", msg);

        let jsn = r#" { "2":"DEF", "7": 100, "1":"ABC" } "#;
        let msg: Msg<FixString> = from_str(&jsn).unwrap();
        info!("msg: {:?}", msg);
    }
}
