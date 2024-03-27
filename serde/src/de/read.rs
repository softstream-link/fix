use crate::error::{Error, Result};
use crate::{EQS, SOH};

// Prevent users from implementing the Read trait.
mod private {
    pub trait Sealed {}
}

// pub trait Read<'de>{
pub trait Read<'de>: private::Sealed {
    #[doc(hidden)]
    fn next(&mut self) -> Result<Option<u8>>;
    #[doc(hidden)]
    fn peek(&mut self) -> Result<Option<u8>>;
    // on
    #[doc(hidden)]
    fn discard(&mut self);

    /// returns a slice of bytes that represents the value of a FIX field.
    /// Fix value always located between `=` and `0x01` bytes known as `SOH` and represented using a `|`.
    /// Example: `"8=value1|9=value2|"`
    #[doc(hidden)]
    fn parse_fix_value(&mut self) -> Result<&'de [u8]>;

    #[doc(hidden)]
    fn parse_unsigned(&mut self) -> Result<u64> {
        // TODO add overflow check and error message
        let mut bytes = self.parse_fix_value()?;
        // skip leading zeros
        for &digit in bytes {
            if digit != b'0' {
                break;
            }
            bytes = &bytes[1..];
        }
        // parse the unsigned integer
        let mut res = 0;
        for &digit in bytes {
            match digit {
                ch @ b'0'..=b'9' => {
                    // note: ch is the ASCII character '2', then ch as u8 is 50. If you subtract b'0' (which is 48) from it, you get 2, which is the numerical value of the character '2'.
                    res = res * 10 + (ch - b'0') as u64;
                }
                _ => return Err(Error::InvalidUnsignedInteger),
            }
        }
        Ok(res)
    }

    #[doc(hidden)]
    fn parse_fix_tag(&mut self) -> Result<&'de [u8]>;
    /// Advance to the next [crate::SOH_U8] byte `without` consuming it
    #[doc(hidden)]
    fn advance_to_soh(&mut self) -> Result<()>;

    /// Advance to the next [crate::EQS_U8] byte `without` consuming it
    #[doc(hidden)]
    fn advance_to_eqs(&mut self) -> Result<()>;
}

// Slice read and its methods are future proofing for the possibility of reading from Io
pub struct SliceRead<'a> {
    slice: &'a [u8],
    index: usize,
}
impl<'de> SliceRead<'de> {
    pub fn new(slice: &'de [u8]) -> Self {
        Self { slice, index: 0 }
    }
}
impl<'de> private::Sealed for SliceRead<'de> {}
impl<'de> Read<'de> for SliceRead<'de> {
    #[inline(always)]
    fn next(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| { self.index += 1; *ch }))`
        // is about 10% slower.
        Ok(if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        })
    }

    #[inline(always)]
    fn peek(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| *ch))` is about 10% slower
        // for some reason.
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        })
    }

    #[inline(always)]
    fn discard(&mut self) {
        self.index += 1;
    }

    #[inline(always)]
    fn advance_to_soh(&mut self) -> Result<()> {
        while match self.peek()? {
            Some(SOH) => false,
            None => return Err(Error::Eof),
            _ => {
                self.discard();
                true
            }
        } {}
        Ok(())
    }
    #[inline(always)]
    fn advance_to_eqs(&mut self) -> Result<()> {
        while match self.peek()? {
            Some(EQS) => false,
            None => return Err(Error::Eof),
            _ => {
                self.discard();
                true
            }
        } {}
        Ok(())
    }
    #[inline(always)]
    fn parse_fix_value(&mut self) -> Result<&'de [u8]> {
        let start = self.index;
        self.advance_to_soh()?;
        let res = &self.slice[start..self.index];
        self.discard(); // SOH
        match res.len() {
            0 => return Err(Error::EmptyValue),
            _ => {}
        }
        Ok(res)
    }

    #[inline(always)]
    fn parse_fix_tag(&mut self) -> Result<&'de [u8]> {
        let start = self.index;
        self.advance_to_eqs()?;
        let res = &self.slice[start..self.index];
        self.discard(); // EQS
        Ok(res)
    }
}
