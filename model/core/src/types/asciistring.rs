use crate::error::{Error, Result};
use crate::prelude::asc;
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    ops::Deref,
};

// #[inline]
#[cold]
pub(crate) fn is_ascii(val: &[u8]) -> Result<()> {
    match val.is_ascii() {
        true => Ok(()),
        false => Err(Error::not_ascii_bytes(val.to_vec())),
    }
}
#[derive(Clone, PartialEq)]
pub struct Ascii(pub(crate) Vec<u8>);
impl Ascii {
    #[inline]
    pub fn as_str(&self) -> &str {
        let fix_str: &asc = self.borrow();
        fix_str.as_str()
    }

    #[inline]
    pub fn try_from_str(value: &str) -> Result<Self> {
        Ascii::try_from(value)
    }
    #[inline]
    pub fn as_asc(&self) -> &asc {
        self.deref()
    }
}
impl TryFrom<&[u8]> for Ascii {
    type Error = Error;
    /// The call will allocate a new [Vec<u8>] and copy input slice into it.
    ///
    /// Returns [Result]:
    /// * [`Ok(Ascii)`] if input is valid ascii.
    /// * [`Err(Error)`] if input is not valid ascii.
    #[inline]
    fn try_from(value: &[u8]) -> Result<Self> {
        if !value.is_ascii() {
            return Err(Error::not_ascii_bytes(value.to_vec()));
        }
        Ok(Self(value.to_vec()))
    }
}
impl TryFrom<Vec<u8>> for Ascii {
    type Error = Error;
    /// The call `WILL NOT` allocate, but consumes input [Vec<u8>]
    ///
    /// Returns [Result]:
    /// * [`Ok(Ascii)`] if input is valid ascii.
    /// * [`Err(Error)`] if input is not valid ascii.
    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self> {
        if !value.is_ascii() {
            return Err(Error::not_ascii_bytes(value));
        }
        // don't use try_from(val: &[u8]) which will allocate a new Vec
        // instead consume the Vec and use it directly
        Ok(Self(value))
    }
}
impl TryFrom<&str> for Ascii {
    type Error = Error;
    /// The call will allocate a new [Vec<u8>] and copy input str into it.
    ///
    /// Returns [Result]:
    /// * [`Ok(Ascii)`] if input is valid ascii.
    /// * [`Err(Error)`] if input is not valid ascii.
    #[inline]
    fn try_from(value: &str) -> Result<Self> {
        if !value.is_ascii() {
            return Err(Error::not_ascii_string(value.to_string()));
        }
        Ok(Self(value.as_bytes().to_vec()))
    }
}
impl<const N: usize> TryFrom<&[u8; N]> for Ascii {
    type Error = Error;
    /// The call will allocate a new [Vec<u8>] and copy input str into it.
    ///
    /// Returns [Result]:
    /// * [`Ok(Ascii)`] if input is valid ascii.
    /// * [`Err(Error)`] if input is not valid ascii.
    #[inline]
    fn try_from(value: &[u8; N]) -> Result<Self> {
        if !value.is_ascii() {
            return Err(Error::not_ascii_bytes(value.to_vec()));
        }
        Ok(Self(value.as_ref().to_vec()))
    }
}
impl TryFrom<String> for Ascii {
    type Error = Error;
    /// The call `WILL NOT` allocate, but consumes input [String]
    ///
    /// Returns [Result]:
    /// * [`Ok(Ascii)`] if input is valid ascii.
    /// * [`Err(Error)`] if input is not valid ascii.
    #[inline]
    fn try_from(value: String) -> Result<Self> {
        if !value.is_ascii() {
            return Err(Error::not_ascii_string(value));
        }
        Ok(Self(value.into_bytes()))
    }
}
impl Borrow<asc> for Ascii {
    #[inline]
    fn borrow(&self) -> &asc {
        // this is what happens below in a single line with types and casts
        // let ptr: &[u8] = &*self.0; // self.0 is a Vec<u8>
        // let ptr: *const [u8] = ptr as *const [u8];
        // let ptr: *const FixStr = ptr as *const FixStr;
        // let ptr: &FixStr = unsafe { &*ptr };
        // ptr
        let ptr = &*self.0 as *const [u8] as *const asc;
        unsafe { &*ptr }
    }
}
impl AsRef<asc> for Ascii {
    #[inline]
    fn as_ref(&self) -> &asc {
        self.borrow()
    }
}
impl AsRef<str> for Ascii {
    #[inline]
    fn as_ref(&self) -> &str {
        let fix_str: &asc = self.borrow();
        fix_str.as_str()
    }
}

impl Deref for Ascii {
    type Target = asc;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &Borrow::<asc>::borrow(self)
    }
}
impl Display for Ascii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl Debug for Ascii {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Borrow::<asc>::borrow(self))
    }
}

impl serde::ser::Serialize for Ascii {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let v = self.as_asc();
        match serializer.is_human_readable() {
            true => serializer.serialize_str(v.as_str()),
            false => serializer.serialize_bytes(v),
        }
    }
}
impl<'de> serde::de::Deserialize<'de> for Ascii {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Ok(<&asc>::deserialize(deserializer)?.to_owned())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use fix_model_test::unittest::setup;
    use log::info;

    #[test]
    fn test_fix_string() -> Result<()> {
        setup::log::configure();
        let v1 = "ABC";
        let s1 = Ascii::try_from(v1)?;
        info!("v1: {:?}", v1);
        info!("s1: {:?}", s1);

        let v2 = b"ABC";
        let s2 = Ascii::try_from(v2)?;
        info!("v2: {:?}", v2);
        info!("s2: {:?}", s2);

        assert_eq!(s1, s2);

        let v3 = "ABC".to_owned();
        info!("v3: {:?}", v3);
        let s3 = Ascii::try_from(v3)?;
        info!("s3: {:?}", s3);

        assert_eq!(s1, s3);

        let v4 = b"ABC".to_vec();
        info!("v4: {:?}", v4);
        let s4 = Ascii::try_from(v4)?;
        info!("s4: {:?}", s4);

        assert_eq!(s1, s4);
        Ok(())
    }

    #[test]
    fn test_fix_string_not_ascii() -> Result<()> {
        setup::log::configure();
        let utf8 = "Hello 💖 World";

        info!("utf8: {:?}", utf8);

        let err = Ascii::try_from(utf8).unwrap_err();
        info!("err: {}", err);

        Ok(())
    }
}
