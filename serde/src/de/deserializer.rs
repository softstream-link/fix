use std::marker::PhantomData;

use serde::de::{self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor};

use super::read::{Read, SliceRead};
use crate::error::{Error, Result};

pub struct Deserializer<R> {
    read: R,
}

impl<'de, R: Read<'de>> Deserializer<R> {
    pub fn new(read: R) -> Self {
        Deserializer { read }
    }
}

impl<'a> Deserializer<SliceRead<'a>> {
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self::new(SliceRead::new(slice))
    }
}

impl<'de, R: Read<'de>> Deserializer<R> {
    /// The [Self::end] should be called after a value is fully deserialized to check if there are any trailing bytes.
    pub fn end(&mut self) -> Result<()> {
        if self.read.peek()?.is_none() {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }
}

fn from_trait<'de, R: Read<'de>, T: de::Deserialize<'de>>(read: R) -> Result<T> {
    let mut deserializer = Deserializer::new(read);
    let t = T::deserialize(&mut deserializer)?;
    deserializer.end()?;
    Ok(t)
}
pub fn from_slice<'a, T: de::Deserialize<'a>>(s: &'a [u8]) -> Result<T> {
    from_trait(SliceRead::new(s))
}
impl<'de, 'a, R: Read<'de>> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // visitor.visit_string(v)
        todo!()
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let str = self.read.parse_str_as_sub_slice()?;
        let mut v = Vec::<u8>::new();
        v.extend_from_slice(str);
        visitor.visit_byte_buf(v)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, name: &'static str, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(self, name: &'static str, len: usize, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_struct<V: Visitor<'de>>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        prelude::{to_bytes, FixStr, FixString, FixStringLike},
        unittest::setup,
    };
    use log::info;
    use serde::{Deserialize, Serialize};
    // use serde_json::from_str;
    #[test]
    fn test_deserializer() -> Result<()> {
        setup::log::configure();
        let string = FixString::try_from(b"123\x01").unwrap();
        let fix = to_bytes(&string).unwrap();
        info!("fix: {}", fix);
        let string: FixString = from_slice(&fix.body)?;
        // let mut deserializer = Deserializer::from_slice(s);
        // let x: u32 = Deserialize::deserialize(&mut deserializer)?;
        info!("string: {:?}", string);
        // assert_eq!(x, 123);
        Ok(())
    }

    #[test]
    fn test_msg_deserialize() {
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
        let msg = Msg::<FixString> {
            account,
            adv_id,
            begin_seq_no,
        };
        // let msg = Msg::<FixString> { account, adv_id };
        // let msg = msg.to_owned();
        let fix = to_bytes(&msg).unwrap();
        info!("fix: {}", fix);
        let slice = &fix.body[..];
        let msg: Msg<FixString> = from_slice(slice).unwrap();
        // let jsn = to_string(&msg).unwrap();
        // info!("jsn: {}", jsn);
        // let msg: Msg<FixString> = from_str(&jsn).unwrap();
        // info!("msg: {:?}", msg);

        // let jsn = r#" { "2":"DEF", "7": 100, "1":"ABC" } "#;
        // let msg: Msg<FixString> = from_str(&jsn).unwrap();
        // info!("msg: {:?}", msg);
    }
}
