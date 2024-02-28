use super::dat::dat;
use crate::prelude::Base64;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, ops::Deref};

#[derive(Clone, PartialEq)]
pub struct Data(pub(crate) Vec<u8>);
impl Data {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
    #[inline]
    pub fn as_dat(&self) -> &dat {
        self.deref()
    }
    #[inline]
    pub fn from_slice(value: &[u8]) -> Self {
        value.to_vec().into()
    }
    #[inline]
    pub fn from_vec(value: Vec<u8>) -> Self {
        value.into()
    }
}
impl Default for Data {
    /// Panics but exists to allow auto generated Default for structs that contain [`MyStruct::<Data>`] to use the following syntax
    /// ```no_run
    /// /*
    /// let l = Logon::<&str, Data> {
    ///     ..Default::default()
    /// };
    /// */
    /// ```
    fn default() -> Self {
        panic!("Data::default() is not implemented")
    }
}
impl From<Vec<u8>> for Data {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
impl From<&[u8]> for Data {
    fn from(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}
impl From<&str> for Data {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}
impl std::fmt::Display for Data {
    /// Display as base64 string
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_dat().to_string())
        // write!(f, "{}", self.as_dat().as_slice().to_base64_string())
    }
}
impl std::fmt::Debug for Data {
    /// Debug as hex
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_dat())
    }
}
impl Deref for Data {
    type Target = dat;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &Borrow::<dat>::borrow(self)
    }
}
impl AsRef<dat> for Data {
    #[inline]
    fn as_ref(&self) -> &dat {
        self.deref()
    }
}
impl Borrow<dat> for Data {
    #[inline]
    fn borrow(&self) -> &dat {
        let ptr = &*self.0 as *const [u8] as *const dat;
        unsafe { &*ptr }
    }
}
impl Serialize for Data {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.as_dat().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Data {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            struct DataVisitor;
            impl<'de> serde::de::Visitor<'de> for DataVisitor {
                type Value = Data;
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("a Data")
                }
                fn visit_borrowed_bytes<E: serde::de::Error>(self, base64_str: &[u8]) -> std::result::Result<Self::Value, E> {
                    base64_str.from_base64().map(|v| Data(v)).map_err(serde::de::Error::custom)
                }
            }
            deserializer.deserialize_bytes(DataVisitor)
        } else {
            Ok(<&dat>::deserialize(deserializer)?.to_owned())
        }
    }
}
