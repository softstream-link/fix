use std::fmt::{Debug, Display};

use crate::prelude::{Serialize, Serializer};

pub trait Value: Serialize {}

impl<const N: usize> Serialize for &[u8; N] {
    fn serialize(&self, ser: &mut impl Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(*self);
        ser.serialize_soh();
    }
}
impl<const N: usize> Value for &[u8; N] {}

impl Serialize for &[u8] {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(*self);
        ser.serialize_soh();
    }
}
impl Value for &[u8] {}
impl Serialize for &str {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(self.as_bytes());
        ser.serialize_soh();
    }
}
impl Value for &str {}
impl Serialize for String {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(self.as_bytes());
        ser.serialize_soh();
    }
}
impl Value for String {}
impl Serialize for char {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_u8(*self as u8);
    }
}
impl Value for char {}

macro_rules! impl_value_for_number {
    ($TYPE:ty) => {
        impl Serialize for $TYPE {
            #[inline(always)]
            fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
                ser.serialize_slice(self.to_string().as_bytes());
                ser.serialize_soh();
            }
        }
        impl Value for $TYPE {}
    };
}
impl_value_for_number!(i8);
impl_value_for_number!(u8);
impl_value_for_number!(i16);
impl_value_for_number!(u16);
impl_value_for_number!(i32);
impl_value_for_number!(u32);
impl_value_for_number!(i64);
impl_value_for_number!(u64);
impl_value_for_number!(isize);
impl_value_for_number!(usize);
impl_value_for_number!(i128);
impl_value_for_number!(u128);

impl_value_for_number!(f64);
impl_value_for_number!(f32);

pub struct Tag(u32);
impl From<u32> for Tag {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self(value)
    }
}
impl Serialize for Tag {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl Serializer) {
        ser.serialize_slice(self.0.to_string().as_bytes());
        ser.serialize_eqs();
    }
}

pub trait Field: Serialize {
    fn tag(&self) -> Tag;
    fn value(&self) -> &impl Value;

    // fn write(&self, dst: &mut impl bytes::BufMut) {
    //     self.tag().serialize_tag(dst);
    //     // self.value().serialize_value(dst);
    // }
}

