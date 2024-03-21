use std::fmt::{Debug, Display};

use bytes::{BufMut, BytesMut};
use serde::{ser, Serialize};

use super::error::{Error, Result};

pub struct BytesSerializer<'s>(&'s BytesMut);

pub struct HeapSerializer {
    header: BytesMut,
    pub body: BytesMut, 
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
}
impl Display for HeapSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = String::from_utf8_lossy(&self.header).replace(crate::SOH_CHAR, crate::PIPE_STR);
        let body = String::from_utf8_lossy(&self.body).replace(crate::SOH_CHAR, crate::PIPE_STR);
        let trailer = String::from_utf8_lossy(&self.trailer).replace(crate::SOH_CHAR, crate::PIPE_STR);
        write!(f, "header: '{}', body: '{}', trailer: '{}'", header, body, trailer)
    }
}
impl Debug for HeapSerializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl ser::Serializer for &mut HeapSerializer {
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
        self.body.put_slice(v.to_string().as_bytes());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.body.put_slice(v.to_string().as_bytes());
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
        self.body.put_slice(v.as_bytes());
        Ok(())
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

impl ser::SerializeSeq for &mut HeapSerializer {
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

impl ser::SerializeTuple for &mut HeapSerializer {
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

impl ser::SerializeTupleStruct for &mut HeapSerializer {
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

impl ser::SerializeTupleVariant for &mut HeapSerializer {
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

impl ser::SerializeMap for &mut HeapSerializer {
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

impl ser::SerializeStruct for &mut HeapSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.body.put_slice(key.as_bytes());
        self.body.put_u8(crate::EQS_U8);
        // self.body.put_slice(value.as_bytes());
        value.serialize(&mut **self)?;
        self.body.put_u8(crate::SOH_U8);
        Ok(())
        // todo!()
    }

    fn end(self) -> Result<()> {
        Ok(())
        // todo!()
    }
}

impl ser::SerializeStructVariant for &mut HeapSerializer {
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

pub fn to_bytes(value: &impl Serialize) -> Result<HeapSerializer> {
    let mut ser = HeapSerializer::with_capacity(1024);
    value.serialize(&mut ser)?;
    Ok(ser)
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {
    use crate::unittest::setup;

    use super::*;
    use crate::prelude::{FixStr, FixString, FixStringLike, Tag};
    use log::info;
    use serde::Deserialize;
    use serde_json::{from_str, to_string, to_string_pretty};

    #[test]
    fn test_tag_serialize() {
        setup::log::configure();
        let tag = Tag::new(1);
        let fix = to_bytes(&tag).unwrap();
        info!("fix: {}", fix);
        let jsn = to_string(&tag).unwrap();
        info!("jsn: {}", jsn);
    }

    #[test]
    fn test_fix_string_serialize() {
        setup::log::configure();
        let str = FixString::try_from(b"ABC").unwrap();
        let fix = to_bytes(&str).unwrap();
        info!("fix: {}", fix);
        let jsn = to_string(&str).unwrap();
        info!("jsn: {}", jsn);
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

        // TODO see if to_pretty_json can be used to get the name of the field to be alias

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
