use crate::prelude::{dat, Base64, Error};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

#[derive(Debug)]
enum MaybeAllocated<'a> {
    Allocated(Vec<u8>),
    Borrowed { slice: &'a [u8], base64: bool },
}
#[repr(transparent)]
#[allow(non_camel_case_types)]
// #[derive()] // TODO add decoder type to generic with default base64
pub struct dat_codec<'a>(MaybeAllocated<'a>);
impl<'a> dat_codec<'a> {
    #[inline]
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self(MaybeAllocated::Borrowed { slice, base64: false })
    }
    pub fn decode(&mut self) -> Result<(), Error> {
        match &self.0 {
            MaybeAllocated::Borrowed { slice, base64 } if *base64 == true => {
                let v = slice.from_base64().map_err(|e| Error::NotBase64String(e.to_string()))?;
                self.0 = MaybeAllocated::Allocated(v);
                Ok(())
            }
            _ => Ok(()),
        }
    }
    #[inline]
    pub fn as_dat(&self) -> &dat {
        let ptr = self.as_slice() as *const [u8] as *const dat;
        unsafe { &*ptr }
    }

    #[inline]
    fn as_slice(&self) -> &[u8] {
        match &self.0 {
            MaybeAllocated::Allocated(v) => v.as_slice(),
            MaybeAllocated::Borrowed { slice, .. } => slice,
        }
    }
}
impl Deref for dat_codec<'_> {
    type Target = dat;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.as_dat()
    }
}
impl PartialEq for dat_codec<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl<'a> From<&'a [u8]> for dat_codec<'a> {
    fn from(value: &'a [u8]) -> Self {
        dat_codec::from_slice(value)
    }
}
impl Display for dat_codec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_dat())
    }
}
impl Debug for dat_codec<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_dat())
    }
}
impl Default for dat_codec<'_> {
    /// Panics but exists to allow auto generated Default for structs that contain [`MyStruct::<dat_codec>`] to use the following syntax
    /// ```no_run
    /// /*
    /// let l = Logon::<&str, dat_codec> {
    ///     ..Default::default()
    /// };
    /// */
    /// ```
    fn default() -> Self {
        panic!("dat_codec::default() is not implemented")
    }
}
impl Serialize for dat_codec<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.as_slice().to_base64_string().serialize(serializer)
        } else {
            let mut seq = serializer.serialize_seq(Some(self.as_slice().len()))?;
            use serde::ser::SerializeSeq;
            struct ByteSlice<'a>(&'a [u8]);
            impl<'a> Serialize for ByteSlice<'a> {
                fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                    serializer.serialize_bytes(self.0)
                }
            }
            let bts = ByteSlice(self.as_slice());
            seq.serialize_element(&bts)?;
            seq.end()
        }
    }
}
impl<'de> Deserialize<'de> for dat_codec<'de> {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct DataVisitor;
            impl<'de> serde::de::Visitor<'de> for DataVisitor {
                type Value = dat_codec<'de>;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a dat_lazybase64")
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(self, slice: &'de [u8]) -> std::result::Result<Self::Value, E> {
                    Ok(dat_codec(MaybeAllocated::Borrowed { slice, base64: true }))
                }
            }
            deserializer.deserialize_bytes(DataVisitor)
        } else {
            struct DataVisitor;
            impl<'de> serde::de::Visitor<'de> for DataVisitor {
                type Value = dat_codec<'de>;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a dat_lazybase64")
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> std::result::Result<Self::Value, E> {
                    Ok(dat_codec::from_slice(v))
                }
            }
            deserializer.deserialize_seq(DataVisitor)
        }
    }
}
