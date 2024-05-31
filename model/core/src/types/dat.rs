use super::data::Data;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

////////////
#[repr(transparent)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct dat([u8]);
impl dat {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
    #[inline]
    pub fn from_slice(b: &[u8]) -> &Self {
        let ptr: *const [u8] = b as *const [u8];
        let ptr: *const dat = ptr as *const dat;
        unsafe { &*ptr }
    }
}
impl<'a> From<&'a [u8]> for &'a dat {
    fn from(value: &'a [u8]) -> Self {
        dat::from_slice(value)
    }
}
impl AsRef<dat> for &dat {
    #[inline]
    fn as_ref(&self) -> &dat {
        self
    }
}
impl ToOwned for dat {
    type Owned = Data;
    #[inline]
    fn to_owned(&self) -> Self::Owned {
        Data(self.0.to_vec())
    }
}
impl Deref for dat {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Display for &dat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::prelude::FixByteSlice2Display;
        let slice = self.as_slice();
        let display = slice.to_string();
        write!(f, "{}", display)
    }
}
impl Debug for &dat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02X?}", self.as_slice())
    }
}
impl Default for &dat {
    /// Panics but exists to allow auto generated Default for structs that contain ['MyStruct::<&dat>'] to use the following syntax
    /// ```
    /// let dat = fix_model_core::prelude::dat::from_slice(b"hello");
    /// /*
    /// let l = Logon::<&str, &dat> {
    ///     ..Default::default()
    /// };
    /// */
    /// ```
    fn default() -> Self {
        panic!("&dat::default() is not implemented")
    }
}
impl Serialize for &dat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            self.as_slice().encode_base64_string().serialize(serializer)
        } else {
            let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
            use serde::ser::SerializeSeq;
            struct ByteSlice<'a>(&'a [u8]);
            impl<'a> Serialize for ByteSlice<'a> {
                fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                    serializer.serialize_bytes(self.0)
                }
            }
            let bts = ByteSlice(self);
            seq.serialize_element(&bts)?;
            seq.end()
        }
        // serializer.serialize_bytes(self)
    }
}
impl<'de> Deserialize<'de> for &'de dat {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct DataVisitor;
            impl<'de> serde::de::Visitor<'de> for DataVisitor {
                type Value = &'de dat;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a &dat")
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> std::result::Result<Self::Value, E> {
                    Ok(dat::from_slice(v))
                }
            }
            deserializer.deserialize_bytes(DataVisitor)
        } else {
            struct DataVisitor;
            impl<'de> serde::de::Visitor<'de> for DataVisitor {
                type Value = &'de dat;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a &dat")
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(self, v: &'de [u8]) -> std::result::Result<Self::Value, E> {
                    Ok(dat::from_slice(v))
                }
            }
            deserializer.deserialize_seq(DataVisitor)
        }
    }
}

impl<'a> Base64 for &'a dat {
    #[inline]
    fn encode_base64_string(&self) -> String {
        self.as_slice().encode_base64_string()
    }
    #[inline]
    fn encode_base64_vec(&self) -> Vec<u8> {
        self.as_slice().encode_base64_vec()
    }
    #[inline]
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeSliceError> {
        self.as_slice().decode_base64()
    }
}

pub trait Base64 {
    fn encode_base64_string(&self) -> String;
    fn encode_base64_vec(&self) -> Vec<u8>;
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeSliceError>;
}
impl Base64 for &[u8] {
    fn encode_base64_string(&self) -> String {
        unsafe { String::from_utf8_unchecked(Base64::encode_base64_vec(self)) }
    }
    #[allow(clippy::slow_vector_initialization)]
    fn encode_base64_vec(&self) -> Vec<u8> {
        use base64::prelude::*;
        let mut buf_out = Vec::new();
        // make sure we'll have a slice big enough for base64 + padding
        buf_out.resize(self.len() * 4 / 3 + 4, 0);

        let bytes_written = BASE64_STANDARD.encode_slice(self, &mut buf_out).unwrap();
        buf_out.truncate(bytes_written);
        buf_out
    }
    #[allow(clippy::slow_vector_initialization)]
    fn decode_base64(&self) -> Result<Vec<u8>, base64::DecodeSliceError> {
        use base64::prelude::*;
        let mut buf_out = Vec::new();
        buf_out.resize(self.len() * 3 / 4, 0);
        let bytes_written = BASE64_STANDARD.decode_slice(self, &mut buf_out)?;
        buf_out.truncate(bytes_written);
        Ok(buf_out)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use fix_model_test::unittest::setup;
    use log::info;
    #[test]
    fn test_base64() {
        setup::log::configure();
        let inp_raw = b"hello world".as_ref();
        info!("inp_raw: {:?}", inp_raw);
        let base64_string = inp_raw.encode_base64_string();
        info!("base64_string: {}", base64_string);
        assert_eq!(base64_string, "aGVsbG8gd29ybGQ=");

        let out_raw = base64_string.as_bytes().decode_base64().unwrap();
        info!("out_raw: {:?}", out_raw);
        assert_eq!(inp_raw, out_raw);
    }
}
