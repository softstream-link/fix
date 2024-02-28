use super::asciistring::is_ascii;
use crate::error::{Error, Result};
use crate::prelude::Ascii;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct asc([u8]);
impl asc {
    #[inline]
    pub fn as_str(&self) -> &str {
        // as long as it is crated using
        //      a) TryFrom<&'static [u8; N]> for &AsciiStr
        //      b) borrowed from AsciiString
        // the array will contains valid ascii because validation is done in AsciiString and TryFrom<&'static [u8; N]> for &AsciiStr
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    /// Caller must ensure that the input str is valid ascii
    #[inline]
    unsafe fn from_str_unchecked(s: &str) -> &Self {
        let ptr: *const [u8] = s.as_bytes() as *const [u8];
        let ptr: *const asc = ptr as *const asc;
        unsafe { &*ptr }
    }
    pub fn try_from_slice(slice: &[u8]) -> Result<&Self> {
        if !slice.is_ascii() {
            Err(Error::NotAsciiBytes(slice.to_vec()))
        } else {
            Ok(unsafe { Self::from_str_unchecked(std::str::from_utf8_unchecked(slice)) })
        }
    }
    unsafe fn from_bytes_unchecked(slice: &[u8]) -> &Self {
        let ptr: *const [u8] = slice as *const [u8];
        let ptr: *const asc = ptr as *const asc;
        unsafe { &*ptr }
    }

    /// only use for testing othwerwise use [TryFrom]
    pub fn try_from_str(value: &str) -> Result<&Self> {
        value.try_into()
    }
}

impl TryFrom<&[u8]> for &asc {
    type Error = Error;
    #[inline]
    fn try_from(val: &[u8]) -> Result<Self> {
        is_ascii(val)?;

        let ptr: &[u8] = val.as_ref();
        let ptr: *const [u8] = ptr as *const [u8];
        let ptr: *const asc = ptr as *const asc;
        Ok(unsafe { &*ptr })
    }
}
impl<const N: usize> TryFrom<&[u8; N]> for &asc {
    type Error = Error;
    #[inline]
    fn try_from(val: &[u8; N]) -> Result<Self> {
        <&asc>::try_from(val as &[u8])
    }
}
impl TryFrom<&str> for &asc {
    type Error = Error;
    #[inline]
    fn try_from(val: &str) -> Result<Self> {
        is_ascii(val.as_bytes())?;

        let ptr: *const [u8] = val.as_bytes() as *const [u8];
        let ptr: *const asc = ptr as *const asc;
        Ok(unsafe { &*ptr })
    }
}
impl Serialize for &asc {
    #[inline]
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => serializer.serialize_str(self.as_str()),
            false => serializer.serialize_bytes(self),
        }
    }
}
impl<'de> Deserialize<'de> for &'de asc {
    #[inline]
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct AsciiStrVisitor;
        impl<'de> serde::de::Visitor<'de> for AsciiStrVisitor {
            type Value = &'de asc;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an &asc")
            }

            fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> std::result::Result<Self::Value, E> {
                is_ascii(v.as_bytes()).map_err(|e| E::custom(e))?;
                Ok(unsafe { asc::from_str_unchecked(v) })
            }
            fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> std::result::Result<Self::Value, E> {
                is_ascii(v).map_err(|e| E::custom(e))?;
                Ok(unsafe { asc::from_bytes_unchecked(v) })
            }
        }
        deserializer.deserialize_bytes(AsciiStrVisitor)
    }
}

impl AsRef<asc> for &asc {
    #[inline]
    fn as_ref(&self) -> &asc {
        self
    }
}
impl AsRef<str> for &asc {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl Deref for asc {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for asc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", String::from_utf8_lossy(&self))
        write!(f, "{}", self.as_str())
    }
}
impl Debug for asc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())?;
        Ok(())
    }
}
impl ToOwned for asc {
    type Owned = Ascii;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Ascii(self.0.to_vec())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use fix_model_test::unittest::setup;
    use log::info;
    use std::borrow::Borrow;

    #[test]
    fn test_borrow_fix_string() -> Result<()> {
        setup::log::configure();
        let _inp: &asc = "ABC".try_into().unwrap();

        let v = "ABC";
        info!("v: {:?}", v);
        let s_owned = Ascii::try_from(v)?;
        info!("s_owned: {:?}", s_owned);

        let s_borrow: &asc = s_owned.borrow();
        // drop(s_owned); // fails as s_owned is borrowed using borrow
        info!("s_borrow: {:?}", s_borrow);

        Ok(())
    }
}
