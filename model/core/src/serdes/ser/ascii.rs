use std::{
    borrow::Borrow,
    fmt::{Debug, Display}, ops::Deref,
};


pub struct FixString(Vec<u8>);

impl From<String> for FixString {
    #[inline(always)]
    fn from(val: String) -> Self {
                val.as_bytes().into()
    }
}
impl From<&str> for FixString {
    #[inline(always)]
    fn from(val: &str) -> Self {
        val.as_bytes().into()
    }
}
impl<const N: usize> From<&[u8; N]> for FixString {
    #[inline(always)]
    fn from(val: &[u8; N]) -> Self {
        val.as_ref().into()
    }
}
impl From<&[u8]> for FixString {
    #[inline(always)]
    fn from(val: &[u8]) -> Self {
        // TODO validate enchoding
        // TODO try_into?
        Self(val.to_vec())
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
#[repr(transparent)]
pub struct FixStr([u8]);
impl Display for FixStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl Debug for FixStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}
// impl From<&str> for &FixStr {
//     fn from(val: &str) -> Self {
//         let x = val.as_bytes() as *const [u8] as *const FixStr;
//         let y = unsafe { &*x };
//         y
//         // unsafe { *x }
//     }
// }

// impl ToOwned for FixStr {
//     type Owned = FixString;
//     fn to_owned(&self) -> Self::Owned {
//         FixString(self.0.to_vec())
//     }
// }
impl Borrow<FixStr> for FixString {
    fn borrow(&self) -> &FixStr {
        // this is what happens below in a single line with types and casts
        // let x: &[u8] = &*self.0; self.0 is a Vec<u8>
        // let x: *const [u8] = x as *const [u8];
        // let x: *const FixStr = x as *const FixStr;
        // let x: &FixStr = unsafe { &*x };
        // x
        let ptr = &*self.0 as *const [u8] as *const FixStr;
        unsafe { &*ptr }
    }
}
impl Borrow<FixStr> for [u8] {
    fn borrow(&self) -> &FixStr {
        let x = self;
        let x = x as *const [u8];
        let x = x as *const FixStr;
        let x = unsafe { &*x };
        x
    }
} 

#[cfg(test)]
mod tests {
    use log::info;

    use crate::unittest::setup;

    use super::*;

    #[test]
    fn test_borrow_slice() {
        setup::log::configure();
        let v = b"ABC".as_slice();
        let borrow  : &FixStr = Borrow::<FixStr>::borrow(v);
        drop(v);
        info!("borrow: {:?}", borrow);
    }
    #[test]
    fn test_borrow_fix_string() {
        setup::log::configure();
        let v = "ABC";
        let owned = FixString::from(v);
        let v = b"ABC";
        let owned = FixString::from(v);
        info!("owned: {:?}", owned);
        info!("owned: {}", owned);

        let borrow: &FixStr = owned.borrow();
        // drop(owned);
        info!("borrow: {:?}", borrow);

        let string = "Blah".to_owned();
        info!("string: {:?}", string);
        let str = string.as_str();
        info!("str: {:?}", str);
        // let fix_str: &FixStr = str.borrow();
        // let fix_str: &FixStr = str.borrow();
        // drop(string); //failss
        // info!("fix_str: {:?}", fix_str);

        // let fix_str: &FixStr = "ABC".into();
    }
}
