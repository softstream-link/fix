use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use bytes::{BufMut, BytesMut};

use crate::error::Result;
mod private {
    pub trait Sealed {}
}

pub trait Write: private::Sealed + Display {
    #[doc(hidden)]
    fn write_value(&mut self, buf: &[u8]) -> Result<()>;
    #[doc(hidden)]
    fn write_tag(&mut self, buf: &'static [u8]) -> Result<()>;
    #[doc(hidden)]
    fn last_written_tag(&self) -> Option<&'static [u8]>;
    #[doc(hidden)]
    fn write_soh(&mut self) -> Result<()>;
    #[doc(hidden)]
    fn write_eqs(&mut self) -> Result<()>;
}

pub struct BytesWrite {
    bytes: BytesMut,
    last_tag: Option<&'static [u8]>,
    write_soh_issued: bool,
}
impl BytesWrite {
    #[inline]
    pub fn new(bytes: BytesMut) -> Self {
        Self {
            bytes,
            last_tag: None,
            write_soh_issued: false,
        }
    }
    pub fn into_inner(self) -> BytesMut {
        self.bytes
    }
    pub fn as_slice(&self) -> &[u8] {
        self
    }
    pub fn join(&mut self, other: BytesWrite) {
        self.bytes.unsplit(other.bytes);
        self.last_tag = other.last_tag;
        self.write_soh_issued = other.write_soh_issued;
    }
    pub fn take(self) -> BytesMut {
        self.bytes
    }
    // pub fn reset(&mut self) {
    //     self.bytes.clear();
    //     self.last_tag = None;
    //     self.write_soh_issued = false;
    // }
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
    #[inline]
    fn write_value(&mut self, buf: &[u8]) -> Result<()> {
        self.bytes.put_slice(buf);
        self.write_soh_issued = false;
        Ok(())
    }
    #[inline]
    fn write_tag(&mut self, buf: &'static [u8]) -> Result<()> {
        self.bytes.put_slice(buf);
        self.last_tag = Some(buf);
        Ok(())
    }
    #[inline]
    fn last_written_tag(&self) -> Option<&'static [u8]> {
        self.last_tag
    }

    // will supress double SOH write by design to handle special case for Data when it is serialized outside the parent struct
    #[inline]
    fn write_soh(&mut self) -> Result<()> {
        // fix does not have json like tags {} or [] indicating start and end of nesting but serde generate calls to open & close
        // any nested struct which will result in double SOH charater for such cases.
        // Hence we are shortcircuting this logic by explicity checking if write_soh was the last call and if so we skip it.
        if !self.write_soh_issued {
            self.bytes.put_u8(crate::SOH);
            self.write_soh_issued = true;
        }
        Ok(())
    }
    #[inline]
    fn write_eqs(&mut self) -> Result<()> {
        self.bytes.put_u8(crate::EQS);
        self.write_soh_issued = false;
        Ok(())
    }
}
impl Display for BytesWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use fix_model_core::prelude::FixByteSlice2Display;
        write!(
            f,
            "len: {}, capacity: {}, bytes: \"{}\"",
            self.bytes.len(),
            self.bytes.capacity(),
            (&self.bytes as &[u8]).to_string()
        )
    }
}
impl Debug for BytesWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}
