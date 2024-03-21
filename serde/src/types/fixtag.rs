use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::prelude::{Error, FixStr};

// #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Tag(u32);
impl Tag {
    #[inline(always)]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}
impl Display for Tag {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO optimize to use
        // let mut buffer = itoa::Buffer::new();
        // let s = buffer.format(value);
        // writer.write_all(s.as_bytes())
        write!(f, "{}", self.0)
    }
}
impl TryFrom<&FixStr> for Tag {
    type Error = Error;
    fn try_from(value: &FixStr) -> Result<Self, Self::Error> {
        // TODO check to see if itoa crate can do this
        let mut result = 0u32;
        for &digit in value.iter() {
            if !digit.is_ascii_digit() {
                return Err(Error::TagStringIsNotNumeric(value.to_owned()).into());
            }
            result = result * 10 + (digit - 48) as u32; // TODO overflow check is it necessary?
                                                        // result = result.checked_mul(10)?.checked_add(u32::from(digit))?;
        }
        Ok(result.into())
    }
}
// impl<S: AsRef<FixStr>> TryFrom<S> for Tag{
//     fn try_from(value: S) -> Result<Self, Self::Error> {
//         let value = value.as_ref();
//         let value = std::str::from_utf8(value).map_err(|e| format!("Invalid utf8: {:?}", e))?;
//         let value = value.parse().map_err(|e| format!("Invalid number: {:?}", e))?;
//         Ok(Self(value))
//     }
// }
impl From<u32> for Tag {
    #[inline(always)]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use crate::unittest::setup;

    use super::*;

    #[test]
    fn test_tag() -> Result<(), Error> {
        setup::log::configure();
        let str = FixStr::from_ascii(b"12");
        info!("str: {:?}", str);
        let tag = Tag::try_from(str)?;
        info!("tag: {:?}", tag);
        assert_eq!(tag, 12.into());

        let str = FixStr::from_ascii(b"1a");
        info!("str: {:?}", str);
        let err = Tag::try_from(str).unwrap_err();
        info!("err: {:?}", err);
        assert!(matches!(err, Error::TagStringIsNotNumeric(_)));
        Ok(())
    }
}
