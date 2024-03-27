use std::{fmt::{Debug, Display}, ops::Deref};

use bytes::{BufMut, BytesMut};

use crate::error::Result;
mod private {
    pub trait Sealed {}
}

pub trait Write: private::Sealed + Display {
    #[doc(hidden)]
    fn write_slice(&mut self, buf: &[u8]) -> Result<()>;
    #[doc(hidden)]
    fn write_u8(&mut self, value: u8) -> Result<()>;
    #[doc(hidden)]
    #[inline(always)]
    fn write_soh(&mut self) -> Result<()> {
        self.write_u8(crate::SOH)
    }
    #[doc(hidden)]
    #[inline(always)]
    fn write_eqs(&mut self) -> Result<()> {
        self.write_u8(crate::EQS)
    }
    
}

pub struct BytesWrite {
    bytes: BytesMut,
}
impl BytesWrite {
    pub fn new(bytes: BytesMut) -> Self {
        Self { bytes }
    }
    pub fn into_inner(self) -> BytesMut {
        self.bytes
    }
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}
impl Deref for BytesWrite {
    type Target = BytesMut;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl From<BytesWrite> for BytesMut {
    fn from(write: BytesWrite) -> Self {
        write.bytes
    }
}
impl private::Sealed for BytesWrite {}
impl Write for BytesWrite {
    #[inline(always)]
    fn write_slice(&mut self, buf: &[u8]) -> Result<()> {
        self.bytes.put_slice(buf);
        Ok(())
    }
    #[inline(always)]
    fn write_u8(&mut self, value: u8) -> Result<()> {
        self.bytes.put_u8(value);
        Ok(())
    }
}
impl Display for BytesWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes).replace(crate::SOH_CHAR, crate::PIPE_STR))
    }
}
impl Debug for BytesWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}
