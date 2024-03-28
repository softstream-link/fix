use std::{
    borrow::Borrow,
    fmt::{self, Debug, Display},
    ops::Deref,
};

use crate::error::Error;
use crate::prelude::{FixStr, Result};
// use serde::{de::Visitor, de::Deserialize, Serialize};

use super::FixStringLike;

#[derive(Clone, PartialEq, Default)]
pub struct FixString(pub(crate) Vec<u8>);
impl TryFrom<&[u8]> for FixString {
    type Error = Error;
    #[inline(always)]
    fn try_from(val: &[u8]) -> Result<Self> {
        #[cfg(not(feature = "assume_u8_is_ascii"))]
        {
            if !val.is_ascii() {
                return Err(Error::Message(format!("Not Ascii, val: {:?}", val)));
            }
        }
        Ok(Self(val.to_vec()))
    }
}
impl TryFrom<Vec<u8>> for FixString {
    type Error = Error;
    #[inline(always)]
    fn try_from(val: Vec<u8>) -> Result<Self> {
        #[cfg(not(feature = "assume_u8_is_ascii"))]
        {
            if !val.is_ascii() {
                return Err(Error::Message(format!("Not Ascii, val: {:?}", val)));
            }
        }
        Ok(Self(val))
    }
}
impl TryFrom<&str> for FixString {
    type Error = Error;
    #[inline(always)]
    fn try_from(val: &str) -> Result<Self> {
        val.as_bytes().try_into()
    }
}
impl<const N: usize> TryFrom<&[u8; N]> for FixString {
    type Error = Error;
    #[inline(always)]
    fn try_from(val: &[u8; N]) -> Result<Self> {
        val.as_ref().try_into()
    }
}
impl TryFrom<String> for FixString {
    type Error = Error;
    #[inline(always)]
    fn try_from(val: String) -> Result<Self> {
        Self::try_from(val.into_bytes())
    }
}

impl AsRef<FixStr> for FixString {
    #[inline(always)]
    fn as_ref(&self) -> &FixStr {
        self.borrow()
    }
}
impl Borrow<FixStr> for FixString {
    #[inline(always)]
    fn borrow(&self) -> &FixStr {
        // this is what happens below in a single line with types and casts
        // let ptr: &[u8] = &*self.0; // self.0 is a Vec<u8>
        // let ptr: *const [u8] = ptr as *const [u8];
        // let ptr: *const FixStr = ptr as *const FixStr;
        // let ptr: &FixStr = unsafe { &*ptr };
        // ptr
        let ptr = &*self.0 as *const [u8] as *const FixStr;
        unsafe { &*ptr }
    }
}
impl Deref for FixString {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for FixString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl Debug for FixString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl serde::ser::Serialize for FixString {
    #[inline(always)]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let str: &FixStr = self.borrow();
        // TODO is this save? also shoudl serailize be implemented for Any T where T: FixStr?
        serializer.serialize_str(unsafe { str.as_str() })
    }
}
impl<'de> serde::de::Deserialize<'de> for FixString {
    #[inline(always)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        deserializer.deserialize_string(FixStringVisitor)
    }
}
impl FixStringLike for FixString {}

struct FixStringVisitor;
impl<'de> serde::de::Visitor<'de> for FixStringVisitor {
    type Value = FixString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a FixString")
    }

    // // used by fix_serde to avoid crating a utf8 string and related validation
    // fn visit_byte_buf<E: serde::de::Error>(self, v: Vec<u8>) -> std::result::Result<Self::Value, E> {
    //     match FixString::try_from(v) {
    //         Ok(x) => Ok(x),
    //         Err(e) => Err(E::custom(e.to_string())),
    //     }
    // }
    // used by serde_json
    fn visit_string<E: serde::de::Error>(self, v: String) -> std::result::Result<Self::Value, E> {
        self.visit_str(v.as_str())
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Self::Value, E> {
        FixString::try_from(v).map_err(|e| E::custom(e.to_string()))
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use crate::unittest::setup;
    use log::info;

    #[test]
    fn test_fix_string() -> Result<()> {
        setup::log::configure();
        let v1 = "ABC";
        let s1 = FixString::try_from(v1)?;
        info!("v1: {:?}", v1);
        info!("s1: {:?}", s1);

        let v2 = b"ABC";
        let s2 = FixString::try_from(v2)?;
        info!("v2: {:?}", v2);
        info!("s2: {:?}", s2);

        assert_eq!(s1, s2);

        let v3 = "ABC".to_owned();
        info!("v3: {:?}", v3);
        let s3 = FixString::try_from(v3)?;
        info!("s3: {:?}", s3);

        assert_eq!(s1, s3);

        let v4 = b"ABC".to_vec();
        info!("v4: {:?}", v4);
        let s4 = FixString::try_from(v4)?;
        info!("s4: {:?}", s4);

        assert_eq!(s1, s4);
        Ok(())
    }
}
