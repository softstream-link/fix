use std::fmt::{Debug, Display};

use crate::prelude::{Serialize, Tag};

pub trait Value: Serialize + Display + Debug + Clone + PartialEq {}

pub trait StringValue: Value + AsRef<str> {}
impl Serialize for &str {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(self.as_bytes());
        ser.serialize_soh();
    }
}
impl Value for &str {}
impl StringValue for &str {}
impl Serialize for String {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_slice(self.as_bytes());
        ser.serialize_soh();
    }
}
impl Value for String {}
impl StringValue for String {}
impl Serialize for char {
    #[inline(always)]
    fn serialize(&self, ser: &mut impl crate::prelude::Serializer) {
        debug_assert!(self.is_ascii(), "Not Ascii '{:x?}'", self);
        ser.serialize_u8(*self as u8);
        ser.serialize_soh();
    }
}
impl Value for char {}

pub trait NumericValue: Value {}
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
        impl NumericValue for $TYPE {}
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

pub trait Field: Serialize {
    fn tag(&self) -> Tag;
    fn value(&self) -> &impl Value;
}
