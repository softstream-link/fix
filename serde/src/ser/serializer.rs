use super::{
    macros::{impl_serialize_integer, serialize_unimplemented},
    write::{BytesWrite, Write},
};
use crate::prelude::{Error, Result};
use bytes::BytesMut;
use fix_model_core::{prelude::Schema, schema::BinaryDataLenPair};
use serde::{ser, Serialize};
use std::{
    any::type_name,
    fmt::{Debug, Display},
    num::FpCategory,
    ops::Deref,
};

const NAME_SERIALIZER: &str = "Serializer";
pub struct Serializer<W, S> {
    write: W,
    phantom: std::marker::PhantomData<S>,
}
impl<W: Write, S: Schema> Serializer<W, S> {
    pub fn new(write: W) -> Self {
        Self {
            write,
            phantom: std::marker::PhantomData,
        }
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
impl<S> From<Serializer<BytesWrite, S>> for BytesMut {
    fn from(serializer: Serializer<BytesWrite, S>) -> Self {
        serializer.write.into()
    }
}
impl<S> Serializer<BytesWrite, S> {
    pub fn as_slice(&self) -> &[u8] {
        self
    }
    pub fn join(&mut self, other: Self) {
        self.write.join(other.write);
    }
}
impl<S> Deref for Serializer<BytesWrite, S> {
    type Target = BytesWrite;
    fn deref(&self) -> &Self::Target {
        &self.write
    }
}
impl<W: Write, S> Display for Serializer<W, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write.fmt(f)
    }
}
impl<W: Write, S> Debug for Serializer<W, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write.fmt(f)
    }
}
impl<W: Write, S: Schema> ser::Serializer for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    fn is_human_readable(&self) -> bool {
        false
    }
    impl_serialize_integer!(
        serialize_u8(self, v: u8),
        serialize_u16(self, v: u16),
        serialize_u32(self, v: u32),
        serialize_u64(self, v: u64),
        serialize_i8(self, v: i8),
        serialize_i16(self, v: i16),
        serialize_i32(self, v: i32),
        serialize_i64(self, v: i64),
    );
    fn serialize_f32(self, v: f32) -> Result<()> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => Err(Error::Message(format!("Float f32 is NaN or Infinity. value: {:?}", v))),
            _ => {
                let mut buffer = ryu::Buffer::new();
                let s = buffer.format_finite(v);
                self.write.write_value(s.as_bytes())
            }
        }
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        match v.classify() {
            FpCategory::Nan | FpCategory::Infinite => Err(Error::Message(format!("Float f64 is NaN or Infinity. value: {:?}", v))),
            _ => {
                let mut buffer = ryu::Buffer::new();
                let s = buffer.format_finite(v);
                self.write.write_value(s.as_bytes())
            }
        }
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        self.write.write_value(v.as_bytes())
    }
    fn serialize_char(self, v: char) -> Result<()> {
        // A char encoded as UTF-8 takes 4 bytes at most.
        let mut buf = [0; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }
    fn serialize_bool(self, v: bool) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.write.write_value(if v { "Y".as_bytes() } else { "N".as_bytes() })
    }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("{}::serialize_unit_variant: name: {}, variant: {}", NAME_SERIALIZER, _name, variant);
        self.write.write_value(variant.as_bytes())
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("{}::serialize_newtype_struct: name: {}", NAME_SERIALIZER, _name);
        value.serialize(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        #[cfg(debug_assertions)]
        log::trace!("{}::serialize_struct: name: {}, len: {}", NAME_SERIALIZER, _name, _len);
        Ok(self)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    /// Used to serialize Data types of fix messages   
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.write.write_value(v)
    }

    serialize_unimplemented!(Serializer<W>, serialize_unit(self),);
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        #[cfg(debug_assertions)]
        log::error!(
            "{}::serialize_tuple: Not Supported, attempting to serialize len: {}",
            NAME_SERIALIZER,
            _len
        );
        Err(Error::NotSupported("Serializer::serialize_tuple"))
    }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        #[cfg(debug_assertions)]
        log::error!(
            "{}::serialize_tuple_struct: Not Supported, attempting to serialize name: {}",
            NAME_SERIALIZER,
            _name
        );
        Err(Error::NotSupported("Serializer::serialize_tuple_struct"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        #[cfg(debug_assertions)]
        log::error!(
            "{}::serialize_tuple_variant: Not Supported, attempting to serialize name: {}",
            NAME_SERIALIZER,
            _name
        );
        Err(Error::NotSupported("Serializer::serialize_tuple_variant"))
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        #[cfg(debug_assertions)]
        log::error!("{}::serialize_map: Not Supported len: {:?}", NAME_SERIALIZER, _len);
        Err(Error::NotSupported("Serializer::serialize_map"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        #[cfg(debug_assertions)]
        log::error!(
            "{}::serialize_struct_variant: Not Supported, attempting to serialize name: {}",
            NAME_SERIALIZER,
            _name
        );
        Err(Error::NotSupported("Serializer::serialize_struct_variant"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()> {
        // note it is impossible to deserialize fix new_type_variant because the message type is in the header
        #[cfg(debug_assertions)]
        log::info!("{}::serialize_newtype_variant: name: {}", NAME_SERIALIZER, name);
        value.serialize(self)
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!(
            "{}::serialize_unit_struct: Not Supported, attempting to serialize name: {}",
            NAME_SERIALIZER,
            _name
        );
        Err(Error::NotSupported("Serializer::serialize_unit_struct"))
    }
    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::NotSupported(
            r#" It appears you are attempting to serialize an empty value. 
        Fix protocol does not support empty values. 
        Instead entire key=value need to be skipped. 
        Please ensure that you annotate the field with #[serde(skip_serializing_if = "Option::is_none")] 
        "#,
        ))
    }
    /// Only used for repeating group serialization & len/data pair
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        #[cfg(debug_assertions)]
        log::trace!("{}::serialize_seq: len: {:?}, buf: {}", NAME_SERIALIZER, len, self.write);

        if let Some(len) = len {
            if len > 0 {
                self.write.write_value(itoa::Buffer::new().format(len).as_bytes())?;
                self.write.write_soh()?;

                match self.write.last_written_tag() {
                    Some(tag) => match S::lookup(tag) {
                        // We are looking at binary data and hence need to add data with equal sign '<dat_tag>=' SerialieSeq::
                        Some(BinaryDataLenPair { tag_len: _, tag_data }) => {
                            self.write.write_tag(tag_data)?;
                            self.write.write_eqs()?;
                        }
                        // we are looking at repeating group and SerailizeSeq::next_element will serialize the rests
                        None => {
                            // self.serialize_soh()?; {}
                        }
                    },
                    None => {}
                }

                return Ok(self);
            }
        }

        Err(Error::Message(format!(
                    "{}::serialize_seq reserved for serializing repeating group & binary data, which requires len argument. len: {:?}.  Please use serialize_struct for other cases",
                    NAME_SERIALIZER,
                    len,
                )))
    }
}

impl<W: Write, S: Schema> ser::SerializeSeq for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    /// SerializeSeq::serialize_element: is exclusively reserved for serializing repeating group & binary data tag pair.
    /// struct Msg {
    ///     #[serde(rename="384")]
    ///     repeating_group: Vec<RepeatingGroup>,
    ///     #[serde(rename="95")]
    ///     data: Data,
    /// }
    /// Note that FIX is a TAG=VALUE pair protocol where TAG is field name and VALUE its content. So this method uses the id of last TAG serialized to dermine how to complete serialization of Binary data.
    ///
    /// In Above example this means that Data which has TAG "95" will be followed by TAG "96", this is determined via Serializer's internal schema.
    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::trace!("{}::serialize_element value type: {}", NAME_SERIALIZER, type_name::<T>());
        value.serialize(&mut **self)?;
        self.serialize_soh() // this is a special case when Data is serialized outside of the parent struct as it will not add SOH
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
impl<W: Write, S: Schema> ser::SerializeStruct for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        self.write.write_tag(key.as_bytes())?; // save key
        self.write.write_eqs()?; // save EQS
        value.serialize(&mut **self)?; // save value will write ...
        self.write.write_soh()?; // save SOH

        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
impl<W: Write, S: Schema> ser::SerializeTupleStruct for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// NOT IMPLEMENTED
impl<W: Write, S: Schema> ser::SerializeTuple for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeTuple::serialize_element: Not Supported value type: {}", type_name::<T>());
        Err(Error::NotSupported("SerializeTuple::serialize_element"))
    }

    fn end(self) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeTuple::serialize_element: Not Supported");
        Err(Error::NotSupported("SerializeTuple::end"))
    }
}
impl<W: Write, S: Schema> ser::SerializeStructVariant for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _key: &'static str, _value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!(
            "SerializeStructVariant::serialize_field: Not Supported key: {}, value type: {}",
            _key,
            type_name::<T>()
        );
        Err(Error::NotSupported("SerializeStructVariant::serialize_field"))
    }

    fn end(self) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeStructVariant::end: Not Supported");
        Err(Error::NotSupported("SerializeStructVariant::end"))
    }
}
impl<W: Write, S: Schema> ser::SerializeMap for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeMap::serialize_key: Not Supported key type: {}", type_name::<T>());
        Err(Error::NotSupported("SerializeMap::serialize_key"))
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeMap::serialize_value: Not Supported value type: {}", type_name::<T>());
        Err(Error::NotSupported("SerializeMap::serialize_value"))
    }

    fn end(self) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeMap::end: Not Supported");
        Err(Error::NotSupported("SerializeMap::end"))
    }
}
impl<W: Write, S: Schema> ser::SerializeTupleVariant for &mut Serializer<W, S> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeTupleVariant::serialize_field: Not Supported value type: {}", type_name::<T>());
        Err(Error::NotSupported("SerializeTupleVariant::serialize_field"))
    }

    fn end(self) -> Result<()> {
        #[cfg(debug_assertions)]
        log::error!("SerializeTupleVariant::end: Not Supported");
        Err(Error::NotSupported("SerializeTupleVariant::end"))
    }
}
