use std::{
    // borrow::Borrow,
    fmt::{Debug, Display},
    mem::transmute,
    ops::Deref,
};

use crate::prelude::FixString;

#[repr(transparent)]
#[derive(PartialEq)]
pub struct FixStr([u8]);
impl FixStr {
    pub fn from_ascii(val: &[u8]) -> &Self {
        unsafe { transmute(val) }
    }
}
impl AsRef<FixStr> for FixStr {
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
        write!(f, "{:?}", unsafe { std::std::from_utf8_unchecked(self) })?;

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
    use log::info;
    use std::borrow::Borrow;
    #[test]
    fn test_borrow_fix_string() -> Result<(), String> {
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
