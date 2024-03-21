use crate::error::{Error, Result};
use crate::SOH_U8;

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

    #[doc(hidden)]
    fn parse_str_as_sub_slice(&mut self) -> Result<&[u8]>;

    /// Advance to the next [crate::SOH_U8] byte `without` consuming it
    #[doc(hidden)]
    fn advance_to_soh(&mut self) -> Result<()>;
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
            Some(SOH_U8) => false,
            None => return Err(Error::EOFWithOutSOH),
            _ => {
                self.discard();
                true
            }
        } {}
        Ok(())
    }
    #[inline(always)]
    fn parse_str_as_sub_slice(&mut self) -> Result<&[u8]> {
        let start = self.index;
        self.advance_to_soh()?;
        let res = &self.slice[start..self.index];
        self.discard(); // SOH
        Ok(res)
    }
}
