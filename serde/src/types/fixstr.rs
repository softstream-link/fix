use std::{
    fmt::{Debug, Display},
    mem::transmute,
    ops::Deref,
};

use serde::Serialize;

use crate::prelude::{FixString, FixStringLike};

#[repr(transparent)]
#[derive(PartialEq)]
pub struct FixStr([u8]);
impl FixStr {
    pub fn from_ascii(val: &[u8]) -> &Self {
        unsafe { transmute(val) }
    }
    pub unsafe fn as_str(&self) -> &str {
        std::str::from_utf8_unchecked(&self.0)
    }
}

// TODO is this even safe unless array ref is 'static
impl<const N: usize> TryFrom<&[u8; N]> for &FixStr {
    type Error = String;
    #[inline(always)]
    fn try_from(val: &[u8; N]) -> Result<Self, Self::Error> {
        if !val.is_ascii() {
            // TODO ascii check based on feature
            Err(format!("Not Ascii, val: {:?}", val))
        } else {
            let ptr: &[u8] = val.as_ref();
            let ptr: *const [u8] = ptr as *const [u8];
            let ptr: *const FixStr = ptr as *const FixStr;
            Ok(unsafe { &*ptr })
        }
    }
}
impl Serialize for &FixStr {
    #[inline(always)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO is this save? also shoudl serailize be implemented for Any T where T: FixStr?
        serializer.serialize_str(unsafe { self.as_str() })
    }
}
impl FixStringLike for &FixStr {}

impl AsRef<FixStr> for &FixStr {
    #[inline(always)]
    fn as_ref(&self) -> &FixStr {
        self
    }
}
impl Deref for FixStr {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for FixStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self))
    }
}
impl Debug for FixStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "assume_u8_is_ascii"))]
        write!(f, "{:?}", self.to_string())?;

        #[cfg(feature = "assume_u8_is_ascii")]
        write!(f, "{:?}", unsafe { std::from_utf8_unchecked(self) })?;

        Ok(())
    }
}
impl ToOwned for FixStr {
    type Owned = FixString;
    #[inline(always)]
    fn to_owned(&self) -> Self::Owned {
        FixString(self.0.to_vec())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::unittest::setup;
    use crate::prelude::Result;
    use log::info;
    use std::borrow::Borrow;

    #[test]
    fn test_borrow_fix_string() -> Result<()> {
        setup::log::configure();
        let v = "ABC";
        info!("v: {:?}", v);
        let s_owned = FixString::try_from(v)?;
        info!("s_owned: {:?}", s_owned);

        let s_borrow: &FixStr = s_owned.borrow();
        // drop(s_owned); // fails as s_owned is borrowed using borrow
        info!("s_borrow: {:?}", s_borrow);

        Ok(())
    }
}
